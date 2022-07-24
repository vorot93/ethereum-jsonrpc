use crate::prelude::*;

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "web3"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "web3"))]
pub trait Web3Api {
    #[method(name = "clientVersion")]
    async fn client_version(&self) -> RpcResult<String>;
}
