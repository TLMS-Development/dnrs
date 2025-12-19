use std::{
    net::{AddrParseError, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

use lum_libs::serde_json;
use lum_log::debug;
use thiserror::Error;

use crate::{
    Config,
    config::{
        dns::{AutomaticRecordConfig, ResolveType},
        resolver::{IpResolver, IpResolverType},
    },
    types::dns::{Record, RecordValue},
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

/// Parses a JSON response and returns the value at the specified path.
///
/// The path uses dot notation to traverse nested objects (e.g., "data.ip").
///
/// # Examples
///
/// ```
/// use dnrs::resolver::parse_json_response;
///
/// let json = r#"{"ip": "1.2.3.4"}"#;
/// let result = parse_json_response(json, "ip").unwrap();
/// assert_eq!(result, "1.2.3.4");
///
/// let nested_json = r#"{"data": {"ip": "1.2.3.4"}}"#;
/// let result = parse_json_response(nested_json, "data.ip").unwrap();
/// assert_eq!(result, "1.2.3.4");
/// ```
///
/// # Errors
///
/// Returns a [`JsonParseError`] if:
/// - The JSON is invalid.
/// - The path is empty.
/// - The path does not exist in the JSON.
/// - The value at the path is not a string.
pub fn parse_json_response(response: &str, path: &str) -> Result<String, JsonParseError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_response_simple() {
        let response = r#"{"ip": "1.2.3.4"}"#;
        let path = "ip";
        let result = parse_json_response(response, path).unwrap();
        assert_eq!(result, "1.2.3.4");
    }

    #[test]
    fn test_parse_json_response_nested() {
        let response = r#"{"data": {"ip": "1.2.3.4"}}"#;
        let path = "data.ip";
        let result = parse_json_response(response, path).unwrap();
        assert_eq!(result, "1.2.3.4");
    }

    #[test]
    fn test_parse_json_response_deeply_nested() {
        let response = r#"{"a": {"b": {"c": "val"}}}"#;
        let path = "a.b.c";
        let result = parse_json_response(response, path).unwrap();
        assert_eq!(result, "val");
    }

    #[test]
    fn test_parse_json_response_empty_path() {
        let response = r#"{"ip": "1.2.3.4"}"#;
        let path = "";
        let result = parse_json_response(response, path);
        assert!(matches!(result, Err(JsonParseError::PathNotFound(_, _))));
        // Note: split('.').collect() on empty string results in [""]
    }

    #[test]
    fn test_parse_json_response_path_not_found() {
        let response = r#"{"ip": "1.2.3.4"}"#;
        let path = "nonexistent";
        let result = parse_json_response(response, path);
        assert!(matches!(result, Err(JsonParseError::PathNotFound(_, _))));
    }

    #[test]
    fn test_parse_json_response_not_a_string() {
        let response = r#"{"ip": 1234}"#;
        let path = "ip";
        let result = parse_json_response(response, path);
        assert!(matches!(result, Err(JsonParseError::NotAString(_))));
    }

    #[test]
    fn test_parse_json_response_invalid_json() {
        let response = r#"{"ip": "1.2.3.4"#;
        let path = "ip";
        let result = parse_json_response(response, path);
        assert!(matches!(result, Err(JsonParseError::SerdeJson(_))));
    }
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
async fn resolve_ip_internal<T>(
    resolver: &IpResolver,
    reqwest: &reqwest::Client,
) -> Result<T, IpResolverError>
where
    T: FromStr,
    IpResolverError: From<<T as FromStr>::Err>,
{
    debug!("Resolving address using resolver: {:?}", resolver);

    let response = reqwest.get(&resolver.url).send().await?;
    let body = response.text().await?.trim().to_string();

    let ip = match &resolver.type_ {
        IpResolverType::Raw => body,
        IpResolverType::JSON(path) => parse_json_response(&body, path)?,
    };

    let addr = T::from_str(&ip)?;
    Ok(addr)
}

pub async fn resolve_ipv4<'resolver>(
    config: &Ipv4ResolverConfig<'resolver>,
    reqwest: &reqwest::Client,
) -> Result<Ipv4Addr, IpResolverError> {
    resolve_ip_internal(config.ipv4_resolver, reqwest).await
}

pub async fn resolve_ipv6<'resolver>(
    config: &Ipv6ResolverConfig<'resolver>,
    reqwest: &reqwest::Client,
) -> Result<Ipv6Addr, IpResolverError> {
    resolve_ip_internal(config.ipv6_resolver, reqwest).await
}

pub async fn resolve_to_record(
    config: &Config,
    reqwest: &reqwest::Client,
    automatic_record_config: &AutomaticRecordConfig,
) -> Result<Record, IpResolverError> {
    let domain = automatic_record_config.domain.clone();
    let ttl = automatic_record_config.ttl;

    match automatic_record_config.resolve_type {
        ResolveType::IPv4 => {
            let ipv4_resolver_config = Ipv4ResolverConfig::from(config);
            let ipv4 = resolve_ipv4(&ipv4_resolver_config, reqwest).await?;
            Ok(Record {
                domain,
                value: RecordValue::A(ipv4),
                ttl,
            })
        }
        ResolveType::IPv6 => {
            let ipv6_resolver_config = Ipv6ResolverConfig::from(config);
            let ipv6 = resolve_ipv6(&ipv6_resolver_config, reqwest).await?;
            Ok(Record {
                domain,
                value: RecordValue::AAAA(ipv6),
                ttl,
            })
        }
    }
}
