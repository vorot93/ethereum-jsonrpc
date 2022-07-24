#[allow(unused_imports)]
use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockFilter {
    #[serde(rename_all = "camelCase")]
    Exact { block_hash: H256 },
    #[serde(rename_all = "camelCase")]
    Bounded {
        from_block: Option<BlockNumber>,
        to_block: Option<BlockNumber>,
    },
}

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogAddressFilter(
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")] pub Vec<Address>,
);

#[serde_as]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogTopicFilter(#[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")] pub Vec<H256>);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogFilter {
    #[serde(flatten)]
    pub block_filter: Option<BlockFilter>,
    pub address: Option<LogAddressFilter>,
    pub topics: Option<ArrayVec<Option<LogTopicFilter>, 4>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyncStatus {
    NotSyncing,
    Syncing {
        highest_block: BlockNumber,
        current_block: BlockNumber,
    },
}

#[derive(Serialize, Deserialize)]
struct Syncing {
    highest_block: BlockNumber,
    current_block: BlockNumber,
}

impl Serialize for SyncStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            SyncStatus::NotSyncing => false.serialize(serializer),
            SyncStatus::Syncing {
                highest_block,
                current_block,
            } => Syncing {
                highest_block,
                current_block,
            }
            .serialize(serializer),
        }
    }
}

struct SyncStatusVisitor;

impl<'de> Visitor<'de> for SyncStatusVisitor {
    type Value = SyncStatus;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "false or a struct describing current sync status"
        )
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if !v {
            Ok(Self::Value::NotSyncing)
        } else {
            Err(de::Error::invalid_type(de::Unexpected::Bool(v), &self))
        }
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        Syncing::deserialize(MapAccessDeserializer::new(map)).map(
            |Syncing {
                 highest_block,
                 current_block,
             }| SyncStatus::Syncing {
                highest_block,
                current_block,
            },
        )
    }
}

impl<'de> Deserialize<'de> for SyncStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SyncStatusVisitor)
    }
}

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "eth"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "eth"))]
pub trait EthApi {
    #[method(name = "blockNumber")]
    async fn block_number(&self) -> RpcResult<U64>;
    #[method(name = "chainId")]
    async fn chain_id(&self) -> RpcResult<U64>;
    #[method(name = "call")]
    async fn call(&self, call_data: MessageCall, block_number: BlockNumber) -> RpcResult<Bytes>;
    #[method(name = "estimateGas")]
    async fn estimate_gas(
        &self,
        call_data: MessageCall,
        block_number: BlockNumber,
    ) -> RpcResult<U64>;
    #[method(name = "gasPrice")]
    async fn gas_price(&self) -> RpcResult<U256>;
    #[method(name = "maxPriorityFeePerGas")]
    async fn max_priority_fee_per_gas(&self) -> RpcResult<U256>;
    #[method(name = "getBalance")]
    async fn get_balance(&self, address: Address, block_number: BlockNumber) -> RpcResult<U256>;
    #[method(name = "getBlockByHash")]
    async fn get_block_by_hash(
        &self,
        block_hash: H256,
        full_tx_obj: bool,
    ) -> RpcResult<Option<Block>>;
    #[method(name = "getBlockByNumber")]
    async fn get_block_by_number(
        &self,
        block_number: BlockNumber,
        full_tx_obj: bool,
    ) -> RpcResult<Option<Block>>;
    #[method(name = "getBlockTransactionCountByHash")]
    async fn get_block_transaction_count_by_hash(&self, block_hash: H256) -> RpcResult<U64>;
    #[method(name = "getBlockTransactionCountByNumber")]
    async fn get_block_transaction_count_by_number(
        &self,
        block_number: BlockNumber,
    ) -> RpcResult<U64>;
    #[method(name = "getCode")]
    async fn get_code(&self, address: Address, block_number: BlockNumber) -> RpcResult<Bytes>;
    #[method(name = "getStorageAt")]
    async fn get_storage_at(
        &self,
        address: Address,
        storage_pos: U256,
        block_number: BlockNumber,
    ) -> RpcResult<U256>; // Storage data is nothing more than 32-bytes
    #[method(name = "getTransactionByHash")]
    async fn get_transaction_by_hash(&self, hash: H256) -> RpcResult<Option<Tx>>;
    #[method(name = "getTransactionByBlockHashAndIndex")]
    async fn get_transaction_by_block_hash_and_index(
        &self,
        block_hash: H256,
        index: U64,
    ) -> RpcResult<Option<Tx>>;
    #[method(name = "getTransactionByBlockNumberAndIndex")]
    async fn get_transaction_by_block_number_and_index(
        &self,
        block_number: BlockNumber,
        index: U64,
    ) -> RpcResult<Option<Tx>>;
    #[method(name = "getTransactionCount")]
    async fn get_transaction_count(
        &self,
        address: Address,
        block_number: BlockNumber,
    ) -> RpcResult<U64>;
    #[method(name = "getTransactionReceipt")]
    async fn get_transaction_receipt(&self, tx_hash: H256)
        -> RpcResult<Option<TransactionReceipt>>;
    #[method(name = "getUncleByBlockHashAndIndex")]
    async fn get_uncle_by_block_hash_and_index(
        &self,
        block_hash: H256,
        index: U64,
    ) -> RpcResult<Option<Block>>;
    #[method(name = "getUncleByBlockNumberAndIndex")]
    async fn get_uncle_by_block_number_and_index(
        &self,
        block_number: BlockNumber,
        index: U64,
    ) -> RpcResult<Option<Block>>;
    #[method(name = "getUncleCountByBlockHash")]
    async fn get_uncle_count_by_block_hash(&self, block_hash: H256) -> RpcResult<U64>;
    #[method(name = "getUncleCountByBlockNumber")]
    async fn get_uncle_count_by_block_number(&self, block_number: BlockNumber) -> RpcResult<U64>;
    #[method(name = "getLogs")]
    async fn get_logs(&self, filter: LogFilter) -> RpcResult<Vec<TransactionLog>>;
    #[method(name = "syncing")]
    async fn syncing(&self) -> RpcResult<SyncStatus>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use serde_json::json;

    #[test]
    fn log_filter_serialize() {
        let encoded = json!({
            "blockHash": "0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3",
            "address": "0xdeadbeef00000000000000000000000000000000",
            "topics": [
                null,
                "0xaa00000000000000000000000000000000000000000000000000000000000000",
                [
                    "0xbb00000000000000000000000000000000000000000000000000000000000000",
                    "0xcc00000000000000000000000000000000000000000000000000000000000000",
                ],
            ],
        });
        let v = LogFilter {
            block_filter: Some(BlockFilter::Exact {
                block_hash: hex!(
                    "d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3"
                )
                .into(),
            }),
            address: Some(LogAddressFilter(vec![hex!(
                "deadbeef00000000000000000000000000000000"
            )
            .into()])),
            topics: Some({
                let mut arr = ArrayVec::new();
                arr.push(None);
                arr.push(Some(LogTopicFilter(vec![hex!(
                    "aa00000000000000000000000000000000000000000000000000000000000000"
                )
                .into()])));
                arr.push(Some(LogTopicFilter(vec![
                    hex!("bb00000000000000000000000000000000000000000000000000000000000000").into(),
                    hex!("cc00000000000000000000000000000000000000000000000000000000000000").into(),
                ])));
                arr
            }),
        };

        assert_eq!(serde_json::from_value::<LogFilter>(encoded).unwrap(), v);
    }
}
