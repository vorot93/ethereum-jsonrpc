#[allow(unused_imports)]
use crate::prelude::*;

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "erigon"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "erigon"))]
pub trait ErigonApi {
    #[method(name = "getHeaderByNumber")]
    async fn get_header_by_number(&self, block_number: u64) -> RpcResult<Option<Header>>;
}
