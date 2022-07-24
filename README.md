# ethereum-jsonrpc
This crate contains definitions for various Ethereum JSONRPC APIs using [jsonrpsee](https://github.com/paritytech/jsonrpsee) framework.

## Client usage example
Enable `client` feature of `ethereum-jsonrpc` crate.

```rust,no_run
use ethereum_jsonrpc::EthApiClient;
use jsonrpsee::http_client::HttpClientBuilder;

#[tokio::main]
async fn main() {
    let client = HttpClientBuilder::default().build("http://localhost:8545").unwrap();

    let block_number = client.block_number().await.unwrap();
    println!("Current block number is {block_number}");
}
```

## License
The entire code within this repository is licensed under the [Mozilla Public License v2.0](./LICENSE)
