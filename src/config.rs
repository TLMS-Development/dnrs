use lum_config::MergeFrom;
use lum_libs::serde::{Deserialize, Serialize};

pub mod dns;
pub mod provider;
pub mod resolver;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
#[serde(default)]
pub struct Config {
    pub resolver: resolver::Config,
    pub providers: Vec<provider::Config>,
    pub dns: Vec<dns::Config>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            resolver: resolver::Config::default(),
            providers: vec![provider::Config::default()],
            dns: vec![dns::Config::default()],
        }
    }
}

impl MergeFrom<Self> for Config {
    fn merge_from(self, other: Self) -> Self {
        Self {
            resolver: other.resolver,
            providers: other.providers,
            dns: other.dns,
        }
    }
}
