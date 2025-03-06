use anyhow::{anyhow, Result};
use sui_json_rpc_types::{SuiObjectData, SuiObjectResponse};
use sui_types::base_types::ObjectID;

/// Extract object data from a SuiObjectResponse
///
/// # Arguments
/// * `response` - The SuiObjectResponse to extract data from
///
/// # Returns
/// * `Result<SuiObjectData>` - The extracted object data or an error
pub fn extract_object_data(response: SuiObjectResponse) -> Result<SuiObjectData> {
    response.data.ok_or_else(|| anyhow!("Object not found"))
}

/// Parse an object ID string
///
/// # Arguments
/// * `id_str` - The object ID string to parse
///
/// # Returns
/// * `Result<ObjectID>` - The parsed ObjectID or an error
pub fn parse_object_id(id_str: &str) -> Result<ObjectID> {
    id_str.parse::<ObjectID>().map_err(|e| anyhow!("Failed to parse object ID: {}", e))
} 
