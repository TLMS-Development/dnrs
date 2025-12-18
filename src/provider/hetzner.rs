use anyhow::Result;
use async_trait::async_trait;
use lum_libs::serde_json;
use reqwest::header::HeaderMap;
use thiserror::Error;

use crate::{
    provider::{Feature, GetAllRecordsInput, GetRecordsInput, Provider},
    types::dns::{self},
};

pub mod config;
pub mod model;

pub use config::{Config, DnsConfig, DomainConfig};
pub use model::{GetRecordsResponse, Record, TryFromRecordError};

pub struct HetznerProvider<'provider_config> {
    pub provider_config: &'provider_config Config,
}

impl<'provider_config> HetznerProvider<'provider_config> {
    pub fn new(provider_config: &'provider_config Config) -> HetznerProvider<'provider_config> {
        HetznerProvider { provider_config }
    }

    async fn get_zone_id(&self, reqwest: reqwest::Client, domain: &str) -> Result<String> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Auth-API-Token",
            self.provider_config.api_key.parse().expect("Invalid Hetzner API key: contains characters that are not allowed in HTTP headers"),
        );

        let url = format!("{}/zones", self.provider_config.api_base_url);
        let response = reqwest.get(&url).headers(headers).send().await?;

        if !response.status().is_success() {
            return Err(Error::Unsuccessful(response.status().as_u16(), response).into());
        }

        let text = response.text().await?;
        let json_value: serde_json::Value = serde_json::from_str(&text)?;

        match json_value
            .get("zones")
            .and_then(|zones| zones.as_array())
            .and_then(|zones_array| {
                zones_array.iter().find_map(|zone| {
                    let zone_name = zone.get("name")?.as_str()?;
                    let zone_id = zone.get("id")?.as_str()?;
                    if zone_name == domain {
                        Some(zone_id.to_string())
                    } else {
                        None
                    }
                })
            }) {
            Some(zone_id) => Ok(zone_id),
            None => Err(Error::DomainNotFound(domain.to_string()).into()),
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

    #[error("Domain '{0}' not found in Hetzner zones")]
    DomainNotFound(String),
}

#[async_trait]
impl Provider for HetznerProvider<'_> {
    fn get_provider_name(&self) -> &'static str {
        "Hetzner"
    }

    fn get_supported_features(&self) -> Vec<Feature> {
        vec![
            Feature::GetRecords,
            Feature::GetAllRecords,
            Feature::AddRecord,
            Feature::UpdateRecord,
            Feature::DeleteRecord,
        ]
    }

    async fn get_records(
        &self,
        reqwest: reqwest::Client,
        input: &GetRecordsInput,
    ) -> Result<Vec<dns::Record>> {
        let get_all_records_input = GetAllRecordsInput::from(input);
        let records = self
            .get_all_records(reqwest, &get_all_records_input)
            .await?;

        let filtered_records = records
            .into_iter()
            .filter(|record| input.subdomains.contains(&record.domain.as_str()))
            .collect::<Vec<_>>();

        Ok(filtered_records)
    }

    async fn get_all_records(
        &self,
        reqwest: reqwest::Client,
        input: &GetAllRecordsInput,
    ) -> Result<Vec<dns::Record>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Auth-API-Token",
            self.provider_config.api_key.parse().expect("Invalid Hetzner API key: contains characters that are not allowed in HTTP headers"),
        );

        let domain = &input.domain;
        let zone_id = self.get_zone_id(reqwest.clone(), domain).await?;

        let url = format!(
            "{}/records?zone_id={}",
            self.provider_config.api_base_url, zone_id
        );

        let response = reqwest.get(&url).headers(headers).send().await?;

        if !response.status().is_success() {
            return Err(Error::Unsuccessful(response.status().as_u16(), response).into());
        }

        let text = response.text().await?;
        let response: GetRecordsResponse = serde_json::from_str(&text)?;
        let records: Vec<dns::Record> = response.try_into()?;

        Ok(records)
    }

    async fn add_record(&self, _reqwest: reqwest::Client, _input: &dns::Record) -> Result<()> {
        unimplemented!("Hetzner add_record not yet implemented")
    }

    async fn update_record(&self, _reqwest: reqwest::Client, _input: &dns::Record) -> Result<()> {
        unimplemented!("Hetzner update_record not yet implemented")
    }

    async fn delete_record(&self, _reqwest: reqwest::Client, _input: &dns::Record) -> Result<()> {
        unimplemented!("Hetzner delete_record not yet implemented")
    }
}
