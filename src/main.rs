use std::fmt::{self, Debug};
use std::fs::{self, File};

use dnrs::{Config, RuntimeError, run, setup_logger};
use lum_config::{ConfigPathError, EnvironmentConfigParseError, FileConfigParseError, merge};
use lum_log::info;
use lum_log::{error, log::SetLoggerError};
use thiserror::Error;

/*
    - Adapters for different DNS providers
      - Nitrado
      - Netcup
      - Custom (see below)
      - Implement Nitrado provider
    - Config
      - Startup check if invalid/double provider/auto names in config
    - Custom DNS provider
      - Allow user to define their own request
        - HTTP method
        - URL
        - Headers
        - Body
    - Trait: `DnsProvider`
      - defines common behavior between different DNS providers
      - check (connection, provider status, etc. Basically make sure provider is ready to use)
      - set(domain: &str, record_type: &str, value: &str)
    - Get current IP address
      - Use a service like `ip.cancom.io` and `ipv6.cancom.io`
      - Raw or JSON with custom path
    - User interface
      - CLI + config file
      - CLI and config file result in same options struct
      - CLI options can override config file options
    - Add command: Always accept multiple domains, use batch if provider supports it
    - Models (Responses, Errors)
*/

const APP_NAME: &str = "dnrs";

#[derive(Error)]
enum Error {
    #[error("Failed to setup logger: {0}")]
    SetLogger(#[from] SetLoggerError),

    #[error("Failed to parse environment config: {0}")]
    EnvConfig(#[from] EnvironmentConfigParseError),

    #[error("Failed to load file config: {0}")]
    FileConfig(#[from] FileConfigParseError),

    #[error("Failed to load file config: {0}")]
    FileHandler(#[from] ConfigPathError),

    #[error("YAML config error: {0}")]
    YamlConfig(#[from] serde_yaml_ng::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unable to determine config directory")]
    NoConfigDirectory,

    #[error("Runtime error: {0}")]
    Runtime(#[from] RuntimeError),
}

// When main() returns an `Error`, it will be printed using the `Display` implementation
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logger()?;

    let config_path = dirs::config_dir()
        .ok_or(Error::NoConfigDirectory)?
        .join(APP_NAME)
        .join("config.yaml");

    let mut loaded_config: Option<Config> = None;
    if config_path.exists() {
        let file = File::open(&config_path)?;
        loaded_config = Some(serde_yaml_ng::from_reader(file)?);
    }
    let config_existed = loaded_config.is_some();

    let default_config = Config::default();
    let config = match loaded_config {
        Some(loaded_config) => merge(default_config, loaded_config),
        None => default_config,
    };

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::create(&config_path)?;
    serde_yaml_ng::to_writer(file, &config)?;

    if !config_existed {
        info!("Created default config file at: {}", config_path.display());
    }

    run(config).await?;
    Ok(())
}

/*
let reqwest = reqwest::Client::new();

    let auth = format!("Bearer {}", dnrs::TOKEN);

    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_str(&auth).unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let response = reqwest
        .get("https://api.nitrado.net/domain/<domain>/records")
        .headers(headers)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(error) => {
            error!("Error sending request: {}", error);
            return Err(1);
        }
    };

    let text = match response.text().await {
        Ok(text) => text,
        Err(error) => {
            error!("Error reading response text: {}", error);
            return Err(1);
        }
    };

    info!("Result: {}", text);
    Ok(())
     */
