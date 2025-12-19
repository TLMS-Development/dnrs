use lum_libs::serde::{Deserialize, Serialize};

/// Represents the type of an IP resolver.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum IpResolverType {
    /// The response is a raw IP address string.
    Raw,
    /// The response is a JSON object, and the IP address is at the specified path.
    JSON(String),
}

/// Configuration for an IP resolver.
///
/// # Examples
///
/// ```
/// use dnrs::config::resolver::{IpResolver, IpResolverType};
///
/// let resolver = IpResolver {
///     url: "https://ip.cancom.io".to_string(),
///     type_: IpResolverType::Raw,
/// };
///
/// assert_eq!(resolver.url, "https://ip.cancom.io");
/// assert!(matches!(resolver.type_, IpResolverType::Raw));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct IpResolver {
    pub url: String,

    #[serde(rename = "type")]
    pub type_: IpResolverType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Config {
    pub ipv4: IpResolver,
    pub ipv6: IpResolver,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ipv4: IpResolver {
                url: "https://ip.cancom.io".to_string(),
                type_: IpResolverType::Raw,
            },
            ipv6: IpResolver {
                url: "https://ipv6.cancom.io".to_string(),
                type_: IpResolverType::Raw,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml_ng;

    #[test]
    fn test_deserialize_ip_resolver_raw() {
        let yaml = r#"
            url: "https://example.com"
            type: Raw
        "#;
        let resolver: IpResolver = serde_yaml_ng::from_str(yaml).unwrap();
        assert_eq!(resolver.url, "https://example.com");
        match resolver.type_ {
            IpResolverType::Raw => (),
            _ => panic!("Expected Raw type"),
        }
    }

    #[test]
    fn test_deserialize_ip_resolver_json() {
        let yaml = r#"
            url: "https://example.com"
            type: !JSON "data.ip"
        "#;
        let resolver: IpResolver = serde_yaml_ng::from_str(yaml).unwrap();
        assert_eq!(resolver.url, "https://example.com");
        match resolver.type_ {
            IpResolverType::JSON(path) => assert_eq!(path, "data.ip"),
            _ => panic!("Expected JSON type"),
        }
    }

    #[test]
    fn test_deserialize_config_default() {
        let config = Config::default();
        let yaml = serde_yaml_ng::to_string(&config).unwrap();
        let deserialized: Config = serde_yaml_ng::from_str(&yaml).unwrap();
        assert_eq!(deserialized.ipv4.url, config.ipv4.url);
        assert_eq!(deserialized.ipv6.url, config.ipv6.url);
    }
}
