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
}

#[cfg(test)]
mod tests {
    use super::*;
    use lum_libs::serde_json;

    #[test]
    fn test_hetzner_error_display() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let err = hetzner::Error::Json(json_err);
        let display = format!("{}", err);
        assert!(display.contains("JSON parsing error"));
    }

    #[test]
    fn test_hetzner_domain_not_found_display() {
        let err = hetzner::Error::DomainNotFound("example.com".to_string());
        let display = format!("{}", err);
        assert_eq!(display, "Domain 'example.com' not found in Hetzner zones");
    }

    #[test]
    fn test_nitrado_error_display() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let err = nitrado::Error::Json(json_err);
        let display = format!("{}", err);
        assert!(display.contains("JSON parsing error"));
    }

    #[test]
    fn test_netcup_error_display() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let err = netcup::Error::Json(json_err);
        let display = format!("{}", err);
        assert!(display.contains("JSON parsing error"));
    }

    #[test]
    fn test_netcup_domain_not_found_display() {
        let err = netcup::Error::DomainNotFound("example.com".to_string());
        let display = format!("{}", err);
        assert_eq!(display, "Domain 'example.com' not found in Netcup zones");
    }

    #[test]
    fn test_provider_error_from_hetzner() {
        let hetzner_err = hetzner::Error::DomainNotFound("example.com".to_string());
        let provider_err: ProviderError = hetzner_err.into();

        assert!(matches!(provider_err, ProviderError::Hetzner(_)));
        let display = format!("{}", provider_err);
        assert!(display.contains("Hetzner provider error"));
        assert!(display.contains("example.com"));
    }

    #[test]
    fn test_provider_error_from_nitrado() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let nitrado_err = nitrado::Error::Json(json_err);
        let provider_err: ProviderError = nitrado_err.into();

        assert!(matches!(provider_err, ProviderError::Nitrado(_)));
        let display = format!("{}", provider_err);
        assert!(display.contains("Nitrado provider error"));
    }

    #[test]
    fn test_provider_error_from_netcup() {
        let netcup_err = netcup::Error::DomainNotFound("test.org".to_string());
        let provider_err: ProviderError = netcup_err.into();

        assert!(matches!(provider_err, ProviderError::Netcup(_)));
        let display = format!("{}", provider_err);
        assert!(display.contains("Netcup provider error"));
        assert!(display.contains("test.org"));
    }

    #[test]
    fn test_provider_error_debug() {
        let hetzner_err = hetzner::Error::DomainNotFound("debug-test.com".to_string());
        let provider_err: ProviderError = hetzner_err.into();

        let debug = format!("{:?}", provider_err);
        assert!(debug.contains("Hetzner"));
        assert!(debug.contains("DomainNotFound"));
    }
}
