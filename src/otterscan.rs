#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationType {
    Transfer = 0,
    SelfDestruct = 1,
    Create = 2,
    Create2 = 3,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InternalOperation {
    #[serde(rename = "type")]
    pub op_type: OperationType,
    pub from: Address,
    pub to: Address,
    pub value: U256,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceiptWithTimestamp {
    #[serde(flatten)]
    pub base: TransactionReceipt,
    pub timestamp: U64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsWithReceipts {
    pub txs: Vec<Transaction>,
    pub receipts: Vec<ReceiptWithTimestamp>,
    pub first_page: bool,
    pub last_page: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Issuance {
    pub block_reward: U256,
    pub uncle_reward: U256,
    pub issuance: U256,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockData {
    #[serde(flatten)]
    pub inner: Block,
    pub transaction_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockDetails {
    pub block: BlockData,
    pub issuance: Issuance,
    pub total_fees: U256,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockTransactions {
    pub fullblock: Block,
    pub receipts: Vec<TransactionReceipt>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TraceOperation {
    Call,
    StaticCall,
    DelegateCall,
    CallCode,
    Create,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceEntry {
    #[serde(rename = "type")]
    pub op_type: TraceOperation,
    pub depth: u16,
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub input: Bytes,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractCreatorData {
    pub tx: H256,
    pub creator: Address,
}

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "ots"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "ots"))]
pub trait OtterscanApi {
    #[method(name = "getApiLevel")]
    async fn get_api_level(&self) -> RpcResult<u8>;
    #[method(name = "getInternalOperations")]
    async fn get_internal_operations(&self, hash: H256) -> RpcResult<Vec<InternalOperation>>;

    /// Search transactions that touch a certain address.
    ///
    /// It searches back a certain block (excluding); the results are sorted descending.
    ///
    /// The `page_size` indicates how many txs may be returned. If there are less txs than `page_size`,
    /// they are just returned. But it may return a little more than pageSize if there are more txs
    /// than the necessary to fill `page_size` in the last found block, i.e., let's say you want `page_size` == 25,
    /// you already found 24 txs, the next block contains 4 matches, then this function will return 28 txs.
    #[method(name = "searchTransactionsBefore")]
    async fn search_transactions_before(
        &self,
        addr: Address,
        block_num: u64,
        page_size: usize,
    ) -> RpcResult<TransactionsWithReceipts>;

    /// Search transactions that touch a certain address.
    ///
    /// It searches forward a certain block (excluding); the results are sorted descending.
    ///
    /// The `page_size` indicates how many txs may be returned. If there are less txs than pageSize,
    /// they are just returned. But it may return a little more than `page_size` if there are more txs
    /// than the necessary to fill `page_size` in the last found block, i.e., let's say you want `page_size` == 25,
    /// you already found 24 txs, the next block contains 4 matches, then this function will return 28 txs.
    #[method(name = "searchTransactionsAfter")]
    async fn search_transactions_after(
        &self,
        addr: Address,
        block_num: u64,
        page_size: usize,
    ) -> RpcResult<TransactionsWithReceipts>;
    #[method(name = "getBlockDetails")]
    async fn get_block_details(&self, number: u64) -> RpcResult<Option<BlockDetails>>;
    #[method(name = "getBlockDetailsByHash")]
    async fn get_block_details_by_hash(&self, hash: H256) -> RpcResult<Option<BlockDetails>>;
    #[method(name = "getBlockTransactions")]
    async fn get_block_transactions(
        &self,
        number: u64,
        page_number: usize,
        page_size: usize,
    ) -> RpcResult<Option<BlockTransactions>>;
    #[method(name = "hasCode")]
    async fn has_code(&self, address: Address, block_id: BlockId) -> RpcResult<bool>;
    #[method(name = "traceTransaction")]
    async fn trace_transaction(&self, hash: H256) -> RpcResult<Vec<TraceEntry>>;
    #[method(name = "getTransactionError")]
    async fn get_transaction_error(&self, hash: H256) -> RpcResult<Bytes>;
    #[method(name = "getTransactionBySenderAndNonce")]
    async fn get_transaction_by_sender_and_nonce(
        &self,
        addr: Address,
        nonce: u64,
    ) -> RpcResult<Option<H256>>;
    #[method(name = "getContractCreator")]
    async fn get_contract_creator(&self, addr: Address) -> RpcResult<Option<ContractCreatorData>>;
}
