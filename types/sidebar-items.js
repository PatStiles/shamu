window.SIDEBAR_ITEMS = {"enum":[["BlockErrorKind",""],["BlockRemoverErrorKind",""],["CollectionErrorType",""],["DigestError",""],["HeaderBuilderError","Error type for HeaderBuilder"],["PrimaryMessage",""],["PrimaryWorkerMessage","The messages sent by the primary to its workers."],["ReconfigureNotification","Message to reconfigure worker tasks. This message must be sent by a trusted source."],["RetrievalResult",""],["WorkerMessage","The message exchanged between workers."],["WorkerPrimaryError",""],["WorkerPrimaryMessage","The messages sent by the workers to their primary."]],"fn":[["serialized_batch_digest","Hashes a serialized batch message without deserializing it into a batch."]],"macro":[["bail",""],["ensure",""]],"mod":[["error",""],["metered_channel",""]],"struct":[["Batch",""],["BatchDigest",""],["BatchMessage",""],["BincodeEncodedPayload","A bincode encoded payload. This is intended to be used in the short-term while we don’t have good protobuf definitions for Narwhal types"],["BlockError",""],["BlockRemoverError",""],["Certificate",""],["CertificateDigest",""],["CertificateDigestProto",""],["ClientBatchRequest",""],["Collection",""],["CollectionError",""],["CollectionRetrievalResult",""],["ConfigurationClient",""],["ConfigurationServer",""],["ConsensusStore","The persistent storage of the sequencer."],["Empty","Empty message for when we don’t have anything to return"],["GetCollectionsRequest",""],["GetCollectionsResponse",""],["GetPrimaryAddressResponse",""],["Header",""],["HeaderBuilder","Builder for `Header`."],["HeaderDigest",""],["MultiAddrProto",""],["NewEpochRequest",""],["NewNetworkInfoRequest",""],["NodeReadCausalRequest",""],["NodeReadCausalResponse",""],["PrimaryAddressesProto",""],["PrimaryToPrimaryClient","The primary-to-primary interface"],["PrimaryToPrimaryServer","The primary-to-primary interface"],["PrimaryToWorkerClient","The primary-to-worker interface"],["PrimaryToWorkerServer","The primary-to-worker interface"],["ProposerClient",""],["ProposerServer",""],["PublicKeyProto",""],["ReadCausalRequest",""],["ReadCausalResponse",""],["RemoveCollectionsRequest",""],["RoundsRequest",""],["RoundsResponse",""],["TransactionProto",""],["TransactionsClient",""],["TransactionsServer",""],["ValidatorClient","The consensus to mempool interface for validator actions."],["ValidatorData",""],["ValidatorServer","The consensus to mempool interface for validator actions."],["Vote",""],["VoteDigest",""],["WorkerInfoResponse",""],["WorkerToPrimaryClient","The worker-to-primary interface"],["WorkerToPrimaryServer","The worker-to-primary interface"],["WorkerToWorkerClient","The worker-to-worker interface"],["WorkerToWorkerServer","The worker-to-worker interface"]],"trait":[["Configuration","Generated trait containing gRPC methods that should be implemented for use with ConfigurationServer."],["PrimaryToPrimary","Generated trait containing gRPC methods that should be implemented for use with PrimaryToPrimaryServer."],["PrimaryToWorker","Generated trait containing gRPC methods that should be implemented for use with PrimaryToWorkerServer."],["Proposer","Generated trait containing gRPC methods that should be implemented for use with ProposerServer."],["Transactions","Generated trait containing gRPC methods that should be implemented for use with TransactionsServer."],["Validator","Generated trait containing gRPC methods that should be implemented for use with ValidatorServer."],["WorkerToPrimary","Generated trait containing gRPC methods that should be implemented for use with WorkerToPrimaryServer."],["WorkerToWorker","Generated trait containing gRPC methods that should be implemented for use with WorkerToWorkerServer."]],"type":[["BlockRemoverResult",""],["BlockResult",""],["Round","The round number."],["SequenceNumber","A global sequence number assigned to every certificate."],["SerializedBatchMessage","Indicates a serialized `WorkerMessage::Batch` message."],["ShutdownToken","Shutdown token dropped when a task is properly shut down."],["StoreResult","Convenience type to propagate store errors."],["Transaction",""]]};