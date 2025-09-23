use std::marker::PhantomData;

use clap::{Args, Parser};
use lum_log::{error, info};
use thiserror::Error;

use crate::{
    Config,
    cli::ExecutableCommand,
    config::provider::Provider as ProviderConfig,
    provider::{
        GetAllRecordsInput, GetRecordsInput, Provider, hetzner::HetznerProvider,
        nitrado::NitradoProvider,
    },
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

#[derive(Debug, Args)]
pub struct SubdomainArgs {
    /// Subdomains to get records for
    #[clap(display_order = 3)]
    subdomains: Vec<String>,

    /// Get all records
    #[clap(short, long, default_value = "false", display_order = 3)]
    pub all: bool,
}

/// Get one or more DNS records from a provider
#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct Command<'command> {
    #[clap(skip)]
    _phantom: PhantomData<&'command ()>,

    /// Name of the provider to get records from
    #[clap(display_order = 1)]
    provider: String,

    /// Domain to get records for
    #[clap(display_order = 2)]
    domain: String,

    #[command(flatten)]
    subdomain_args: SubdomainArgs,
}

//TODO: Move
fn get_provider<'config>(
    name: &str,
    config: &'config Config,
) -> Option<Box<dyn Provider + 'config>> {
    for provider in config.providers.iter() {
        match provider {
            ProviderConfig::Nitrado(nitrado_config) => {
                if name == nitrado_config.name {
                    return Some(Box::new(NitradoProvider::new(nitrado_config)));
                }
            }
            ProviderConfig::Hetzner(hetzner_config) => {
                if name == hetzner_config.name {
                    return Some(Box::new(HetznerProvider::new(hetzner_config)));
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
        if self.subdomain_args.all && !self.subdomain_args.subdomains.is_empty() {
            error!("Cannot specify both --all and specific subdomains");
            return Err(Error::ProviderError(anyhow::anyhow!(
                "Cannot specify both --all and specific subdomains"
            )));
        }

        if !self.subdomain_args.all && self.subdomain_args.subdomains.is_empty() {
            error!("Must specify either --all or specific subdomains");
            return Err(Error::ProviderError(anyhow::anyhow!(
                "Must specify either --all or specific subdomains"
            )));
        }

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
                error!("Error: {}", e);
                return Err(e.into());
            }
            Ok(records) => records,
        };

        info!("Records: {:#?}", records);
        Ok(())
    }
}
