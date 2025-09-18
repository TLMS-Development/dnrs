use lum_config::MergeFrom;
use lum_libs::serde::{Deserialize, Serialize};

pub mod dns;
pub mod provider;
pub mod resolver;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct FileConfig {
    resolver: resolver::FileConfig,
    providers: Vec<provider::FileConfig>,
    dns: Vec<dns::FileConfig>,
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            resolver: resolver::FileConfig::default(),
            providers: vec![provider::FileConfig::default()],
            dns: vec![dns::FileConfig::default()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct Config {
    pub resolver: resolver::FileConfig,
    pub providers: Vec<provider::FileConfig>,
    pub dns: Vec<dns::FileConfig>,
}

impl Default for Config {
    fn default() -> Self {
        let file_config = FileConfig::default();

        Config {
            resolver: file_config.resolver,
            providers: file_config.providers,
            dns: file_config.dns,
        }
    }
}

impl MergeFrom<FileConfig> for Config {
    fn merge_from(self, other: FileConfig) -> Self {
        Self {
            resolver: other.resolver,
            providers: other.providers,
            dns: other.dns,
        }
    }
}
