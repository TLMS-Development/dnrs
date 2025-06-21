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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "lum_libs::serde")]
pub struct FileConfig {
    pub ipv4: IpResolver,
    pub ipv6: IpResolver,
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            ipv4: IpResolver {
                url: "https://ip.cancom.io".to_string(),
                type_: IpResolverType::Raw,
            },
            ipv6: IpResolver {
                url: "https://ipv6.cancom.io".to_string(),
                type_: IpResolverType::Raw,
            },
        }
    }
}
