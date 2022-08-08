use crate::prelude::*;
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// Macro used by MessageCall types (LegacyType, EIP2930Type, EIP1155Type)
///
/// It implements `Display` and `FromStr` to convert to/from the market to the string of its type.
///
/// The MessageCallTypes must implement the associated constant `TYPE`.
macro_rules! impl_display_and_from_str_for_type {
    ($ty:ty) => {
        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", <$ty>::TYPE)
            }
        }

        impl std::str::FromStr for $ty {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s == <$ty>::TYPE {
                    Ok(<$ty>::default())
                } else {
                    Err(format!("Invalid type {}, expected {}", s, <$ty>::TYPE))
                }
            }
        }
    };
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default, DeserializeFromStr, SerializeDisplay)]
pub struct LegacyType;
impl LegacyType {
    const TYPE: &'static str = "0x00";
}
impl_display_and_from_str_for_type!(LegacyType);

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default, DeserializeFromStr, SerializeDisplay)]
pub struct EIP2930Type;
impl EIP2930Type {
    const TYPE: &'static str = "0x01";
}
impl_display_and_from_str_for_type!(EIP2930Type);

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default, DeserializeFromStr, SerializeDisplay)]
pub struct EIP1559Type;
impl EIP1559Type {
    const TYPE: &'static str = "0x02";
}
impl_display_and_from_str_for_type!(EIP1559Type);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessListEntry {
    pub address: Address,
    pub storage_keys: Vec<H256>,
}

