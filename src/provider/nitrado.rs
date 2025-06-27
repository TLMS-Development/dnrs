use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use lum_libs::serde::{Deserialize, Serialize};

use crate::types::dns::{self, RecordValue};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct ProviderConfig {
    pub name: String,
    pub api_key: String,
    pub api_base_url: String,
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
    pub name: String,
    pub domains: Vec<Domain>,
}

impl Default for AutoConfig {
    fn default() -> Self {
        AutoConfig {
            name: "Nitrado".to_string(),
            domains: vec![Domain {
                domain: "example.com".to_string(),
                records: vec![
                    Record {
                        record: dns::Record {
                            name: "ipv4".to_string(),
                            value: RecordValue::A(Ipv4Addr::from_str("127.0.0.1").unwrap()),
                        },
                        ttl: Some(3600),
                    },
                    Record {
                        record: dns::Record {
                            name: "ipv6".to_string(),
                            value: RecordValue::AAAA(Ipv6Addr::from_str("::1").unwrap()),
                        },
                        ttl: Some(3600),
                    },
                    Record {
                        record: dns::Record {
                            name: "forward".to_string(),
                            value: RecordValue::CNAME("example.com".to_string()),
                        },
                        ttl: Some(3600),
                    },
                ],
            }],
        }
    }
}
