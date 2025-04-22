use anyhow::Result;
use async_trait::async_trait;
use sui_types::base_types::SuiAddress;
use std::{collections::HashMap, str::FromStr, sync::Arc};

use crate::SuiQueryZClient;

pub struct WalletQuery {
    client: Arc<SuiQueryZClient>,
}

impl WalletQuery {
    pub fn new(client: Arc<SuiQueryZClient>) -> Self {
        Self { client }
    }

    pub async fn get_all_balances(&self, address: &str) -> Result<HashMap<String, u64>> {
        let wallet_address = SuiAddress::from_str(address)?;
        // println!("Wallet Balance for address: {}", address);

        let balances = self.client
            .get_coin_balances(wallet_address)
            .await?;
        
        let mut balances_map = HashMap::new();
        for balance in balances {
            balances_map.insert(balance.coin_type, balance.total_balance as u64);
        }

        Ok(balances_map)
    }
}

#[async_trait]
pub trait WalletQuerier {
    /// Get all balances in a wallet
    /// 
    /// # Arguments
    /// * `address` - The address of the wallet to query
    ///
    /// # Returns
    /// * `Result<HashMap<String, u64>>` - The balances in the wallet or an error
    async fn get_all_balances(&self, address: &str) -> Result<HashMap<String, u64>>;
}

#[async_trait]
impl WalletQuerier for SuiQueryZClient {
    async fn get_all_balances(&self, address: &str) -> Result<HashMap<String, u64>> {
        let query = WalletQuery::new(Arc::new(self.clone()));
        query.get_all_balances(address).await
    }
} 
