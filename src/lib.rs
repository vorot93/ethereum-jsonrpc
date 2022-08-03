#![doc = include_str!("../README.md")]

mod debug;
mod engine;
mod erigon;
mod eth;
mod net;
mod otterscan;
mod trace;
pub mod types;
mod web3;

pub use debug::*;
pub use engine::*;
pub use erigon::*;
pub use eth::*;
pub use net::*;
pub use otterscan::*;
pub use trace::*;
pub use web3::*;

mod prelude {
    pub use crate::types::*;
    pub use arrayvec::ArrayVec;
    pub use ethereum_types::{Address, Bloom, H256, H64, U64};
    pub use ethnum::prelude::*;
    pub use std::collections::HashSet;

    #[cfg(any(feature = "client", feature = "server"))]
    pub use jsonrpsee::core::RpcResult;
    pub use jsonrpsee::proc_macros::rpc;
    pub use serde::de::{self, value::MapAccessDeserializer, Visitor};
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub use serde_with::{formats::PreferOne, serde_as, OneOrMany};
}
