// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
use crate::{header_waiter::WaiterMessage, primary::PayloadToken, BlockCommand};
use config::{Committee, WorkerId};
use consensus::dag::Dag;
use crypto::PublicKey;
use fastcrypto::Hash as _;
use std::{collections::HashMap, sync::Arc};
use storage::CertificateStore;
use store::Store;
use tracing::log::{trace, warn};
use types::{
    error::DagResult, metered_channel::Sender, BatchDigest, Certificate, CertificateDigest, Header,
    Round,
};

/// The number of missing rounds when a catch up process using range synchronization should be started.
const MISSING_ROUNDS_CATCH_UP_THRESHOLD: Round = 2;

#[cfg(test)]
#[path = "tests/synchronizer_tests.rs"]
pub mod synchronizer_tests;

/// The `Synchronizer` checks if we have all batches and parents referenced by a header. If we don't, it sends
/// a command to the `Waiter` to request the missing data.
pub struct Synchronizer {
    /// The public key of this primary.
    name: PublicKey,
    /// The persistent storage.
    certificate_store: CertificateStore,
    payload_store: Store<(BatchDigest, WorkerId), PayloadToken>,
    /// Send commands to the `HeaderWaiter`.
    tx_header_waiter: Sender<WaiterMessage>,
    /// Send commands to the `CertificateWaiter`.
    tx_certificate_waiter: Sender<Certificate>,
    /// Send commands to the `BlockWaiter`.
    tx_get_block_commands: Sender<BlockCommand>,
    /// The genesis and its digests.
    genesis: Vec<(CertificateDigest, Certificate)>,
    /// The dag used for the external consensus
    dag: Option<Arc<Dag>>,
}

impl Synchronizer {
    pub fn new(
        name: PublicKey,
        committee: &Committee,
        certificate_store: CertificateStore,
        payload_store: Store<(BatchDigest, WorkerId), PayloadToken>,
        tx_header_waiter: Sender<WaiterMessage>,
        tx_certificate_waiter: Sender<Certificate>,
        tx_get_block_commands: Sender<BlockCommand>,
        dag: Option<Arc<Dag>>,
    ) -> Self {
        let mut synchronizer = Self {
            name,
            certificate_store,
            payload_store,
            tx_header_waiter,
            tx_certificate_waiter,
            tx_get_block_commands,
            genesis: Vec::default(),
            dag,
        };
        synchronizer.update_genesis(committee);
        synchronizer
    }

    /// Update the genesis (called upon reconfiguration).
    pub fn update_genesis(&mut self, committee: &Committee) {
        self.genesis = Certificate::genesis(committee)
            .into_iter()
            .map(|x| (x.digest(), x))
            .collect();
    }

    /// Returns `true` if we have all transactions of the payload. If we don't, we return false,
    /// synchronize with other nodes (through our workers), and re-schedule processing of the
    /// header for when we will have its complete payload.
    pub async fn missing_payload(&mut self, header: &Header) -> DagResult<bool> {
        // We don't store the payload of our own workers.
        if header.author == self.name {
            return Ok(false);
        }

        let mut missing = HashMap::new();
        for (digest, worker_id) in header.payload.iter() {
            // Check whether we have the batch. If one of our worker has the batch, the primary stores the pair
            // (digest, worker_id) in its own storage. It is important to verify that we received the batch
            // from the correct worker id to prevent the following attack:
            //      1. A Bad node sends a batch X to 2f good nodes through their worker #0.
            //      2. The bad node proposes a malformed block containing the batch X and claiming it comes
            //         from worker #1.
            //      3. The 2f good nodes do not need to sync and thus don't notice that the header is malformed.
            //         The bad node together with the 2f good nodes thus certify a block containing the batch X.
            //      4. The last good node will never be able to sync as it will keep sending its sync requests
            //         to workers #1 (rather than workers #0). Also, clients will never be able to retrieve batch
            //         X as they will be querying worker #1.
            if self
                .payload_store
                .read((*digest, *worker_id))
                .await?
                .is_none()
            {
                missing.insert(*digest, *worker_id);
            }
        }

        if missing.is_empty() {
            return Ok(false);
        }

        self.tx_header_waiter
            .send(WaiterMessage::SyncBatches(missing, header.clone()))
            .await
            .expect("Failed to send sync batch request");
        Ok(true)
    }

    /// Returns the parents of a header if we have them all. If at least one parent is missing,
    /// we return an empty vector, synchronize with other nodes, and re-schedule processing
    /// of the header for when we will have all the parents.
    pub async fn get_parents(&mut self, header: &Header) -> DagResult<Vec<Certificate>> {
        let mut missing = Vec::new();
        let mut parents = Vec::new();
        for digest in &header.parents {
            if let Some(genesis) = self
                .genesis
                .iter()
                .find(|(x, _)| x == digest)
                .map(|(_, x)| x)
            {
                parents.push(genesis.clone());
                continue;
            }

            match self.certificate_store.read(*digest)? {
                Some(certificate) => parents.push(certificate),
                None => missing.push(*digest),
            };
        }

        if missing.is_empty() {
            return Ok(parents);
        }

        self.tx_header_waiter
            .send(WaiterMessage::SyncParents(missing, header.clone()))
            .await
            .expect("Failed to send sync parents request");
        Ok(Vec::new())
    }

    /// Check whether we have seen all the ancestors of the certificate. If we don't, send the
    /// certificate to the `CertificateWaiter` which will trigger re-processing once we have
    /// all the missing data.
    pub async fn deliver_certificate(&mut self, certificate: &Certificate) -> DagResult<bool> {
        for digest in &certificate.header.parents {
            if self.genesis.iter().any(|(x, _)| x == digest) {
                continue;
            }

            if !self.has_processed_certificate(*digest).await? {
                self.tx_certificate_waiter
                    .send(certificate.clone())
                    .await
                    .expect("Failed to send sync certificate request");
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Checks if this primary is missing > 2 rounds of certificates and needs to start a range sync to catch up.
    /// Returns whether a catch up process is started. Does not block on finishing the catch up process.
    pub async fn maybe_catch_up(&self, cert_round: Round) -> bool {
        let latest_round = self.certificate_store.last_round_number().unwrap_or(0);
        let round_lag = cert_round.saturating_sub(latest_round);
        if round_lag <= MISSING_ROUNDS_CATCH_UP_THRESHOLD {
            // No catch up is needed.
            return false;
        }
        let cmd = BlockCommand::GetRange {};
        self.tx_get_block_commands
            .send(cmd)
            .await
            .expect("Failed to send get range request");
        let msg = "Missing rounds detected: received certificate at {cert_round}, local latest at {latest_round}. Starting to catch up ...";
        if round_lag >= 20 {
            warn!("{msg}");
        } else {
            trace!("{msg}");
        }
        true
    }

    /// This method answers to the question of whether the certificate with the
    /// provided digest has ever been successfully processed (seen) by this
    /// node. Depending on the mode of running the node (internal Vs external
    /// consensus) either the dag will be used to confirm that or the
    /// certificate_store.
    async fn has_processed_certificate(&self, digest: CertificateDigest) -> DagResult<bool> {
        if let Some(dag) = &self.dag {
            return Ok(dag.has_ever_contained(digest).await);
        }
        Ok(self.certificate_store.read(digest)?.is_some())
    }
}
