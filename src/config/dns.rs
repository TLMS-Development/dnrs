use lum_libs::serde::{Deserialize, Serialize};

use crate::provider::nitrado;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum Type {
    Nitrado(nitrado::DnsConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum Record<MANUAL, AUTOMATIC> {
    Manual(MANUAL),
    Automatic(AUTOMATIC),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum ResolveType {
    IPv4,
    IPv6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct FileConfig {
    #[serde(flatten)]
    pub dns: Type,
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            dns: Type::Nitrado(nitrado::DnsConfig::default()),
        }
    }
}
