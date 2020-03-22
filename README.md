## Bytom Rust SDK 

[![Crates.io](https://img.shields.io/crates/v/bytom-rust-sdk.svg?style=popout-square)](https://crates.io/crates/bytom-rust-sdk)
[![License](https://img.shields.io/crates/l/bytom-rust-sdk.svg?style=popout-square)](https://github.com/ruster-cn/bytom-rust-sdk/blob/master/LICENSE)

This repo contains the Rust SDK for [Bytom](https://bytom.io/zh/) . Bytom is software designed to operate and connect to highly scalable blockchain networks confirming to the Bytom Blockchain Protocol, which allows partipicants to define, issue and transfer digitial assets on a multi-asset shared ledger. By using Bytom Rust SDK, rust developers can more easily use the [Bytom API](https://github.com/Bytom/bytom/wiki/API-Reference).


## Example

This asynchronous example uses Tokio and enables some optional features, so your Cargo.toml could look like this:

```toml
[dependencies]
tokio = {version="0.2.9",features = ["full"] }
bytom-rust-sdk = {version="0.1.0"}
```

And then the code:

```rust
#[tokio::main]
async fn main() {
    create_key().await;
}

#[allow(dead_code)]
async fn create_key() {
    let client = BtmClient::new("47.103.115.37:9888".to_string(), "http".to_string());
    client
        .create_key("delete-key", "delete-key", "en", "")
        .await
        .map_err(|err| {
            println!("{}", err.to_string());
        })
        .map(|response| println!("{:?}", response))
        .ok();
}
```


For more usage examples, please refer to [example](https://github.com/ruster-cn/bytom-rust-sdk/blob/master/example/src/main.rs)

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

