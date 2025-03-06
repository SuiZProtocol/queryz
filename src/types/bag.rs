use serde::{Deserialize, Serialize};
use sui_types::base_types::ObjectID;

/// Represents a balance entry in a Sui Bag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balancez {
    /// The coin type (e.g., "0x2::sui::SUI")
    pub coin_type: String,
    /// The balance amount
    pub balance: f64,
}

/// Represents all balances in a Sui Bag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BagBalances {
    /// The ID of the bag object
    pub bag_id: ObjectID,
    /// List of balance entries in the bag
    pub balances: Vec<Balancez>,
} 
