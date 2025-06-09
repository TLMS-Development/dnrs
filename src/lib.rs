pub mod config;
pub mod logger;
pub mod resolver;

use std::time::Instant;

pub use config::{Config, EnvConfig, FileConfig, IpResolver, IpResolverType};
pub use logger::setup_logger;
use lum_log::{debug, info};
use thiserror::Error;

use crate::resolver::{IpResolverError, Ipv4ResolverConfig, Ipv6ResolverConfig};

pub const TOKEN: &str = "nope";

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Failed to resolve IPv4 address: {0}")]
    ResolveIpv4(IpResolverError),

    #[error("Failed to resolve IPv6 address: {0}")]
    ResolveIpv6(IpResolverError),
}

pub async fn run(config: Config) -> Result<(), RuntimeError> {
    let start = Instant::now();

    let reqwest = reqwest::Client::new();

    let ipv4_resolver_config = Ipv4ResolverConfig::from(&config);
    let ipv4 = match resolver::resolve_ipv4(&ipv4_resolver_config, &reqwest).await {
        Ok(ip) => ip,
        Err(e) => return Err(RuntimeError::ResolveIpv4(e)),
    };

    let ipv6_resolver_config = Ipv6ResolverConfig::from(&config);
    let ipv6 = match resolver::resolve_ipv6(&ipv6_resolver_config, &reqwest).await {
        Ok(ip) => ip,
        Err(e) => return Err(RuntimeError::ResolveIpv6(e)),
    };

    info!("Resolved IPv4 address: {}", ipv4);
    info!("Resolved IPv6 address: {}", ipv6);

    let elapsed = start.elapsed();
    debug!("Done in {}ms", elapsed.as_millis());

    Ok(())
}
