//! Example of subscribing to blocks and watching blocks.

use alloy::{network::Ethereum, node_bindings::Anvil};
use alloy_provider::{Provider, RootProvider};
use alloy_rpc_client::RpcClient;
use eyre::Result;
use futures_util::{stream, StreamExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Spin up a local Anvil node.
    // Ensure `anvil` is available in $PATH
    let anvil = Anvil::new().block_time(1).try_spawn()?;

    let ws = alloy_rpc_client::WsConnect::new(anvil.ws_endpoint());
    let provider = RootProvider::<Ethereum, _>::new(RpcClient::connect_pubsub(ws).await?);

    let sub = provider.subscribe_blocks().await?;
    let mut stream = sub.into_stream().take(2);

    while let Some(block) = stream.next().await {
        println!("Subscribed Block: {:?}", block.header.number);
    }

    let poller = provider.watch_blocks().await?;
    let mut stream = poller.into_stream().flat_map(stream::iter).take(2);

    while let Some(block_hash) = stream.next().await {
        println!("Watched Block: {:?}", block_hash);
    }

    Ok(())
}