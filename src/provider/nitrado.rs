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
pub use model::{GetRecordsResponse, Record, RecordMode, TryFromRecordError};

pub struct NitradoProvider<'provider_config> {
    pub provider_config: &'provider_config Config,
}

impl<'provider_config> NitradoProvider<'provider_config> {
    pub fn new(provider_config: &'provider_config Config) -> NitradoProvider<'provider_config> {
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
        let records = records
            .into_iter()
            .filter(|record| input.subdomains.contains(&record.domain.as_str()))
            .collect();

        Ok(records)
    }

    async fn get_all_records(
        &self,
        reqwest: reqwest::Client,
        input: &GetAllRecordsInput,
    ) -> Result<Vec<dns::Record>> {
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
        let response: GetRecordsResponse = serde_json::from_str(&text)?;
        let records: Vec<dns::Record> = response.try_into()?;

        Ok(records)
    }

    async fn add_record(&self, _reqwest: reqwest::Client, _input: &dns::Record) -> Result<()> {
        unimplemented!()
    }

    async fn update_record(&self, _reqwest: reqwest::Client, _input: &dns::Record) -> Result<()> {
        unimplemented!()
    }

    async fn delete_record(&self, _reqwest: reqwest::Client, _input: &dns::Record) -> Result<()> {
        unimplemented!()
    }
}
