use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::StateError;
use crate::handle::{EnvironmentHandle, ProfileHandle};

/// Lifecycle state of a managed identity.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum IdentityState {
    /// Waiting for the scheduler.
    Queued,
    /// Environment creation has started.
    Spawning,
    /// Environment exists and the profile is being loaded.
    Warming,
    /// The identity is active and healthy.
    Running,
    /// Task artifacts are being collected.
    Harvesting,
    /// Cleanup has started.
    Destroying,
    /// Cleanup completed successfully.
    Destroyed,
    /// The identity entered an error state and requires cleanup.
    Failed,
}

/// Well-known label keys used for reconciliation and scheduling.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum LabelKey {
    /// Session identifier.
    Session,
    /// Run identifier.
    Run,
    /// Image or rootfs identifier.
    Image,
    /// Mount identifier.
    Mount,
}

/// A typed key/value label attached to a state snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Label {
    /// Label key.
    pub key: LabelKey,
    /// Label value.
    pub value: String,
}

/// Persistent state for one managed identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateSnapshot {
    /// Identity identifier.
    pub id: Uuid,
    /// Current lifecycle state.
    pub state: IdentityState,
    /// Identifier of the run that owns this identity.
    pub run_id: Uuid,
    /// Scheduling priority. Higher values are scheduled first.
    pub priority: u8,
    /// Creation timestamp.
    pub created_at: SystemTime,
    /// Last update timestamp.
    pub updated_at: SystemTime,
    /// Labels used for reconciliation and filtering.
    pub labels: Vec<Label>,
    /// Profile handle, once profile generation has completed.
    pub profile_handle: Option<ProfileHandle>,
    /// Environment handle, once environment creation has completed.
    pub environment_handle: Option<EnvironmentHandle>,
}

/// Contract for persisting and restoring identity lifecycle state.
///
/// Implementations whose backend is synchronous (e.g., rusqlite)
/// MUST wrap blocking calls in `tokio::task::spawn_blocking`.
/// Blocking the async runtime violates 01_philosophy.md §4.
#[cfg_attr(test, mockall::automock)]
pub trait StateStore: Send + Sync + 'static {
    /// Inserts or replaces `snapshot`.
    ///
    /// ```no_run
    /// # use sgida_ports::{StateError, StateSnapshot, StateStore};
    /// # use std::future::Future;
    /// # #[allow(dead_code)]
    /// fn put_signature<T: StateStore>(
    ///     store: &T,
    ///     snapshot: StateSnapshot,
    /// ) -> impl Future<Output = Result<(), StateError>> + Send {
    ///     store.put(snapshot)
    /// }
    /// ```
    fn put(&self, snapshot: StateSnapshot) -> impl Future<Output = Result<(), StateError>> + Send;

    /// Returns the snapshot with identifier `id`, if present.
    ///
    /// ```no_run
    /// # use sgida_ports::{StateError, StateSnapshot, StateStore};
    /// # use std::future::Future;
    /// # use uuid::Uuid;
    /// # #[allow(dead_code)]
    /// fn get_signature<T: StateStore>(
    ///     store: &T,
    ///     id: Uuid,
    /// ) -> impl Future<Output = Result<Option<StateSnapshot>, StateError>> + Send {
    ///     store.get(id)
    /// }
    /// ```
    fn get(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<StateSnapshot>, StateError>> + Send;

    /// Deletes the snapshot with identifier `id`.
    ///
    /// ```no_run
    /// # use sgida_ports::{StateError, StateStore};
    /// # use std::future::Future;
    /// # use uuid::Uuid;
    /// # #[allow(dead_code)]
    /// fn delete_signature<T: StateStore>(
    ///     store: &T,
    ///     id: Uuid,
    /// ) -> impl Future<Output = Result<(), StateError>> + Send {
    ///     store.delete(id)
    /// }
    /// ```
    fn delete(&self, id: Uuid) -> impl Future<Output = Result<(), StateError>> + Send;

    /// Returns all stored snapshots.
    ///
    /// ```no_run
    /// # use sgida_ports::{StateError, StateSnapshot, StateStore};
    /// # use std::future::Future;
    /// # #[allow(dead_code)]
    /// fn list_all_signature<T: StateStore>(
    ///     store: &T,
    /// ) -> impl Future<Output = Result<Vec<StateSnapshot>, StateError>> + Send {
    ///     store.list_all()
    /// }
    /// ```
    fn list_all(&self) -> impl Future<Output = Result<Vec<StateSnapshot>, StateError>> + Send;

    /// Returns identities in Queued state, ordered by (priority DESC, created_at ASC).
    /// Implementations must honor this ordering.
    ///
    /// ```no_run
    /// # use sgida_ports::{StateError, StateSnapshot, StateStore};
    /// # use std::future::Future;
    /// # #[allow(dead_code)]
    /// fn list_pending_signature<T: StateStore>(
    ///     store: &T,
    ///     limit: usize,
    /// ) -> impl Future<Output = Result<Vec<StateSnapshot>, StateError>> + Send {
    ///     store.list_pending(limit)
    /// }
    /// ```
    fn list_pending(
        &self,
        limit: usize,
    ) -> impl Future<Output = Result<Vec<StateSnapshot>, StateError>> + Send;
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use uuid::Uuid;

    use super::{
        EnvironmentHandle, IdentityState, Label, LabelKey, MockStateStore, ProfileHandle,
        StateSnapshot,
    };

    #[test]
    fn automock_compiles() {
        let _mock = MockStateStore::new();
    }

    #[test]
    fn state_snapshot_roundtrips_through_json() -> Result<(), serde_json::Error> {
        let snapshot = StateSnapshot {
            id: Uuid::nil(),
            state: IdentityState::Queued,
            run_id: Uuid::nil(),
            priority: 7,
            created_at: SystemTime::UNIX_EPOCH,
            updated_at: SystemTime::UNIX_EPOCH,
            labels: vec![Label {
                key: LabelKey::Session,
                value: "session-1".to_owned(),
            }],
            profile_handle: Some(ProfileHandle::from_uuid(Uuid::nil())),
            environment_handle: Some(EnvironmentHandle::from_uuid(Uuid::nil())),
        };

        let json = serde_json::to_string(&snapshot)?;
        let restored: StateSnapshot = serde_json::from_str(&json)?;

        assert_eq!(snapshot, restored);
        Ok(())
    }
}
