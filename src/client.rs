use anyhow::Result;
use sui_json_rpc_types::{Balance, DynamicFieldPage, SuiCoinMetadata, SuiObjectDataOptions, SuiObjectResponse};
use std::sync::Arc;
use sui_sdk::{SuiClient, SuiClientBuilder};
use sui_sdk::types::base_types::SuiAddress;
use sui_types::base_types::ObjectID;

use crate::core_clients::coin_metadata::CoinMetadataClient;

/// SuiClient is a wrapper around the Sui SDK client
/// It provides simplified access to common Sui operations
#[derive(Clone)]
pub struct SuiQueryZClient {
    sui_client: Arc<SuiClient>,
    coin_metadata_client: Arc<CoinMetadataClient>,
}

impl SuiQueryZClient {
    /// Create a new SuiClient connected to the specified RPC URL
    ///
    /// # Arguments
    /// * `rpc_url` - The URL of the Sui RPC endpoint
    ///
    /// # Returns
    /// * `Result<SuiClient>` - A new SuiClient instance or an error
    pub async fn new(rpc_url: &str) -> Result<Self> {
        let sui_client = SuiClientBuilder::default()
            .build(rpc_url)
            .await?;

        let coin_metadata_client = CoinMetadataClient::new(Arc::new(sui_client.clone()));
        
        Ok(Self {
            sui_client: Arc::new(sui_client),
            coin_metadata_client: Arc::new(coin_metadata_client),
        })
    }
    
    /// Get the inner SuiClient
    ///
    /// # Returns
    /// * `Arc<sui_sdk::SuiClient>` - The inner SuiClient
    pub fn sui_client(&self) -> Arc<sui_sdk::SuiClient> {
        self.sui_client.clone()
    }
    
    /// Get object data by ID
    ///
    /// # Arguments
    /// * `object_id` - The ID of the object to retrieve
    ///
    /// # Returns
    /// * `Result<sui_sdk::rpc_types::SuiObjectResponse>` - The object data or an error
    pub async fn get_object(&self, object_id: ObjectID) -> Result<SuiObjectResponse> {
        Ok(self.sui_client.read_api().get_object_with_options(object_id, SuiObjectDataOptions::default()).await?)
    }
    
    /// Get objects owned by an address
    ///
    /// # Arguments
    /// * `address` - The address to query
    ///
    /// # Returns
    /// * `Result<Vec<sui_sdk::rpc_types::SuiObjectResponse>>` - The objects owned by the address or an error
    pub async fn get_objects_owned_by_address(&self, address: SuiAddress) -> Result<Vec<SuiObjectResponse>> {
        let objects = self.sui_client.read_api().get_owned_objects(address, None, None, None).await?;
        Ok(objects.data)
    }

    pub async fn get_dynamic_fields(&self, object_id: ObjectID, cursor: Option<ObjectID>, limit: Option<usize>) -> Result<DynamicFieldPage> {
        Ok(self.sui_client.read_api().get_dynamic_fields(object_id, cursor, limit).await?)
    }

    pub async fn multi_get_object_with_options(&self, object_ids: Vec<ObjectID>, options: SuiObjectDataOptions) -> Result<Vec<SuiObjectResponse>> {
        Ok(self.sui_client.read_api().multi_get_object_with_options(object_ids, options).await?)
    }

    /// Get coin metadata
    ///
    /// # Arguments
    /// * `coin_type` - The type of the coin to get metadata for
    ///
    /// # Returns
    /// * `Result<CoinMetadata>` - The metadata for the coin or an error
    pub async fn get_coin_metadata(&self, coin_type: &str) -> Result<SuiCoinMetadata> {
        self.coin_metadata_client.get_metadata(coin_type).await
    }

    pub async fn get_coin_balance(&self, address: SuiAddress, coin_type: &str) -> Result<u64> {
        let balance = self.sui_client.coin_read_api().get_all_balances(address).await?;
        let coin_balance = balance.iter().find(|balance| balance.coin_type == coin_type).map(|balance| balance.total_balance).unwrap_or(0);
        Ok(coin_balance as u64)
    }

    pub async fn get_coin_balances(&self, address: SuiAddress) -> Result<Vec<Balance>> {
        let balance = self.sui_client.coin_read_api().get_all_balances(address).await?;
        Ok(balance)
    }
} 
