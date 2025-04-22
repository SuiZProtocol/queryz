use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use sui_types::TypeTag;

use crate::utils::coin::format_coin_address;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Default)]
pub struct TypeName {
    pub name: String,
}

impl TypeName {
    pub fn into_string(&self) -> String {
        format_coin_address(self.name.as_str())
    }
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.into_string())
    }
}

impl FromStr for TypeName {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let type_tag = TypeTag::from_str(s)?;
        Ok(TypeName {
            name: type_tag.to_string(),
        })
    }
}

impl PartialEq for TypeName {
    fn eq(&self, other: &Self) -> bool {
        self.into_string().eq(&other.into_string())
    }
}
