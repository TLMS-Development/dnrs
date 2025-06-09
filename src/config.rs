use lum_config::MergeFrom;
use lum_libs::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum IpResolverType {
    Raw,
    JSON(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct IpResolver {
    pub url: String,

    #[serde(rename = "type")]
    pub type_: IpResolverType,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct EnvConfig {
    pub ipv4_resolver_url: Option<String>,
    pub ipv4_resolver_type: Option<IpResolverType>,
    pub ipv4_resolver_type_path: String,

    pub ipv6_resolver_url: Option<String>,
    pub ipv6_resolver_type: Option<IpResolverType>,
    pub ipv6_resolver_type_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct FileConfig {
    pub ipv4_resolver: IpResolver,
    pub ipv6_resolver: IpResolver,
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            ipv4_resolver: IpResolver {
                url: "https://v4.ident.me".to_string(),
                type_: IpResolverType::Raw,
            },
            ipv6_resolver: IpResolver {
                url: "https://v6.ident.me".to_string(),
                type_: IpResolverType::Raw,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct Config {
    pub ipv4_resolver: IpResolver,
    pub ipv6_resolver: IpResolver,
}

impl Default for Config {
    fn default() -> Self {
        let file_config = FileConfig::default();

        Config {
            ipv4_resolver: file_config.ipv4_resolver,
            ipv6_resolver: file_config.ipv6_resolver,
        }
    }
}

impl MergeFrom<EnvConfig> for Config {
    fn merge_from(mut self, other: EnvConfig) -> Self {
        if let Some(url) = other.ipv4_resolver_url {
            self.ipv4_resolver.url = url;
        }
        if let Some(resolver_type) = other.ipv4_resolver_type {
            self.ipv4_resolver.type_ = match resolver_type {
                IpResolverType::Raw => IpResolverType::Raw,
                IpResolverType::JSON(_) => IpResolverType::JSON(other.ipv4_resolver_type_path),
            };
        }

        if let Some(url) = other.ipv6_resolver_url {
            self.ipv6_resolver.url = url;
        }
        if let Some(resolver_type) = other.ipv6_resolver_type {
            self.ipv6_resolver.type_ = match resolver_type {
                IpResolverType::Raw => IpResolverType::Raw,
                IpResolverType::JSON(_) => IpResolverType::JSON(other.ipv6_resolver_type_path),
            };
        }

        self
    }
}

impl MergeFrom<FileConfig> for Config {
    fn merge_from(self, other: FileConfig) -> Self {
        Self {
            ipv4_resolver: other.ipv4_resolver,
            ipv6_resolver: other.ipv6_resolver,
        }
    }
}
