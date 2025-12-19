use lum_libs::serde::{Deserialize, Serialize};

use crate::provider::{hetzner, netcup, nitrado};
use crate::types;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum Type {
    Nitrado(nitrado::DnsConfig),
    Hetzner(hetzner::DnsConfig),
    Netcup(netcup::DnsConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum RecordConfig {
    Manual(types::dns::Record),
    Automatic(AutomaticRecordConfig),
}

/// Configuration for an automatically updated DNS record.
///
/// # Examples
///
/// ```
/// use dnrs::config::dns::{AutomaticRecordConfig, ResolveType};
///
/// let config = AutomaticRecordConfig {
///     domain: "home.example.com".to_string(),
///     ttl: Some(300),
///     resolve_type: ResolveType::IPv4,
/// };
///
/// assert_eq!(config.domain, "home.example.com");
/// assert_eq!(config.ttl, Some(300));
/// ```
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
    // https://github.com/acatton/serde-yaml-ng/issues/14
    //#[serde(flatten)]
    pub dns: Vec<Type>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dns: vec![
                Type::Nitrado(nitrado::DnsConfig::default()),
                Type::Hetzner(hetzner::DnsConfig::default()),
            ],
        }
    }
}
