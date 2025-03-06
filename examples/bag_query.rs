use anyhow::Result;
use queryz::{SuiQueryZClient, query::bag::BagQuerier};
use sui_types::base_types::ObjectID;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to Sui RPC endpoint
    let rpc_url = "https://fullnode.mainnet.sui.io:443";
    let client = SuiQueryZClient::new(rpc_url).await?;
    
    // Example bag object ID (replace with a real bag object ID)
    let bag_id_str = "0x64ac48a57c8dfb3f69d5b0956be0c6727267978a11a53659c71f77c13c58aaad";
    let bag_id = bag_id_str.parse::<ObjectID>()?;
    
    // Query bag balances
    let balances = client.get_bag_balances(bag_id).await?;
    
    // Print results
    println!("Bag ID: {}", balances.bag_id);
    println!("Balances:");
    
    for balance in balances.balances {
        println!("  {} - {} units", balance.coin_type, balance.balance);
    }
    
    Ok(())
} 
