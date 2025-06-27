use std::{
    net::{Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use lum_libs::{
    serde::{Deserialize, Serialize},
    serde_json,
};
use reqwest::header::HeaderMap;
use thiserror::Error;

use crate::{
    provider::{Feature, Provider},
    types::dns::{self, RecordValue},
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

pub struct NitradoProvider<'provider_config, 'auto_config> {
    pub provider_config: &'provider_config ProviderConfig,
    pub auto_config: &'auto_config AutoConfig,
}

impl<'provider_config, 'auto_config> NitradoProvider<'provider_config, 'auto_config> {
    pub fn new(
        provider_config: &'provider_config ProviderConfig,
        auto_config: &'auto_config AutoConfig,
    ) -> NitradoProvider<'provider_config, 'auto_config> {
        NitradoProvider {
            provider_config,
            auto_config,
        }
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
pub struct GetRecordInput {}
pub struct GetRecordsInput {
    pub domain: String,
}
pub struct AddRecordInput {}
pub struct UpdateRecordInput {}
pub struct DeleteRecordInput {}

impl Provider for NitradoProvider<'_, '_> {
    type GetRecordInput = GetRecordInput;
    type GetRecordOutput = serde_json::Value;
    type GetRecordsInput = GetRecordsInput;
    type GetRecordsOutput = Result<serde_json::Value, Error>;
    type AddRecordInput = AddRecordInput;
    type AddRecordOutput = serde_json::Value;
    type UpdateRecordInput = UpdateRecordInput;
    type UpdateRecordOutput = serde_json::Value;
    type DeleteRecordInput = DeleteRecordInput;
    type DeleteRecordOutput = serde_json::Value;

    fn get_provider_name() -> &'static str {
        "Nitrado"
    }

    fn get_supported_features() -> Vec<Feature> {
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
        _input: &Self::GetRecordInput,
    ) -> Self::GetRecordOutput {
        unimplemented!()
    }

    async fn get_records(
        &self,
        reqwest: reqwest::Client,
        input: &Self::GetRecordsInput,
    ) -> Self::GetRecordsOutput {
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
            return Err(Error::Unsuccessful(response.status().as_u16(), response));
        }

        let text = response.text().await?;
        let json = serde_json::from_str(&text)?;
        Ok(json)
    }

    async fn add_record(
        &self,
        _reqwest: reqwest::Client,
        _input: &Self::AddRecordInput,
    ) -> Self::AddRecordOutput {
        unimplemented!()
    }

    async fn update_record(
        &self,
        _reqwest: reqwest::Client,
        _input: &Self::UpdateRecordInput,
    ) -> Self::UpdateRecordOutput {
        unimplemented!()
    }

    async fn delete_record(
        &self,
        _reqwest: reqwest::Client,
        _input: &Self::DeleteRecordInput,
    ) -> Self::DeleteRecordOutput {
        unimplemented!()
    }
}
