use lum_libs::serde::{Deserialize, Serialize};

use crate::types::dns;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct ProviderConfig {
    pub name: String,
    pub api_key: String,
    pub api_base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Record {
    pub record: dns::Record,
    pub ttl: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Domain {
    pub domain: String,
    pub records: Vec<Record>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct AutoConfig {
    pub domains: Vec<Domain>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        ProviderConfig {
            name: "Nitrado".to_string(),
            api_key: "your_api_key".to_string(),
            api_base_url: "https://api.nitrado.net".to_string(),
        }
    }
}

//TODO:
// - Provider trait
//   - Get records
//   - Get record
//   - Add record
//   - Update record
//   - Delete record
// - Implement Nitrado provider