#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub enum MessageCall {
    #[serde(rename_all = "camelCase")]
    Legacy {
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        tag: Option<LegacyType>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gas: Option<U64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gas_price: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<Bytes>,
    },
    #[serde(rename_all = "camelCase")]
    EIP2930 {
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        tag: Option<EIP2930Type>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gas: Option<U64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gas_price: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<Bytes>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        access_list: Option<Vec<AccessListEntry>>,
    },
    #[serde(rename_all = "camelCase")]
    EIP1559 {
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        tag: Option<EIP1559Type>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        from: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        gas: Option<U64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_fee_per_gas: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_priority_fee_per_gas: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        value: Option<U256>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        data: Option<Bytes>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        access_list: Option<Vec<AccessListEntry>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum TransactionMessage {
    #[serde(rename = "0x0")]
    #[serde(rename_all = "camelCase")]
    Legacy {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        chain_id: Option<U64>,
        nonce: U64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        gas: U64,
        gas_price: U256,
        value: U256,
        input: Bytes,
    },
    #[serde(rename = "0x1")]
    #[serde(rename_all = "camelCase")]
    EIP2930 {
        chain_id: U64,
        nonce: U64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        gas: U64,
        gas_price: U256,
        value: U256,
        input: Bytes,
        access_list: Vec<AccessListEntry>,
    },
    #[serde(rename = "0x2")]
    #[serde(rename_all = "camelCase")]
    EIP1559 {
        chain_id: U64,
        nonce: U64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        to: Option<Address>,
        gas: U64,
        max_fee_per_gas: U256,
        max_priority_fee_per_gas: U256,
        value: U256,
        input: Bytes,
        access_list: Vec<AccessListEntry>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(flatten)]
    pub message: TransactionMessage,
    /// RLP encoded representation of the transaction.
    pub v: U64,
    pub r: H256,
    pub s: H256,

    pub from: Address,
    pub hash: H256,
    pub transaction_index: Option<U64>,
    pub block_number: Option<U64>,
    pub block_hash: Option<H256>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// Tx is either a transaction or a transaction hash.
pub enum Tx {
    /// Transaction.
    Transaction(Box<Transaction>),
    /// Transaction hash.
    Hash(H256),
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use serde_json::json;

    #[test]
    fn test_ser_de_hexbytes_option() {
        let call_data = MessageCall::Legacy {
            tag: None,
            from: None,
            to: Some(Address::from([0; 20])),
            gas: None,
            gas_price: None,
            value: None,
            data: None,
        };
        let hexstring = json!({
            "to":"0x0000000000000000000000000000000000000000",
        });
        assert_eq!(serde_json::to_value(&call_data).unwrap(), hexstring);
        assert_eq!(
            serde_json::from_value::<MessageCall>(hexstring).unwrap(),
            call_data
        );

        let call_data_with_data = MessageCall::Legacy {
            tag: None,
            from: None,
            to: Some(Address::from([0; 20])),
            gas: None,
            gas_price: None,
            value: None,
            data: Some(Bytes::from(&b"Hello Akula"[..])),
        };

        let hexstring_with_data = json!({
            "to":"0x0000000000000000000000000000000000000000",
            "data":"0x48656c6c6f20416b756c61",
        });
        assert_eq!(
            serde_json::to_value(&call_data_with_data).unwrap(),
            hexstring_with_data
        );
        assert_eq!(
            serde_json::from_value::<MessageCall>(hexstring_with_data).unwrap(),
            call_data_with_data
        );
    }

    #[test]
    fn test_deserialize_with_tag() {
        let call_data = MessageCall::Legacy {
            tag: Some(LegacyType),
            from: None,
            to: Some(Address::from([0; 20])),
            gas: None,
            gas_price: None,
            value: None,
            data: None,
        };

        let hexstring = json!({
            "type": "0x00",
            "to":"0x0000000000000000000000000000000000000000",
        });

        assert_eq!(
            serde_json::from_value::<MessageCall>(hexstring).unwrap(),
            call_data,
        );
    }

    #[test]
    fn test_deserialize_with_tag_fails() {
        let hexstring = json!({
            "type": "0xdeadbeef",
            "to":"0x0000000000000000000000000000000000000000",
        });

        assert_eq!(
            &serde_json::from_value::<MessageCall>(hexstring)
                .err()
                .unwrap()
                .to_string(),
            "data did not match any variant of untagged enum MessageCall"
        );
    }

    #[test]
    fn test_tx_ser() {
        let tx = Transaction {
            message: TransactionMessage::Legacy {
                chain_id: Some(2_u64.into()),
                nonce: 12_u64.into(),
                gas: 21000_u64.into(),
                gas_price: 20_000_000_000_u64.into(),
                to: Some(hex!("727fc6a68321b754475c668a6abfb6e9e71c169a").into()),
                value: 10.as_u256() * 1_000_000_000 * 1_000_000_000,
                input: hex!("a9059cbb000000000213ed0f886efd100b67c7e4ec0a85a7d20dc971600000000000000000000015af1d78b58c4000").to_vec().into(),
            },
            v: 40_u64.into(),
            r: hex!("be67e0a07db67da8d446f76add590e54b6e92cb6b8f9835aeb67540579a27717").into(),
            s: hex!("2d690516512020171c1ec870f6ff45398cc8609250326be89915fb538e7bd718").into(),
            from: Address::repeat_byte(0xAA),
            hash: H256::repeat_byte(0xBB),
            transaction_index: Some(0x42.into()),
            block_hash: None,
            block_number: None,
        };
        let serialized = json!({
            "type": "0x0",
            "chainId": "0x2",
            "nonce": "0xc",
            "to": "0x727fc6a68321b754475c668a6abfb6e9e71c169a",
            "gas": "0x5208",
            "gasPrice":"0x4a817c800",
            "value":"0x8ac7230489e80000",
            "input":"0xa9059cbb000000000213ed0f886efd100b67c7e4ec0a85a7d20dc971600000000000000000000015af1d78b58c4000",
            "v":"0x28",
            "r":"0xbe67e0a07db67da8d446f76add590e54b6e92cb6b8f9835aeb67540579a27717",
            "s":"0x2d690516512020171c1ec870f6ff45398cc8609250326be89915fb538e7bd718",
            "from":"0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "hash":"0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "transactionIndex":"0x42",
            "blockHash": null,
            "blockNumber": null,
        });

        assert_eq!(serde_json::to_value(&tx).unwrap(), serialized);
        assert_eq!(
            serde_json::from_value::<Transaction>(serialized).unwrap(),
            tx
        );
    }
}
