use anyhow::Result;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use sui_json_rpc_types::SuiCoinMetadata;
use sui_sdk::SuiClient;

/// CoinMetadataClient manages coin metadata
/// Contains a cache and SUI client to fetch metadata from chain or read from cache
pub struct CoinMetadataClient {
    /// SUI client
    client: Arc<SuiClient>,
    /// Cache using coin type address as key
    cache: RwLock<HashMap<String, SuiCoinMetadata>>,
}

impl CoinMetadataClient {
    /// Create a new CoinMetadataClient
    ///
    /// # Arguments
    /// * `client` - SuiQueryZClient instance
    ///
    /// # Returns
    /// * `CoinMetadataClient` - A new manager instance
    pub fn new(client: Arc<SuiClient>) -> Self {
        Self {
            client,
            cache: RwLock::new(HashMap::new()),
        }
    }

    /// Get coin metadata
    ///
    /// # Arguments
    /// * `coin_type` - Coin type string, e.g. "0x2::sui::SUI"
    ///
    /// # Returns
    /// * `Result<CoinMetadata>` - Coin metadata or error
    pub async fn get_metadata(&self, coin_type: &str) -> Result<SuiCoinMetadata> {
        // First check the cache
        {
            let cache = self.cache.read().await;
            if let Some(metadata) = cache.get(coin_type) {
                return Ok(metadata.clone());
            }
        }

        // Not in cache, fetch from chain
        let metadata = self.fetch_metadata_from_chain(coin_type).await?;
        
        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(coin_type.to_string(), metadata.clone());
        }
        
        Ok(metadata)
    }

    /// Fetch coin metadata from the blockchain
    ///
    /// # Arguments
    /// * `coin_type` - Coin type string
    ///
    /// # Returns
    /// * `Result<CoinMetadata>` - Coin metadata or error
    async fn fetch_metadata_from_chain(&self, coin_type: &str) -> Result<SuiCoinMetadata> {
        let resp = self.client.coin_read_api().get_coin_metadata(coin_type.to_string()).await?;
        match resp {
            Some(metadata) => Ok(metadata),
            None => Err(anyhow::anyhow!("Coin metadata not found, coin_type: {}", coin_type)),
        }
    }

    /// Clear the cache
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
    
    /// Manually add or update metadata in cache
    pub async fn update_cache(&self, coin_type: &str, metadata: SuiCoinMetadata) {
        let mut cache = self.cache.write().await;
        cache.insert(coin_type.to_string(), metadata);
    }
}
