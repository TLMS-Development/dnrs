use lum_libs::serde::{Deserialize, Serialize};

use crate::config::dns::RecordConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Config {
    pub name: String,
    pub customer_number: u32,
    pub api_key: String,
    pub api_password: String,
    pub api_base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "Netcup1".to_string(),
            customer_number: 0,
            api_key: "your_api_key".to_string(),
            api_password: "your_api_password".to_string(),
            api_base_url: "https://ccp.netcup.net/run/webservice/servers/endpoint.php".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct DomainConfig {
    pub domain: String,
    pub records: Vec<RecordConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct DnsConfig {
    pub provider_name: String,
    pub domains: Vec<DomainConfig>,
}

impl Default for DnsConfig {
    fn default() -> Self {
        DnsConfig {
            provider_name: "Netcup1".to_string(),
            domains: vec![],
        }
    }
}
