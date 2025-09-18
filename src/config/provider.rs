use lum_libs::serde::{Deserialize, Serialize};

use crate::provider::nitrado;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum Provider {
    Nitrado(nitrado::ProviderConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct FileConfig {
    #[serde(flatten)]
    pub provider: Provider,
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            provider: Provider::Nitrado(nitrado::ProviderConfig::default()),
        }
    }
}
