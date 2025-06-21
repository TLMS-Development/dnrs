use std::{
    net::{AddrParseError, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use lum_libs::serde_json;
use lum_log::debug;
use thiserror::Error;

use crate::{
    Config,
    config::resolver::{IpResolver, IpResolverType},
};

#[derive(Debug)]
pub struct Ipv4ResolverConfig<'resolver> {
    pub ipv4_resolver: &'resolver IpResolver,
}

impl<'config> From<&'config Config> for Ipv4ResolverConfig<'config> {
    fn from(config: &'config Config) -> Self {
        Self {
            ipv4_resolver: &config.resolver.ipv4,
        }
    }
}

#[derive(Debug)]
pub struct Ipv6ResolverConfig<'resolver> {
    pub ipv6_resolver: &'resolver IpResolver,
}

impl<'config> From<&'config Config> for Ipv6ResolverConfig<'config> {
    fn from(config: &'config Config) -> Self {
        Self {
            ipv6_resolver: &config.resolver.ipv6,
        }
    }
}

#[derive(Debug, Error)]
pub enum JsonParseError {
    #[error("Could not parse JSON response: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("JSON path was empty")]
    EmptyPath,

    #[error("JSON path {0} not found at {1}")]
    PathNotFound(String, String),

    #[error("JSON value is not a string")]
    NotAString(serde_json::Value),
}

fn parse_json_response(response: &str, path: &str) -> Result<String, JsonParseError> {
    let path_parts = path.split('.').collect::<Vec<&str>>();
    if path_parts.is_empty() {
        return Err(JsonParseError::EmptyPath);
    }

    let json: serde_json::Value = serde_json::from_str(response)?;
    let mut current_json = &json;
    for part in path_parts {
        if let Some(next_json) = current_json.get(part) {
            current_json = next_json;
        } else {
            return Err(JsonParseError::PathNotFound(
                path.to_string(),
                part.to_string(),
            ));
        }
    }

    let value = match current_json.as_str() {
        Some(value) => value,
        None => return Err(JsonParseError::NotAString(current_json.clone())),
    };

    Ok(value.to_string())
}

#[derive(Debug, Error)]
pub enum IpResolverError {
    #[error("Error while sending HTTP request: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Error while parsing JSON response: {0}")]
    JsonParse(#[from] JsonParseError),

    #[error("Invalid IP address format: {0}")]
    InvalidIpFormat(#[from] AddrParseError),
}
pub async fn resolve_ipv4<'resolver>(
    config: &Ipv4ResolverConfig<'resolver>,
    reqwest: &reqwest::Client,
) -> Result<Ipv4Addr, IpResolverError> {
    debug!(
        "Resolving IPv4 address using resolver: {:?}",
        config.ipv4_resolver
    );

    let response = reqwest.get(&config.ipv4_resolver.url).send().await?;
    let body = response.text().await?.trim().to_string();

    let ip = match &config.ipv4_resolver.type_ {
        IpResolverType::Raw => body,
        IpResolverType::JSON(path) => parse_json_response(&body, path)?,
    };

    let ipv4_addr = Ipv4Addr::from_str(&ip)?;
    Ok(ipv4_addr)
}

pub async fn resolve_ipv6<'resolver>(
    config: &Ipv6ResolverConfig<'resolver>,
    reqwest: &reqwest::Client,
) -> Result<Ipv6Addr, IpResolverError> {
    debug!(
        "Resolving IPv6 address using resolver: {:?}",
        config.ipv6_resolver
    );

    let response = reqwest.get(&config.ipv6_resolver.url).send().await?;
    let body = response.text().await?.trim().to_string();

    let ip = match &config.ipv6_resolver.type_ {
        IpResolverType::Raw => body,
        IpResolverType::JSON(path) => parse_json_response(&body, path)?,
    };

    let ipv6_addr = Ipv6Addr::from_str(&ip)?;
    Ok(ipv6_addr)
}
