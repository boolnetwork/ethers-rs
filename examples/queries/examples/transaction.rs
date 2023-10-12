use ethers::{
    core::{
        abi::AbiDecode,
        types::{BlockNumber, Filter, U256, H256},
    },
    providers::{Middleware, Provider, StreamExt, Ws, Http},
};
use eyre::Result;
use std::sync::Arc;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::registry().with(fmt::layer()).init();

    let client: Provider<Http> =
        Provider::<Http>::try_from("https://zksync.drpc.org").unwrap();
    let client = Arc::new(client);

    let tx_hash = H256::from_str("0xf1514d3594f4ad36623d26ad560109dfda8fa47461fd4269b1783f1a8b5e7a52").unwrap();
    let tx = client.get_transaction(tx_hash).await?;
    println!("{tx:?}");

    Ok(())
}
