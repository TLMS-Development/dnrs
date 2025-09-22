use std::fmt::{self, Debug};
use std::fs::{self, File};

use dnrs::{Config, RuntimeError, run, setup_logger};
use lum_config::{ConfigPathError, EnvironmentConfigParseError, FileConfigParseError, merge};
use lum_log::{error, info, log::SetLoggerError};
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
    - Add conditional Config files (per provider configs)
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

    #[error("Config error: {0}")]
    Config(#[from] anyhow::Error),

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

    let config_dir = dirs::config_dir()
        .ok_or(Error::NoConfigDirectory)?
        .join(APP_NAME);

    let config = if config_dir.exists() {
        Config::load_from_directory(&config_dir)?
    } else {
        info!("Config directory does not exist, creating default structure...");
        fs::create_dir_all(&config_dir)?;

        Config::create_example_structure(&config_dir)?;
        info!(
            "Created default config structure at: {}",
            config_dir.display()
        );
        info!("Please configure your providers and DNS settings, then run again.");

        Config::default()
    };

    run(config).await?;
    Ok(())
}
