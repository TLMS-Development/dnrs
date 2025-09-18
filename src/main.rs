use std::fmt::{self, Debug};

use dnrs::{Config, FileConfig, RuntimeError, run, setup_logger};
use lum_config::{
    ConfigPathError, EnvironmentConfigParseError, FileConfigParseError, FileHandler, merge,
};
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

    #[error("Runtime error: {0}")]
    Runtime(#[from] RuntimeError),
}

// When main() returns a `Error`, it will be printed using the `Display` implementation
impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logger()?;

    let file_config: FileConfig = FileHandler::new(APP_NAME, None, None)?.load_config()?;

    let config = Config::default();
    let config = merge(config, file_config);

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
