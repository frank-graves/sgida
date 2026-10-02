use std::time::Duration;

use uuid::Uuid;

/// Failure modes for identity/profile providers.
#[derive(Debug, thiserror::Error)]
pub enum IdentityError {
    /// Profile generation failed.
    #[error("profile generation failed: {0}")]
    Generation(String),

    /// No profile exists for the requested identifier.
    #[error("profile not found: {id}")]
    NotFound {
        /// Identifier of the missing profile.
        id: Uuid,
    },

    /// A generated profile failed coherence validation.
    #[error("coherence validation failed: {0}")]
    Incoherent(String),
}

/// Failure modes for isolation drivers.
#[derive(Debug, thiserror::Error)]
pub enum IsolationError {
    /// Environment creation failed.
    #[error("failed to create environment: {0}")]
    Create(String),

    /// Command execution inside an environment failed.
    #[error("failed to exec in environment: {0}")]
    Exec(String),

    /// Environment destruction failed.
    #[error("failed to destroy environment: {0}")]
    Destroy(String),

    /// No environment exists for the requested identifier.
    #[error("environment not found: {id}")]
    NotFound {
        /// Identifier of the missing environment.
        id: Uuid,
    },

    /// The operation exceeded its deadline.
    #[error("operation timed out after {0:?}")]
    Timeout(Duration),
}

/// Failure modes for state stores.
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    /// The storage backend failed.
    #[error("backend error: {0}")]
    Backend(String),

    /// No snapshot exists for the requested identifier.
    #[error("snapshot not found: {id}")]
    NotFound {
        /// Identifier of the missing snapshot.
        id: Uuid,
    },

    /// Serialization or deserialization failed.
    #[error("serialization error: {0}")]
    Serialization(String),
}

/// Umbrella error for all port contracts.
#[derive(Debug, thiserror::Error)]
pub enum PortError {
    /// Identity provider failure.
    #[error(transparent)]
    Identity(#[from] IdentityError),

    /// Isolation driver failure.
    #[error(transparent)]
    Isolation(#[from] IsolationError),

    /// State store failure.
    #[error(transparent)]
    State(#[from] StateError),
}
