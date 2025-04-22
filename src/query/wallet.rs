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

    pub async fn get_balance_by_coin_type(&self, address: &str, coin_type: &str) -> Result<u64> {
        let wallet_address = SuiAddress::from_str(address)?;
        let balance = self.client.get_coin_balance(wallet_address, coin_type).await?;
        Ok(balance)
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

    pub async fn process_all_balances(&self, balances: HashMap<String, u64>) -> Result<HashMap<String, f64>> {
        let mut processed_balances = HashMap::new();

        for (coin_type, balance) in balances {
            let metadata = self.client.get_coin_metadata(&coin_type).await?;
            let actual_balance = balance as f64 / 10.0_f64.powi(metadata.decimals as i32);
            processed_balances.insert(metadata.symbol, actual_balance);
        }

        Ok(processed_balances)
    }

    pub async fn get_wallet_balances(&self, address: &str) -> Result<HashMap<String, f64>> {
        let balances = self.get_all_balances(address).await?;
        let processed_balances = self.process_all_balances(balances).await?;
        Ok(processed_balances)
    }

    pub async fn get_wallet_balances_by_coin_types(&self, address: &str, coin_types: Vec<String>) -> Result<HashMap<String, f64>> {
        let mut balances = HashMap::new();
        for coin_type in coin_types {
            let balance = self.get_balance_by_coin_type(address, &coin_type).await?;
            balances.insert(coin_type, balance);
        }

        let processed_balances = self.process_all_balances(balances).await?;
        Ok(processed_balances)
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
    async fn get_wallet_balances(&self, address: &str) -> Result<HashMap<String, f64>>;

    /// Get all balances in a wallet by coin types
    /// 
    /// # Arguments
    /// * `address` - The address of the wallet to query
    /// * `coin_types` - The coin types to query
    ///
    /// # Returns
    /// * `Result<HashMap<String, f64>>` - The balances in the wallet or an error
    async fn get_wallet_balances_by_coin_types(&self, address: &str, coin_types: Vec<String>) -> Result<HashMap<String, f64>>;
}

#[async_trait]
impl WalletQuerier for SuiQueryZClient {
    async fn get_wallet_balances(&self, address: &str) -> Result<HashMap<String, f64>> {
        let query = WalletQuery::new(Arc::new(self.clone()));
        query.get_wallet_balances(address).await    
    }

    async fn get_wallet_balances_by_coin_types(&self, address: &str, coin_types: Vec<String>) -> Result<HashMap<String, f64>> {
        let query = WalletQuery::new(Arc::new(self.clone()));
        query.get_wallet_balances_by_coin_types(address, coin_types).await
    }
} 
