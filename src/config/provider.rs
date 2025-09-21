use lum_libs::serde::{Deserialize, Serialize};

use crate::provider::nitrado;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum Provider {
    Nitrado(nitrado::Config),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Config {
    // https://github.com/acatton/serde-yaml-ng/issues/14
    //#[serde(flatten)]
    pub provider: Provider,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            provider: Provider::Nitrado(nitrado::Config::default()),
        }
    }
}
