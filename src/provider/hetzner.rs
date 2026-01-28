use async_trait::async_trait;
use lum_libs::serde_json;
use reqwest::header::HeaderMap;
use thiserror::Error;

use crate::{
    provider::{Feature, GetAllRecordsInput, Provider, ProviderError},
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

    async fn get_zone_id(&self, reqwest: reqwest::Client, domain: &str) -> Result<String, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Auth-API-Token",
            self.provider_config.api_key.parse().expect(
                "Invalid Hetzner API key: contains characters that are not allowed in HTTP headers",
            ),
        );

        let url = format!("{}/zones", self.provider_config.api_base_url);
        let response = reqwest.get(&url).headers(headers).send().await?;

        if !response.status().is_success() {
            return Err(Error::Unsuccessful(response.status().as_u16(), response));
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
            None => Err(Error::DomainNotFound(domain.to_string())),
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

    #[error("Record conversion error: {0}")]
    RecordConversion(#[from] TryFromRecordError),
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

    async fn get_all_records(
        &self,
        reqwest: reqwest::Client,
        input: &GetAllRecordsInput,
    ) -> Result<Vec<dns::Record>, ProviderError> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Auth-API-Token",
            self.provider_config.api_key.parse().expect(
                "Invalid Hetzner API key: contains characters that are not allowed in HTTP headers",
            ),
        );

        let domain = &input.domain;
        let zone_id = self.get_zone_id(reqwest.clone(), domain).await?;

        let url = format!(
            "{}/records?zone_id={}",
            self.provider_config.api_base_url, zone_id
        );

        let response = reqwest
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(Error::from)?;

        if !response.status().is_success() {
            return Err(Error::Unsuccessful(response.status().as_u16(), response).into());
        }

        let text = response.text().await.map_err(Error::from)?;
        let response: GetRecordsResponse = serde_json::from_str(&text).map_err(Error::from)?;
        let records: Vec<dns::Record> = response.try_into().map_err(Error::from)?;

        Ok(records)
    }

    async fn add_record(
        &self,
        _reqwest: reqwest::Client,
        _input: &dns::Record,
    ) -> Result<(), ProviderError> {
        unimplemented!("Hetzner add_record not yet implemented")
    }

    async fn update_record(
        &self,
        _reqwest: reqwest::Client,
        _input: &dns::Record,
    ) -> Result<(), ProviderError> {
        unimplemented!("Hetzner update_record not yet implemented")
    }

    async fn delete_record(
        &self,
        _reqwest: reqwest::Client,
        _input: &dns::Record,
    ) -> Result<(), ProviderError> {
        unimplemented!("Hetzner delete_record not yet implemented")
    }
}
