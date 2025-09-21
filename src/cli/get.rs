use std::marker::PhantomData;

use clap::{Args, Parser};
use thiserror::Error;

use crate::{
    Config,
    cli::ExecutableCommand,
    config::provider::Provider as ProviderConfig,
    provider::{GetAllRecordsInput, GetRecordsInput, Provider, nitrado::NitradoProvider},
};

#[derive(Debug)]
pub struct Input<'config> {
    pub config: &'config Config,
    pub reqwest: reqwest::Client,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("The given provider is not configured: {0}")]
    ProviderNotConfigured(String),

    #[error("Provider error: {0}")]
    ProviderError(#[from] anyhow::Error),
}

//TODO: Fix order of usage message (provider should come first)
#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct SubdomainArgs {
    /// Subdomains to get records for
    subdomains: Vec<String>,

    /// Get all records
    #[clap(short, long, default_value = "false")]
    pub all: bool,
}

/// Get one or more DNS records from a provider
#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct Command<'command> {
    #[clap(skip)]
    _phantom: PhantomData<&'command ()>,

    /// Name of the provider to get records from
    provider: String,

    /// Domain to get records for
    domain: String,

    #[command(flatten)]
    subdomain_args: SubdomainArgs,
}

//TODO: Move
fn get_provider<'config>(
    name: &str,
    config: &'config Config,
) -> Option<Box<dyn Provider + 'config>> {
    for provider_file_config in config.providers.iter() {
        match &provider_file_config.provider {
            ProviderConfig::Nitrado(nitrado_config) => {
                if name == nitrado_config.name {
                    return Some(Box::new(NitradoProvider::new(nitrado_config)));
                }
            }
        }
    }

    None
}

impl<'command> ExecutableCommand<'command> for Command<'command> {
    type I = Input<'command>;
    type R = Result<(), Error>;

    async fn execute(&self, input: &'command Self::I) -> Self::R {
        let config = input.config;
        let provider_name = self.provider.as_str();

        let provider = match get_provider(provider_name, config) {
            Some(p) => p,
            None => return Err(Error::ProviderNotConfigured(provider_name.to_string())),
        };

        let reqwest = reqwest::Client::new();

        let results = if self.subdomain_args.all {
            let input = GetAllRecordsInput {
                domain: self.domain.as_str(),
            };

            provider.get_all_records(reqwest, &input).await
        } else {
            let input = GetRecordsInput {
                domain: self.domain.as_str(),
                subdomains: self
                    .subdomain_args
                    .subdomains
                    .iter()
                    .map(|s| s.as_str())
                    .collect(),
            };

            provider.get_records(reqwest, &input).await
        };

        let records = match results {
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(e.into());
            }
            Ok(records) => records,
        };

        println!("Records: {:#?}", records);
        Ok(())
    }
}
