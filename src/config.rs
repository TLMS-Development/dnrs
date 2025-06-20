use lum_config::MergeFrom;
use lum_libs::serde::{Deserialize, Serialize};

use crate::config::resolver::IpResolverType;

pub mod resolver;

//TODO: Put command-related options into their own struct as soon as serde-env supports flatten
// See: https://github.com/Xuanwo/serde-env/issues/15
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct EnvConfig {
    pub ipv4_resolver_url: Option<String>,
    pub ipv4_resolver_type: Option<IpResolverType>,
    pub ipv4_resolver_json_path: Option<String>,

    pub ipv6_resolver_url: Option<String>,
    pub ipv6_resolver_type: Option<IpResolverType>,
    pub ipv6_resolver_json_path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct FileConfig {
    resolver: resolver::FileConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct Config {
    pub resolver: resolver::FileConfig,
}

impl Default for Config {
    fn default() -> Self {
        let file_config = FileConfig::default();

        Config {
            resolver: file_config.resolver,
        }
    }
}

impl MergeFrom<EnvConfig> for Config {
    fn merge_from(mut self, other: EnvConfig) -> Self {
        if let Some(url) = other.ipv4_resolver_url {
            self.resolver.ipv4.url = url;
        }
        if let Some(resolver_type) = other.ipv4_resolver_type {
            self.resolver.ipv4.type_ = match resolver_type {
                IpResolverType::Raw => IpResolverType::Raw,
                IpResolverType::JSON(_) => {
                    IpResolverType::JSON(other.ipv4_resolver_json_path.unwrap())
                } // TODO: Handle unwrap
            };
        }

        if let Some(url) = other.ipv6_resolver_url {
            self.resolver.ipv6.url = url;
        }
        if let Some(resolver_type) = other.ipv6_resolver_type {
            self.resolver.ipv6.type_ = match resolver_type {
                IpResolverType::Raw => IpResolverType::Raw,
                IpResolverType::JSON(_) => {
                    IpResolverType::JSON(other.ipv6_resolver_json_path.unwrap())
                } // TODO: Handle unwrap
            };
        }

        self
    }
}

impl MergeFrom<FileConfig> for Config {
    fn merge_from(self, other: FileConfig) -> Self {
        Self {
            resolver: other.resolver,
        }
    }
}
