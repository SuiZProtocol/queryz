use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use sui_types::coin::Coin;
use sui_types::dynamic_field::Field;
use sui_types::base_types::ObjectID;
use std::sync::Arc;
use sui_json_rpc_types::{SuiData, SuiObjectDataOptions};
use crate::client::SuiQueryZClient;
use crate::types::bag::{Balancez, BagBalances};
use crate::types::type_name::TypeName;
/// BagQuery provides methods to query Sui Bag objects
#[derive(Clone)]
pub struct BagQuery {
    client: Arc<SuiQueryZClient>,
}

impl BagQuery {
    /// Create a new BagQuery instance
    ///
    /// # Arguments
    /// * `client` - The SuiClient to use for queries
    ///
    /// # Returns
    /// * `BagQuery` - A new BagQuery instance
    pub fn new(client: Arc<SuiQueryZClient>) -> Self {
        Self { client }
    }

    /// Get all balances in a Bag object
    ///
    /// # Arguments
    /// * `bag_id` - The ID of the Bag object to query
    ///
    /// # Returns
    /// * `Result<BagBalances>` - The balances in the Bag or an error
    pub async fn get_bag_raw_fields<T: DeserializeOwned>(&self, bag_id: ObjectID) -> Result<Vec<T>> {
        let mut fields: Vec<T> = vec![];
        let mut cursor: Option<ObjectID> = None;
    
        let limit = 50;
        loop {
            let fields_resp = self.client
                .get_dynamic_fields(bag_id, cursor, Some(limit))
                .await?;
            cursor = fields_resp.next_cursor;
            let field_ids = fields_resp
                .data
                .iter()
                .map(|x| x.object_id)
                .collect::<Vec<ObjectID>>();

            let resp = self.client
                .multi_get_object_with_options(field_ids.clone(), SuiObjectDataOptions::bcs_lossless())
                .await?;
    
            for i in 0..field_ids.len() {
                let item = &resp[i];
                if item.data.is_none() {
                    continue;
                }
                let data = item.data.clone().ok_or(anyhow!("object data is none"))?;
                let field: T = bcs::from_bytes(
                    data.clone()
                        .bcs
                        .ok_or(anyhow!("object data bcs is none"))?
                        .try_as_move()
                        .ok_or(anyhow!("object data bcs is not move"))?
                        .bcs_bytes
                        .as_ref(),
                )?;
                fields.push(field);
            }
    
            if !fields_resp.has_next_page {
                break;
            }
        }
        Ok(fields)
    }   

    /// Convert raw field data to balance information
    ///
    /// # Arguments
    /// * `bag_id` - The ID of the Bag object
    /// * `fields` - Raw field data retrieved from the chain
    ///
    /// # Returns
    /// * `BagBalances` - Processed balance information
    async fn process_bag_balances(&self, bag_id: ObjectID, fields: Vec<Field<TypeName, Coin>>) -> Result<BagBalances> {
        let mut balances: Vec<Balancez> = vec![];

        for field in fields {
            let metadata = self.client.get_coin_metadata(&field.name.to_string()).await?;

            let actual_balance = field.value.value() as f64 / 10.0_f64.powi(metadata.decimals as i32);
            let balance = Balancez {
                symbol: metadata.symbol,
                coin_type: field.name.to_string(),
                balance: actual_balance,
            };
            balances.push(balance);
        }
        
        Ok(BagBalances { bag_id, balances })
    }

    /// Get all balances in a Bag object
    ///
    /// # Arguments
    /// * `bag_id` - The ID of the Bag object to query
    ///
    /// # Returns
    /// * `Result<BagBalances>` - The balances in the Bag or an error
    pub async fn get_bag_balances(&self, bag_id: ObjectID) -> Result<BagBalances> {
        let fields: Vec<Field<TypeName, Coin>> = self.get_bag_raw_fields(bag_id).await?;
        self.process_bag_balances(bag_id, fields).await
    }
}

/// Trait for querying Bag objects
#[async_trait]
pub trait BagQuerier {
    /// Get all balances in a Bag object
    ///
    /// # Arguments
    /// * `bag_id` - The ID of the Bag object to query
    ///
    /// # Returns
    /// * `Result<BagBalances>` - The balances in the Bag or an error
    async fn get_bag_balances(&self, bag_id: ObjectID) -> Result<BagBalances>;
}

#[async_trait]
impl BagQuerier for SuiQueryZClient {
    async fn get_bag_balances(&self, bag_id: ObjectID) -> Result<BagBalances> {
        let query = BagQuery::new(Arc::new(self.clone()));
        query.get_bag_balances(bag_id).await
    }
} 
