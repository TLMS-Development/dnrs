//TODO: No anyhow
use anyhow::Result;
use lum_config::MergeFrom;
use lum_libs::serde::{Deserialize, Serialize};
use lum_log::{debug, error, info};
use std::{fs, path::Path};

use crate::{
    config::provider::Provider,
    provider::{hetzner, netcup, nitrado},
};

pub mod dns;
pub mod provider;
pub mod resolver;

/// Configuration for the dnrs application.
///
/// This struct holds all the configuration required to run the application,
/// including IP resolver settings, provider credentials, and DNS record definitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct Config {
    pub resolver: resolver::Config,
    pub providers: Vec<Provider>,
    pub dns: Vec<dns::Type>,
}

impl Config {
    pub fn load_from_directory(config_dir: impl AsRef<Path>) -> Result<Self> {
        let config_dir = config_dir.as_ref();
        let resolver = Self::load_resolver_config(config_dir)?;
        let providers = Self::load_provider_configs(&config_dir.join("providers"))?;
        let dns = Self::load_dns_configs(&config_dir.join("dns"))?;

        let loaded_config = Config {
            resolver,
            providers,
            dns,
        };

        let default_config = Config::default();
        Ok(default_config.merge_from(loaded_config))
    }

    fn load_resolver_config(config_dir: impl AsRef<Path>) -> Result<resolver::Config> {
        let resolver_path = config_dir.as_ref().join("resolver.yaml");

        //TODO: Fail with error if resolver config is missing
        if resolver_path.exists() {
            let content = fs::read_to_string(resolver_path)?;
            Ok(serde_yaml_ng::from_str(&content)?)
        } else {
            Ok(resolver::Config::default())
        }
    }

    fn load_provider_configs(providers_dir: impl AsRef<Path>) -> Result<Vec<Provider>> {
        let providers_dir = providers_dir.as_ref();
        //TODO: Fail with error if providers config is missing
        if !providers_dir.exists() {
            info!(
                "Providers directory {:?} does not exist, using defaults",
                providers_dir
            );
            return Ok(vec![
                Provider::Nitrado(nitrado::Config::default()),
                Provider::Hetzner(hetzner::Config::default()),
                Provider::Netcup(netcup::Config::default()),
            ]);
        }

        let mut configs = Vec::new();
        for entry in fs::read_dir(providers_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path
                .extension()
                .map_or(false, |ext| ext == "yaml" || ext == "yml")
            {
                let content = fs::read_to_string(&path)?;

                let file_stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                //TODO: Hardcoded config file names. Detect type differently?
                match file_stem {
                    "hetzner" => {
                        let config: hetzner::Config = serde_yaml_ng::from_str(&content)?;
                        configs.push(Provider::Hetzner(config));
                        debug!("Loaded Hetzner provider config from {:?}", path);
                    }
                    "nitrado" => {
                        let config: nitrado::Config = serde_yaml_ng::from_str(&content)?;
                        configs.push(Provider::Nitrado(config));
                        debug!("Loaded Nitrado provider config from {:?}", path);
                    }
                    "netcup" => {
                        let config: netcup::Config = serde_yaml_ng::from_str(&content)?;
                        configs.push(Provider::Netcup(config));
                        debug!("Loaded Netcup provider config from {:?}", path);
                    }
                    _ => {
                        error!("Unknown provider config file: {}", path.display());
                    }
                }
            }
        }

        if configs.is_empty() {
            info!("No provider configs found, using defaults");
            configs.push(Provider::Nitrado(nitrado::Config::default()));
            configs.push(Provider::Hetzner(hetzner::Config::default()));
        }

        Ok(configs)
    }

    fn load_dns_configs(dns_dir: impl AsRef<Path>) -> Result<Vec<dns::Type>> {
        let dns_dir = dns_dir.as_ref();

        //TODO: Fail with error if dns config is missing
        if !dns_dir.exists() {
            info!(
                "DNS directory {:?} does not exist, using empty configs",
                dns_dir
            );
            return Ok(vec![]);
        }

        let mut configs = Vec::new();
        for entry in fs::read_dir(dns_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path
                .extension()
                .map_or(false, |ext| ext == "yaml" || ext == "yml")
            {
                let content = fs::read_to_string(&path)?;

                let file_stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                //TODO: Hardcoded config file names. Detect type differently?
                if file_stem.contains("hetzner") {
                    let config: hetzner::DnsConfig = serde_yaml_ng::from_str(&content)?;
                    configs.push(dns::Type::Hetzner(config));
                    debug!("Loaded Hetzner DNS config from {:?}", path);
                } else if file_stem.contains("nitrado") {
                    let config: nitrado::DnsConfig = serde_yaml_ng::from_str(&content)?;
                    configs.push(dns::Type::Nitrado(config));
                    debug!("Loaded Nitrado DNS config from {:?}", path);
                } else if file_stem.contains("netcup") {
                    let config: netcup::DnsConfig = serde_yaml_ng::from_str(&content)?;
                    configs.push(dns::Type::Netcup(config));
                    debug!("Loaded Netcup DNS config from {:?}", path);
                } else {
                    error!(
                        "Cannot determine DNS config type for file: {}",
                        path.display()
                    );
                }
            }
        }

        debug!("Loaded {} DNS configurations", configs.len());
        Ok(configs)
    }

