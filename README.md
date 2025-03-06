# Queryz

A utility library for querying complex objects on the Sui blockchain.

## Features

- Simplified interface for interacting with Sui objects
- Query Sui Bag objects and retrieve all balances
- Easy to use API with async/await support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
queryz = "0.1.0"
```

## Usage

### Querying Bag Balances

```rust
use anyhow::Result;
use queryz::{SuiClient, query::bag::BagQuerier};
use sui_types::base_types::ObjectID;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to Sui RPC endpoint
    let rpc_url = "https://fullnode.mainnet.sui.io:443";
    let client = SuiClient::new(rpc_url).await?;
    
    // Example bag object ID (replace with a real bag object ID)
    let bag_id_str = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
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
```

## Examples

Run the examples with:

```bash
cargo run --example bag_query
```

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
