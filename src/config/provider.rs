use lum_libs::serde::{Deserialize, Serialize};

use crate::provider::nitrado::{self, ProviderConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum ProviderType {
    NitradoConfig(ProviderConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct FileConfig {
    pub providers: Vec<ProviderType>,
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            providers: vec![ProviderType::NitradoConfig(
                nitrado::ProviderConfig::default(),
            )],
        }
    }
}
