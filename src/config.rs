use anyhow::Result;
use lum_config::MergeFrom;
use lum_libs::serde::{Deserialize, Serialize};
use lum_log::{debug, error, info};
use std::path::Path;

pub mod dns;
pub mod provider;
pub mod resolver;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct Config {
    pub resolver: resolver::Config,
    pub providers: Vec<provider::Provider>,
    pub dns: Vec<dns::Type>,
}

impl Config {
    pub fn load_from_directory<P: AsRef<Path>>(config_dir: P) -> Result<Self> {
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

    fn load_resolver_config<P: AsRef<Path>>(config_dir: P) -> Result<resolver::Config> {
        let resolver_path = config_dir.as_ref().join("resolver.yaml");
        if resolver_path.exists() {
            let content = std::fs::read_to_string(resolver_path)?;
            Ok(serde_yaml_ng::from_str(&content)?)
        } else {
            Ok(resolver::Config::default())
        }
    }

    fn load_provider_configs<P: AsRef<Path>>(providers_dir: P) -> Result<Vec<provider::Provider>> {
        let mut configs = Vec::new();
        let providers_dir = providers_dir.as_ref();

        if !providers_dir.exists() {
            info!(
                "Providers directory {:?} does not exist, using defaults",
                providers_dir
            );
            return Ok(vec![
                provider::Provider::Nitrado(crate::provider::nitrado::Config::default()),
                provider::Provider::Hetzner(crate::provider::hetzner::Config::default()),
                provider::Provider::Netcup(crate::provider::netcup::Config::default()),
            ]);
        }

        for entry in std::fs::read_dir(providers_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path
                .extension()
                .map_or(false, |ext| ext == "yaml" || ext == "yml")
            {
                let content = std::fs::read_to_string(&path)?;

                let file_stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                match file_stem {
                    "hetzner" => {
                        let config: crate::provider::hetzner::Config =
                            serde_yaml_ng::from_str(&content)?;
                        configs.push(provider::Provider::Hetzner(config));
                        debug!("Loaded Hetzner provider config from {:?}", path);
                    }
                    "nitrado" => {
                        let config: crate::provider::nitrado::Config =
                            serde_yaml_ng::from_str(&content)?;
                        configs.push(provider::Provider::Nitrado(config));
                        debug!("Loaded Nitrado provider config from {:?}", path);
                    }
                    "netcup" => {
                        let config: crate::provider::netcup::Config =
                            serde_yaml_ng::from_str(&content)?;
                        configs.push(provider::Provider::Netcup(config));
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
            configs.push(provider::Provider::Nitrado(
                crate::provider::nitrado::Config::default(),
            ));
            configs.push(provider::Provider::Hetzner(
                crate::provider::hetzner::Config::default(),
            ));
        }

        Ok(configs)
    }

    fn load_dns_configs<P: AsRef<Path>>(dns_dir: P) -> Result<Vec<dns::Type>> {
        let mut configs = Vec::new();
        let dns_dir = dns_dir.as_ref();

        if !dns_dir.exists() {
            info!(
                "DNS directory {:?} does not exist, using empty configs",
                dns_dir
            );
            return Ok(vec![]);
        }

        for entry in std::fs::read_dir(dns_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path
                .extension()
                .map_or(false, |ext| ext == "yaml" || ext == "yml")
            {
                let content = std::fs::read_to_string(&path)?;

                let file_stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                if file_stem.contains("hetzner") {
                    let config: crate::provider::hetzner::DnsConfig =
                        serde_yaml_ng::from_str(&content)?;
                    configs.push(dns::Type::Hetzner(config));
                    debug!("Loaded Hetzner DNS config from {:?}", path);
                } else if file_stem.contains("nitrado") {
                    let config: crate::provider::nitrado::DnsConfig =
                        serde_yaml_ng::from_str(&content)?;
                    configs.push(dns::Type::Nitrado(config));
                    debug!("Loaded Nitrado DNS config from {:?}", path);
                } else if file_stem.contains("netcup") {
                    let config: crate::provider::netcup::DnsConfig =
                        serde_yaml_ng::from_str(&content)?;
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

    pub fn create_example_structure<P: AsRef<Path>>(config_dir: P) -> Result<()> {
        let config_dir = config_dir.as_ref();

        std::fs::create_dir_all(config_dir.join("providers"))?;
        std::fs::create_dir_all(config_dir.join("dns"))?;

        let resolver_config = resolver::Config::default();
        let resolver_yaml = serde_yaml_ng::to_string(&resolver_config)?;
        std::fs::write(config_dir.join("resolver.yaml"), resolver_yaml)?;

        let hetzner_config = crate::provider::hetzner::Config::default();
        let hetzner_yaml = serde_yaml_ng::to_string(&hetzner_config)?;
        std::fs::write(config_dir.join("providers/hetzner.yaml"), hetzner_yaml)?;

        let nitrado_config = crate::provider::nitrado::Config::default();
        let nitrado_yaml = serde_yaml_ng::to_string(&nitrado_config)?;
        std::fs::write(config_dir.join("providers/nitrado.yaml"), nitrado_yaml)?;

        let netcup_config = crate::provider::netcup::Config::default();
        let netcup_yaml = serde_yaml_ng::to_string(&netcup_config)?;
        std::fs::write(config_dir.join("providers/netcup.yaml"), netcup_yaml)?;

        let hetzner_dns_config = crate::provider::hetzner::DnsConfig::default();
        let hetzner_dns_yaml = serde_yaml_ng::to_string(&hetzner_dns_config)?;
        std::fs::write(
            config_dir.join("dns/hetzner-domains.yaml"),
            hetzner_dns_yaml,
        )?;

        let nitrado_dns_config = crate::provider::nitrado::DnsConfig::default();
        let nitrado_dns_yaml = serde_yaml_ng::to_string(&nitrado_dns_config)?;
        std::fs::write(
            config_dir.join("dns/nitrado-domains.yaml"),
            nitrado_dns_yaml,
        )?;

        let netcup_dns_config = crate::provider::netcup::DnsConfig::default();
        let netcup_dns_yaml = serde_yaml_ng::to_string(&netcup_dns_config)?;
        std::fs::write(config_dir.join("dns/netcup-domains.yaml"), netcup_dns_yaml)?;

        info!("Created example config structure in {:?}", config_dir);
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolver: resolver::Config::default(),
            providers: vec![
                provider::Provider::Nitrado(crate::provider::nitrado::Config::default()),
                provider::Provider::Hetzner(crate::provider::hetzner::Config::default()),
                provider::Provider::Netcup(crate::provider::netcup::Config::default()),
            ],
            dns: vec![
                dns::Type::Nitrado(crate::provider::nitrado::DnsConfig::default()),
                dns::Type::Hetzner(crate::provider::hetzner::DnsConfig::default()),
                dns::Type::Netcup(crate::provider::netcup::DnsConfig::default()),
            ],
        }
    }
}

impl MergeFrom<Self> for Config {
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
