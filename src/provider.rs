use anyhow::Result;
use async_trait::async_trait;

use crate::types::dns::Record;

pub mod hetzner;
pub mod netcup;
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

/// Trait for DNS providers.
///
/// This trait defines the interface for interacting with various DNS providers
/// (e.g., Nitrado, Hetzner, Netcup).
///
/// # Examples
///
/// ```
/// use dnrs::provider::{Provider, Feature};
///
/// // Example of checking feature support
/// fn check_features(provider: &dyn Provider) {
///     if provider.is_feature_supported(&Feature::AddRecord) {
///         println!("Provider {} supports adding records", provider.get_provider_name());
///     }
/// }
/// ```
#[async_trait]
pub trait Provider: Send + Sync {
    fn get_provider_name(&self) -> &'static str;
    fn get_supported_features(&self) -> Vec<Feature>;
    fn is_feature_supported(&self, feature: &Feature) -> bool {
        self.get_supported_features().contains(feature)
    }

    async fn get_records(
        &self,
        reqwest: reqwest::Client,
        input: &GetRecordsInput,
    ) -> Result<Vec<Record>> {
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
    ) -> Result<Vec<Record>>;

    async fn add_record(&self, reqwest: reqwest::Client, record: &Record) -> Result<()>;
    async fn update_record(&self, reqwest: reqwest::Client, record: &Record) -> Result<()>;
    async fn delete_record(&self, reqwest: reqwest::Client, record: &Record) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::dns::RecordValue;
    use std::net::Ipv4Addr;

    struct MockProvider {
        name: &'static str,
        records: Vec<Record>,
    }

    #[async_trait]
    impl Provider for MockProvider {
        fn get_provider_name(&self) -> &'static str {
            self.name
        }

        fn get_supported_features(&self) -> Vec<Feature> {
            vec![Feature::GetRecords, Feature::GetAllRecords]
        }

        async fn get_all_records(
            &self,
            _reqwest: reqwest::Client,
            _input: &GetAllRecordsInput,
        ) -> Result<Vec<Record>> {
            Ok(self.records.clone())
        }

        async fn add_record(&self, _reqwest: reqwest::Client, _record: &Record) -> Result<()> {
            unimplemented!()
        }

        async fn update_record(&self, _reqwest: reqwest::Client, _record: &Record) -> Result<()> {
            unimplemented!()
        }

        async fn delete_record(&self, _reqwest: reqwest::Client, _record: &Record) -> Result<()> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_provider_generic_get_records() {
        let records = vec![
            Record {
                domain: "a.example.com".to_string(),
                value: RecordValue::A(Ipv4Addr::new(1, 1, 1, 1)),
                ttl: None,
            },
            Record {
                domain: "b.example.com".to_string(),
                value: RecordValue::A(Ipv4Addr::new(2, 2, 2, 2)),
                ttl: None,
            },
            Record {
                domain: "c.example.com".to_string(),
                value: RecordValue::A(Ipv4Addr::new(3, 3, 3, 3)),
                ttl: None,
            },
        ];

        let provider = MockProvider {
            name: "Mock",
            records,
        };

        let reqwest = reqwest::Client::new();
        let input = GetRecordsInput {
            domain: "example.com",
            subdomains: vec!["a.example.com", "c.example.com"],
        };

        let filtered = provider.get_records(reqwest, &input).await.unwrap();

        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].domain, "a.example.com");
        assert_eq!(filtered[1].domain, "c.example.com");
    }

    #[test]
    fn test_provider_is_feature_supported() {
        let provider = MockProvider {
            name: "Mock",
            records: vec![],
        };

        assert!(provider.is_feature_supported(&Feature::GetRecords));
        assert!(provider.is_feature_supported(&Feature::GetAllRecords));
        assert!(!provider.is_feature_supported(&Feature::AddRecord));
    }
}
