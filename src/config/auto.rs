use lum_libs::serde::{Deserialize, Serialize};

use crate::provider::nitrado::AutoConfig as NitradoAutoConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum ProviderType {
    NitradoConfig(NitradoAutoConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct FileConfig {
    pub domains: Vec<ProviderType>,
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            domains: vec![ProviderType::NitradoConfig(NitradoAutoConfig::default())],
        }
    }
}
