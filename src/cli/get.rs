use std::marker::PhantomData;

use clap::{Args, Parser};
use thiserror::Error;

use crate::{
    Config, cli::ExecutableCommand, config::provider::ProviderType,
    provider::nitrado::NitradoProvider,
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
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct DomainArgs {
    /// Domains to get information for
    domains: Vec<String>,

    /// Get all records
    #[clap(short, long, default_value = "false")]
    pub all: bool,
}

/// Update providers as defined in the configuration file
#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct Command<'command> {
    #[clap(skip)]
    _phantom: PhantomData<&'command ()>,

    /// Name of the provider to get information from
    provider: String,

    #[command(flatten)]
    domain_args: DomainArgs,
}

impl<'command> ExecutableCommand<'command> for Command<'command> {
    type I = Input<'command>;
    type R = Result<(), Error>;

    async fn execute(&self, input: &'command Self::I) -> Self::R {
        let config = input.config;
        let reqwest = reqwest::Client::new();

        let provider_config = config.providers.providers.iter().find(|p| match p {
            ProviderType::NitradoConfig(nitrado_config) => nitrado_config.name == self.provider,
        });

        if provider_config.is_none() {
            return Err(Error::ProviderNotConfigured(self.provider.clone()));
        }

        let provider_config = provider_config.unwrap();
        let auto_config = input.config.auto;
        match provider_config {
            ProviderType::NitradoConfig(nitrado_config) => {
                let nitrado_provider = NitradoProvider::new(provider_config, auto_config);
            }
            _ => unreachable!("Unexpected provider type"),
        }

        Ok(())
    }
}
