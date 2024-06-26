//! Example of querying contract storage from the Ethereum network.

use alloy::{
    primitives::{address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::BlockId,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a provider.
    let rpc_url = "https://eth.merkle.io".parse()?;
    let provider = ProviderBuilder::new().on_http(rpc_url)?;

    // Get storage slot 0 from the Uniswap V3 USDC-ETH pool on Ethereum mainnet.
    let pool_address = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");
    let storage_slot = U256::from(0);
    let storage = provider.get_storage_at(pool_address, storage_slot, BlockId::latest()).await?;

    println!("Slot 0: {storage:?}");

    Ok(())
}
