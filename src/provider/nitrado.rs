use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use anyhow::Result;
use async_trait::async_trait;
use lum_libs::{
    serde::{Deserialize, Serialize},
    serde_json,
};
use reqwest::header::HeaderMap;
use thiserror::Error;

use crate::{
    config::dns::{AutomaticRecordConfig, RecordConfig, ResolveType},
    provider::{self, Feature, Provider},
    types::dns::{self, Record, RecordValue},
};

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
            name: "Nitrado1".to_string(),
            api_key: "your_api_key".to_string(),
            api_base_url: "https://api.nitrado.net".to_string(),
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
            provider_name: "Nitrado1".to_string(),
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

pub struct NitradoProvider<'provider_config> {
    pub provider_config: &'provider_config ProviderConfig,
}

impl<'provider_config> NitradoProvider<'provider_config> {
    pub fn new(
        provider_config: &'provider_config ProviderConfig,
    ) -> NitradoProvider<'provider_config> {
        NitradoProvider { provider_config }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("HTTP response is not successful: {0}")]
    Unsuccessful(u16, reqwest::Response),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
}

#[async_trait]
impl Provider for NitradoProvider<'_> {
    fn get_provider_name(&self) -> &'static str {
        "Nitrado"
    }

    fn get_supported_features(&self) -> Vec<Feature> {
        vec![
            Feature::GetRecord,
            Feature::GetRecords,
            Feature::AddRecord,
            Feature::UpdateRecord,
            Feature::DeleteRecord,
        ]
    }

    async fn get_record(
        &self,
        _reqwest: reqwest::Client,
        _input: &provider::GetRecordInput,
    ) -> Result<Record> {
        unimplemented!()
    }

    async fn get_records(
        &self,
        reqwest: reqwest::Client,
        input: &provider::GetRecordsInput,
    ) -> Result<Vec<Record>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.provider_config.api_key)
                .parse()
                .unwrap(),
        );

        let domain = &input.domain;
        let url = format!(
            "{}/domain/{}/records",
            self.provider_config.api_base_url, domain
        );
        let response = reqwest.get(&url).headers(headers).send().await?;

        if !response.status().is_success() {
            return Err(Error::Unsuccessful(response.status().as_u16(), response).into());
        }

        let text = response.text().await?;
        let json = serde_json::from_str(&text)?;
        Ok(json)
    }

    async fn add_record(&self, _reqwest: reqwest::Client, _input: &Record) -> Result<()> {
        unimplemented!()
    }

    async fn update_record(&self, _reqwest: reqwest::Client, _input: &Record) -> Result<()> {
        unimplemented!()
    }

    async fn delete_record(&self, _reqwest: reqwest::Client, _input: &Record) -> Result<()> {
        unimplemented!()
    }
}
