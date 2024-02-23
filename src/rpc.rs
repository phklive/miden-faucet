use miden_node_proto::generated::{
    requests::{
        CheckNullifiersRequest, GetBlockHeaderByNumberRequest, SubmitProvenTransactionRequest,
        SyncStateRequest,
    },
    responses::{
        CheckNullifiersResponse, GetBlockHeaderByNumberResponse, SubmitProvenTransactionResponse,
        SyncStateResponse,
    },
    rpc::{
        api_client::ApiClient,
        api_server::{self},
    },
};
use miden_objects::Digest;
use tonic::{transport::Channel, Request, Response, Status};

#[derive(Clone, Debug)]
pub struct RpcApi {
    pub rpc: ApiClient<Channel>,
}

#[tonic::async_trait]
impl api_server::Api for RpcApi {
    async fn sync_state(
        &self,
        request: Request<SyncStateRequest>,
    ) -> Result<Response<SyncStateResponse>, Status> {
        self.clone().rpc.sync_state(request).await
    }

    async fn submit_proven_transaction(
        &self,
        request: Request<SubmitProvenTransactionRequest>,
    ) -> Result<Response<SubmitProvenTransactionResponse>, Status> {
        self.submit_proven_transaction(request).await
    }

    async fn get_block_header_by_number(
        &self,
        request: Request<GetBlockHeaderByNumberRequest>,
    ) -> Result<Response<GetBlockHeaderByNumberResponse>, Status> {
        self.clone().rpc.get_block_header_by_number(request).await
    }

    async fn check_nullifiers(
        &self,
        request: Request<CheckNullifiersRequest>,
    ) -> Result<Response<CheckNullifiersResponse>, Status> {
        // validate all the nullifiers from the user request
        for nullifier in request.get_ref().nullifiers.iter() {
            let _: Digest = nullifier.try_into().or(Err(Status::invalid_argument(
                "Digest field is not in the modulus range",
            )))?;
        }

        self.check_nullifiers(request).await
    }
}
