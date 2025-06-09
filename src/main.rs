use dnrs::setup_logger;
use lum_log::{error, info};
use reqwest::header::{self, HeaderMap, HeaderValue};

/*
    - Adapters for different DNS providers
      - Nitrado
      - Netcup
      - Custom (see below)
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
      - Use a service like `v4.ident.me` and `v6.ident.me`
      - Raw or JSON with custom path
    - User interface
      - CLI + config file
      - CLI and config file result in same options struct
      - CLI options can override config file options
*/

#[tokio::main]
async fn main() -> Result<(), u8> {
    info!("Info without logger");
    if let Err(e) = setup_logger() {
        error!("Failed to setup logger: {}", e);
        return Err(1);
    }
    info!("Info with logger");

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
}
