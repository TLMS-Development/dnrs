use lum_libs::serde::{Deserialize, Serialize};

use crate::{provider::nitrado, types};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum Type {
    Nitrado(nitrado::DnsConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum RecordConfig {
    Manual(types::dns::Record),
    Automatic(AutomaticRecordConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct AutomaticRecordConfig {
    pub domain: String,
    pub ttl: Option<u32>,
    pub resolve_type: ResolveType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum ResolveType {
    IPv4,
    IPv6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Config {
    #[serde(flatten)]
    pub dns: Type,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dns: Type::Nitrado(nitrado::DnsConfig::default()),
        }
    }
}