    pub fn create_example_structure(config_dir: impl AsRef<Path>) -> Result<()> {
        let config_dir = config_dir.as_ref();

        fs::create_dir_all(config_dir.join("providers"))?;
        fs::create_dir_all(config_dir.join("dns"))?;

        let resolver_config = resolver::Config::default();
        let resolver_yaml = serde_yaml_ng::to_string(&resolver_config)?;
        fs::write(config_dir.join("resolver.yaml"), resolver_yaml)?;

        let hetzner_config = hetzner::Config::default();
        let hetzner_yaml = serde_yaml_ng::to_string(&hetzner_config)?;
        fs::write(config_dir.join("providers/hetzner.yaml"), hetzner_yaml)?;

        let nitrado_config = nitrado::Config::default();
        let nitrado_yaml = serde_yaml_ng::to_string(&nitrado_config)?;
        fs::write(config_dir.join("providers/nitrado.yaml"), nitrado_yaml)?;

        let netcup_config = netcup::Config::default();
        let netcup_yaml = serde_yaml_ng::to_string(&netcup_config)?;
        fs::write(config_dir.join("providers/netcup.yaml"), netcup_yaml)?;

        let hetzner_dns_config = hetzner::DnsConfig::default();
        let hetzner_dns_yaml = serde_yaml_ng::to_string(&hetzner_dns_config)?;
        fs::write(
            config_dir.join("dns/hetzner-domains.yaml"),
            hetzner_dns_yaml,
        )?;

        let nitrado_dns_config = nitrado::DnsConfig::default();
        let nitrado_dns_yaml = serde_yaml_ng::to_string(&nitrado_dns_config)?;
        fs::write(
            config_dir.join("dns/nitrado-domains.yaml"),
            nitrado_dns_yaml,
        )?;

        let netcup_dns_config = netcup::DnsConfig::default();
        let netcup_dns_yaml = serde_yaml_ng::to_string(&netcup_dns_config)?;
        fs::write(config_dir.join("dns/netcup-domains.yaml"), netcup_dns_yaml)?;

        info!("Created example config structure in {:?}", config_dir);
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolver: resolver::Config::default(),
            providers: vec![
                Provider::Nitrado(nitrado::Config::default()),
                Provider::Hetzner(hetzner::Config::default()),
                Provider::Netcup(netcup::Config::default()),
            ],
            dns: vec![
                dns::Type::Nitrado(nitrado::DnsConfig::default()),
                dns::Type::Hetzner(hetzner::DnsConfig::default()),
                dns::Type::Netcup(netcup::DnsConfig::default()),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_merge_from_empty() {
        let default_config = Config::default();
        let other = Config {
            resolver: resolver::Config {
                ipv4: resolver::IpResolver {
                    url: "https://new.ipv4.com".to_string(),
                    type_: resolver::IpResolverType::Raw,
                },
                ipv6: resolver::IpResolver {
                    url: "https://new.ipv6.com".to_string(),
                    type_: resolver::IpResolverType::Raw,
                },
            },
            providers: vec![],
            dns: vec![],
        };

        let merged = default_config.clone().merge_from(other.clone());

        assert_eq!(merged.resolver.ipv4.url, "https://new.ipv4.com");
        assert_eq!(merged.providers.len(), default_config.providers.len());
        assert_eq!(merged.dns.len(), default_config.dns.len());
    }

    #[test]
    fn test_config_merge_from_not_empty() {
        let default_config = Config::default();
        let other = Config {
            resolver: resolver::Config::default(),
            providers: vec![Provider::Nitrado(
                nitrado::Config {
                    name: "OtherNitrado".to_string(),
                    ..Default::default()
                },
            )],
            dns: vec![],
        };

        let merged = default_config.clone().merge_from(other.clone());

        assert_eq!(merged.providers.len(), 1);
        if let Provider::Nitrado(config) = &merged.providers[0] {
            assert_eq!(config.name, "OtherNitrado");
        } else {
            panic!("Expected Nitrado provider");
        }
        assert_eq!(merged.dns.len(), default_config.dns.len());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(!config.providers.is_empty());
        assert!(!config.dns.is_empty());
    }

    #[test]
    fn test_load_from_directory() {
        let temp_dir = std::env::temp_dir().join("dnrs_load_test");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(&temp_dir).unwrap();

        Config::create_example_structure(&temp_dir).unwrap();

        let config = Config::load_from_directory(&temp_dir).unwrap();
        assert_eq!(config.providers.len(), 3);
        assert_eq!(config.dns.len(), 3);

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_load_from_directory_missing() {
        let temp_dir = std::env::temp_dir().join("dnrs_missing_test");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }

        let config = Config::load_from_directory(&temp_dir).unwrap();
        assert!(!config.providers.is_empty());
    }

    #[test]
    fn test_load_from_directory_invalid_yaml() {
        let temp_dir = std::env::temp_dir().join("dnrs_invalid_yaml_test");
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).unwrap();
        }
        fs::create_dir_all(temp_dir.join("providers")).unwrap();

        fs::write(temp_dir.join("providers/nitrado.yaml"), "invalid: yaml: :").unwrap();

        let result = Config::load_from_directory(&temp_dir);
        assert!(result.is_err());

        fs::remove_dir_all(&temp_dir).unwrap();
    }
}

impl MergeFrom<Self> for Config {
    /// Merges another configuration into this one.
    ///
    /// Values from `other` will override values in `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dnrs::Config;
    /// use lum_config::MergeFrom;
    ///
    /// let mut config = Config::default();
    /// let mut other = Config::default();
    /// other.resolver.ipv4.url = "https://example.com".to_string();
    ///
    /// let merged = config.merge_from(other);
    /// assert_eq!(merged.resolver.ipv4.url, "https://example.com");
    /// ```
    fn merge_from(self, other: Self) -> Self {
        Self {
            resolver: other.resolver,
            providers: if !other.providers.is_empty() {
                other.providers
            } else {
                self.providers
            },
            dns: if !other.dns.is_empty() {
                other.dns
            } else {
                self.dns
            },
        }
    }
}
