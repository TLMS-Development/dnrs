use anyhow::Result;
use async_trait::async_trait;

use crate::types::dns::Record;

pub mod nitrado;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Feature {
    GetRecords,
    GetAllRecords,
    AddRecord,
    UpdateRecord,
    DeleteRecord,
}

pub struct GetRecordsInput<'input> {
    pub domain: &'input str,
    pub subdomains: Vec<&'input str>,
}

pub struct GetAllRecordsInput<'input> {
    pub domain: &'input str,
}

impl<'input> From<GetRecordsInput<'input>> for GetAllRecordsInput<'input> {
    fn from(input: GetRecordsInput<'input>) -> Self {
        GetAllRecordsInput {
            domain: input.domain,
        }
    }
}

impl<'input> From<&'input GetRecordsInput<'input>> for GetAllRecordsInput<'input> {
    fn from(input: &'input GetRecordsInput<'input>) -> Self {
        GetAllRecordsInput {
            domain: input.domain,
        }
    }
}

#[async_trait]
pub trait Provider {
    fn get_provider_name(&self) -> &'static str;
    fn get_supported_features(&self) -> Vec<Feature>;
    fn is_feature_supported(&self, feature: &Feature) -> bool {
        self.get_supported_features().contains(feature)
    }

    async fn get_records(
        &self,
        reqwest: reqwest::Client,
        input: &GetRecordsInput,
    ) -> Result<Vec<Record>>;

    async fn get_all_records(
        &self,
        reqwest: reqwest::Client,
        input: &GetAllRecordsInput,
    ) -> Result<Vec<Record>>;

    async fn add_record(&self, reqwest: reqwest::Client, record: &Record) -> Result<()>;
    async fn update_record(&self, reqwest: reqwest::Client, record: &Record) -> Result<()>;
    async fn delete_record(&self, reqwest: reqwest::Client, record: &Record) -> Result<()>;
}
