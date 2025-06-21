use lum_libs::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct NitradoConfig {
    pub name: String,
    pub api_key: String,
    pub api_base_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub enum ProviderType {
    NitradoConfig(NitradoConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct FileConfig {
    pub providers: Vec<ProviderType>,
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            providers: vec![ProviderType::NitradoConfig(NitradoConfig {
                name: "Nitrado".to_string(),
                api_key: "your_api_key".to_string(),
                api_base_url: "https://api.nitrado.net".to_string(),
            })],
        }
    }
}
