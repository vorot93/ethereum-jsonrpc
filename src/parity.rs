#[allow(unused_imports)]
use crate::prelude::*;

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "parity"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "parity"))]
pub trait ParityApi {
    #[method(name = "listStorageKeys")]
    async fn list_storage_keys(
        &self,
        address: Address,
        number_of_slots: NonZeroUsize,
        offset: Option<H256>,
        block: Option<BlockId>,
    ) -> RpcResult<Option<BTreeSet<H256>>>;
}
