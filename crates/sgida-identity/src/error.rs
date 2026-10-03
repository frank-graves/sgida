// crates/sgida-identity/src/error.rs
//! Internal errors for the identity module.

use sgida_ports::IdentityError;
use thiserror::Error;

/// Internal errors specific to the identity module.
#[derive(Debug, Error)]
pub enum InternalError {
    /// Coherence validation failed.
    #[error("invalid coherence: {0}")]
    Coherence(String),
    /// The user agent catalog is empty.
    #[error("user agent catalog is empty")]
    EmptyCatalog,
    /// Failed to parse the user agent catalog.
    #[error("failed to parse user agent catalog: {0}")]
    CatalogParse(String),
    /// HKDF expand failed for an unexpected reason.
    #[error("hkdf expand failed for {0}")]
    Hkdf(String),
}

impl From<InternalError> for IdentityError {
    fn from(err: InternalError) -> Self {
        match err {
            InternalError::Coherence(msg) => IdentityError::Incoherent(msg),
            InternalError::EmptyCatalog => {
                IdentityError::Generation("user agent catalog is empty".to_string())
            }
            InternalError::CatalogParse(msg) => IdentityError::Generation(msg),
            InternalError::Hkdf(msg) => IdentityError::Generation(msg),
        }
    }
}
