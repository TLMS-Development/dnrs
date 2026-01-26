use thiserror::Error;

use crate::provider::{hetzner, netcup, nitrado};

/// Error type for all DNS provider operations.
///
/// This enum wraps provider-specific errors and common failure cases,
/// providing a unified error type for the provider trait.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Error from Hetzner DNS provider
    #[error("Hetzner provider error: {0}")]
    Hetzner(#[from] hetzner::Error),

    /// Error from Nitrado DNS provider
    #[error("Nitrado provider error: {0}")]
    Nitrado(#[from] nitrado::Error),

    /// Error from Netcup DNS provider
    #[error("Netcup provider error: {0}")]
    Netcup(#[from] netcup::Error),

    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    Json(#[from] lum_libs::serde_json::Error),

    /// Invalid API key format for HTTP headers
    #[error("Invalid API key: contains characters not allowed in HTTP headers")]
    InvalidApiKey,
}
