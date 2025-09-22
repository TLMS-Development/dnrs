use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use lum_libs::serde::{Deserialize, Serialize};

use crate::{
    config::dns::{AutomaticRecordConfig, RecordConfig, ResolveType},
    types::dns::{self, MxRecord, RecordValue},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Config {
    pub name: String,
    pub api_key: String,
    pub api_base_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "Hetzner1".to_string(),
            api_key: "your_api_key".to_string(),
            api_base_url: "https://dns.hetzner.com/api/v1".to_string(),
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
            provider_name: "Hetzner1".to_string(),
            domains: vec![DomainConfig {
                domain: "example.com".to_string(),
                records: vec![
                    RecordConfig::Manual(dns::Record {
                        domain: "ipv4".to_string(),
                        value: RecordValue::A(Ipv4Addr::from_str("127.0.0.1").unwrap()),
                        ttl: Some(3600),
                    }),
                    RecordConfig::Manual(dns::Record {
                        domain: "ipv6".to_string(),
                        value: RecordValue::AAAA(Ipv6Addr::from_str("::1").unwrap()),
                        ttl: Some(3600),
                    }),
                    RecordConfig::Manual(dns::Record {
                        domain: "forward".to_string(),
                        value: RecordValue::CNAME("example.com".to_string()),
                        ttl: Some(3600),
                    }),
                    RecordConfig::Manual(dns::Record {
                        domain: "@".to_string(),
                        value: RecordValue::TXT("v=spf1 include:example.com ~all".to_string()),
                        ttl: Some(3600),
                    }),
                    RecordConfig::Manual(dns::Record {
                        domain: "@".to_string(),
                        value: RecordValue::MX(MxRecord {
                            priority: 10,
                            target: "mail.example.com".to_string(),
                        }),
                        ttl: Some(3600),
                    }),
                    RecordConfig::Manual(dns::Record {
                        domain: "@".to_string(),
                        value: RecordValue::SRV(10, 10, 10, "example.com".to_string()),
                        ttl: Some(3600),
                    }),
                    RecordConfig::Manual(dns::Record {
                        domain: "_443._tcp".to_string(),
                        value: RecordValue::TLSA(3, 1, 1, "abcdef0123456789".to_string()),
                        ttl: Some(3600),
                    }),
                    RecordConfig::Manual(dns::Record {
                        domain: "@".to_string(),
                        value: RecordValue::CAA(
                            0,
                            "issue".to_string(),
                            "letsencrypt.org".to_string(),
                        ),
                        ttl: Some(3600),
                    }),
                    RecordConfig::Automatic(AutomaticRecordConfig {
                        domain: "auto-ipv4".to_string(),
                        ttl: Some(300),
                        resolve_type: ResolveType::IPv4,
                    }),
                    RecordConfig::Automatic(AutomaticRecordConfig {
                        domain: "auto-ipv6".to_string(),
                        ttl: Some(300),
                        resolve_type: ResolveType::IPv6,
                    }),
                ],
            }],
        }
    }
}
