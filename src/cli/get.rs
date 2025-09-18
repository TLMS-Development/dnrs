use std::marker::PhantomData;

use clap::{Args, Parser};
use lum_log::config;
use thiserror::Error;

use crate::{
    Config,
    cli::ExecutableCommand,
    config::provider::Provider as ProviderConfig,
    provider::{Provider, nitrado::NitradoProvider},
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

        Ok(())
    }
}
