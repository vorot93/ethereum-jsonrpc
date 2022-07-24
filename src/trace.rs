#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceFilterMode {
    Union,
    Intersection,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub from_block: Option<BlockId>,
    pub to_block: Option<BlockId>,
    pub from_address: Option<HashSet<Address>>,
    pub to_address: Option<HashSet<Address>>,
    pub after: Option<usize>,
    pub count: Option<usize>,
    pub mode: Option<TraceFilterMode>,
}

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "trace"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "trace"))]
pub trait TraceApi {
    #[method(name = "call")]
    async fn call(
        &self,
        call: MessageCall,
        trace_types: HashSet<TraceType>,
        block_id: Option<BlockId>,
    ) -> RpcResult<FullTrace>;
    #[method(name = "callMany")]
    async fn call_many(
        &self,
        calls: Vec<(MessageCall, HashSet<TraceType>)>,
        block_id: Option<BlockId>,
    ) -> RpcResult<Vec<FullTrace>>;
    #[method(name = "rawTransaction")]
    async fn raw_transaction(
        &self,
        rlp: Bytes,
        trace_types: HashSet<TraceType>,
        block_id: Option<BlockId>,
    ) -> RpcResult<FullTrace>;
    #[method(name = "replayBlockTransactions")]
    async fn replay_block_transactions(
        &self,
        block_id: BlockId,
        trace_types: HashSet<TraceType>,
    ) -> RpcResult<Option<Vec<FullTraceWithTransactionHash>>>;
    #[method(name = "replayTransaction")]
    async fn replay_transaction(
        &self,
        hash: H256,
        trace_types: HashSet<TraceType>,
    ) -> RpcResult<FullTrace>;
    #[method(name = "block")]
    async fn block(
        &self,
        block_id: BlockId,
    ) -> RpcResult<Option<Vec<TransactionTraceWithLocation>>>;
    #[method(name = "filter")]
    async fn filter(&self, filter: Filter) -> RpcResult<Vec<TransactionTraceWithLocation>>;
}
