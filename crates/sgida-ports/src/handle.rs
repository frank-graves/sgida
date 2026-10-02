use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Opaque profile identifier.
///
/// Opaque to the orchestrator: no method exposes the underlying resource.
/// Serializable so that crash recovery can persist and restore the handle.
/// Do not add accessors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ProfileHandle(Uuid);

impl ProfileHandle {
    /// Returns the underlying identifier for persistence and reconciliation.
    pub const fn as_uuid(&self) -> Uuid {
        self.0
    }

    /// Creates a handle from a persisted identifier.
    ///
    /// Used by `sgida-orchestrator` when restoring handles from `StateSnapshot`
    /// after a crash. See `docs/decisions/0002-opacidad-operativa.md`.
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

/// Opaque isolated environment identifier.
///
/// Opaque to the orchestrator: no method exposes the underlying resource.
/// Serializable so that crash recovery can persist and restore the handle.
/// Do not add accessors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct EnvironmentHandle(Uuid);

impl EnvironmentHandle {
    /// Returns the underlying identifier for persistence and reconciliation.
    pub const fn as_uuid(&self) -> Uuid {
        self.0
    }

    /// Creates a handle from a persisted identifier.
    ///
    /// Used by `sgida-orchestrator` when restoring handles from `StateSnapshot`
    /// after a crash. See `docs/decisions/0002-opacidad-operativa.md`.
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
