use anyhow::Result;
use async_trait::async_trait;

use crate::types::dns::Record;

pub mod nitrado;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Feature {
    GetRecord,
    GetRecords,
    AddRecord,
    UpdateRecord,
    DeleteRecord,
}

pub struct GetRecordInput {}
pub struct GetRecordsInput {
    pub domain: String,
}

#[async_trait]
pub trait Provider {
    fn get_provider_name(&self) -> &'static str;
    fn get_supported_features(&self) -> Vec<Feature>;
    fn is_feature_supported(&self, feature: &Feature) -> bool {
        Self::get_supported_features(&self).contains(feature)
    }

    async fn get_record(&self, reqwest: reqwest::Client, input: &GetRecordInput) -> Result<Record>;
    async fn get_records(
        &self,
        reqwest: reqwest::Client,
        input: &GetRecordsInput,
    ) -> Result<Vec<Record>>;
    async fn add_record(&self, reqwest: reqwest::Client, record: &Record) -> Result<()>;
    async fn update_record(&self, reqwest: reqwest::Client, record: &Record) -> Result<()>;
    async fn delete_record(&self, reqwest: reqwest::Client, record: &Record) -> Result<()>;
}
