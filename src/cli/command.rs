use std::marker::PhantomData;

use clap::{Parser, Subcommand as ClapSubcommand};
use lum_log::{error, info};
use thiserror::Error;

use crate::{
    Config,
    cli::ExecutableCommand,
    resolver::{self, IpResolverError, Ipv4ResolverConfig, Ipv6ResolverConfig},
};

#[derive(Debug, ClapSubcommand)]
#[command(version, about, long_about = None, propagate_version = true)]
pub enum Subcommand {
    /// Test
    Test,
}

#[derive(Debug)]
pub struct Input<'config> {
    pub config: &'config Config,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to resolve IPv4 and IPv6 addresses: {0}; {1}")]
    ResolveIp(IpResolverError, IpResolverError),
}

/// dnrs
#[derive(Debug, Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct Command<'command> {
    #[clap(skip)]
    _phantom: PhantomData<&'command ()>,
    /*
    TODO: Implement when supported by lum_log
    /// Show verbose output
    #[clap(short, long, default_value = "false")]
    pub verbose: bool,
    */
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,
}

impl<'command> ExecutableCommand<'command> for Command<'command> {
    type I = Input<'command>;
    type R = Result<(), Error>;

    async fn execute(&self, input: &'command Self::I) -> Self::R {
        let config = input.config;
        let reqwest = reqwest::Client::new();

        let ipv4_resolver_config = Ipv4ResolverConfig::from(config);
        let ipv4 = resolver::resolve_ipv4(&ipv4_resolver_config, &reqwest).await;

        let ipv6_resolver_config = Ipv6ResolverConfig::from(config);
        let ipv6 = resolver::resolve_ipv6(&ipv6_resolver_config, &reqwest).await;

        match (ipv4, ipv6) {
            (Ok(ipv4), Ok(ipv6)) => {
                info!("Successfully resolved IPv4 address: {}", ipv4);
                info!("Successfully resolved IPv6 address: {}", ipv6);
            }
            (Ok(ipv4), Err(ipv6_err)) => {
                info!("Successfully resolved IPv4 address: {}", ipv4);
                error!(
                    "Failed to resolve IPv6 address: {}. Still proceeding with IPv4 address update.",
                    ipv6_err
                );
            }
            (Err(ipv4_err), Ok(ipv6)) => {
                info!("Successfully resolved IPv6 address: {}", ipv6);
                error!(
                    "Failed to resolve IPv4 address: {}. Still proceeding with IPv6 address update.",
                    ipv4_err
                );
            }
            _ => {}
        }

        Ok(())
    }
}
