use lum_libs::serde::{Deserialize, Serialize};

use crate::provider::{hetzner, nitrado};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum Provider {
    Nitrado(nitrado::Config),
    Hetzner(hetzner::Config),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct Config {
    // https://github.com/acatton/serde-yaml-ng/issues/14
    //#[serde(flatten)]
    pub providers: Vec<Provider>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            providers: vec![
                Provider::Nitrado(nitrado::Config::default()),
                Provider::Hetzner(hetzner::Config::default()),
            ],
        }
    }
}
