use anyhow::Result;
use async_trait::async_trait;
use lum_libs::serde_json;
use thiserror::Error;

use crate::{
    provider::{Feature, GetAllRecordsInput, Provider},
    types::dns::{self},
};

pub mod config;
pub mod model;

pub use config::{Config, DnsConfig, DomainConfig};
pub use model::{GetRecordsResponse, Record, TryFromRecordError};

pub struct NetcupProvider<'provider_config> {
    pub provider_config: &'provider_config Config,
}

impl<'provider_config> NetcupProvider<'provider_config> {
    pub fn new(provider_config: &'provider_config Config) -> NetcupProvider<'provider_config> {
        NetcupProvider { provider_config }
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

    #[error("Domain '{0}' not found in Netcup zones")]
    DomainNotFound(String),
}

#[async_trait]
impl Provider for NetcupProvider<'_> {
    fn get_provider_name(&self) -> &'static str {
        "Netcup"
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
        _reqwest: reqwest::Client,
        _input: &GetAllRecordsInput,
    ) -> Result<Vec<dns::Record>> {
        unimplemented!("Netcup get_all_records not yet implemented")
    }

    async fn add_record(&self, _reqwest: reqwest::Client, _input: &dns::Record) -> Result<()> {
        unimplemented!("Netcup add_record not yet implemented")
    }

    async fn update_record(&self, _reqwest: reqwest::Client, _input: &dns::Record) -> Result<()> {
        unimplemented!("Netcup update_record not yet implemented")
    }

    async fn delete_record(&self, _reqwest: reqwest::Client, _input: &dns::Record) -> Result<()> {
        unimplemented!("Netcup delete_record not yet implemented")
    }
}
