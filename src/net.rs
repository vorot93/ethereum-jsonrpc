#[allow(unused_imports)]
use crate::prelude::*;

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "net"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "net"))]
pub trait NetApi {
    #[method(name = "listening")]
    async fn listening(&self) -> RpcResult<bool>;
    #[method(name = "peerCount")]
    async fn peer_count(&self) -> RpcResult<U64>;
    #[method(name = "version")]
    async fn version(&self) -> RpcResult<StringU64>;
}
