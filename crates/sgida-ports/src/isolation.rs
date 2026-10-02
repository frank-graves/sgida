use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::IsolationError, handle::EnvironmentHandle, state::Label};

/// A bind mount applied to an isolated environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mount {
    /// Source path on the host.
    pub source: PathBuf,
    /// Target path inside the isolated environment.
    pub target: PathBuf,
    /// Whether the mount is read-only.
    pub read_only: bool,
}

/// Network configuration for an isolated environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMode {
    /// No network interfaces are created.
    None,
    /// Only loopback networking is available.
    LoopbackOnly,
    /// A bridged interface is attached.
    Bridge {
        /// Host interface used by the bridge.
        interface: String,
    },
}

/// Resource constraints applied to an isolated environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Memory limit in MiB.
    pub memory_mb: u64,
    /// Relative CPU share.
    pub cpu_shares: u64,
    /// Maximum number of processes.
    pub pids_limit: u64,
}

/// Declarative description of an isolated environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerSpec {
    /// Stable identifier for the environment request.
    pub id: Uuid,
    /// Optional image or rootfs reference.
    pub image: Option<String>,
    /// Environment variables injected into the process.
    pub env_vars: BTreeMap<String, String>,
    /// Mounts applied to the environment.
    pub mounts: Vec<Mount>,
    /// Network mode for the environment.
    pub network_mode: NetworkMode,
    /// Resource limits for the environment.
    pub resources: ResourceLimits,
    /// Labels used for reconciliation and scheduling.
    pub labels: Vec<Label>,
}

/// Captured output from a command executed in an isolated environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecOutput {
    /// Process exit code.
    pub exit_code: i32,
    /// Captured standard output.
    pub stdout: Bytes,
    /// Captured standard error.
    pub stderr: Bytes,
    /// Wall-clock duration of the command.
    pub duration: Duration,
    /// Whether standard output was truncated.
    pub stdout_truncated: bool,
    /// Whether standard error was truncated.
    pub stderr_truncated: bool,
}

/// Health status of an isolated environment.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum Health {
    /// The environment is fully operational.
    Healthy,
    /// The environment is still initializing.
    Starting,
    /// The environment is operational but impaired.
    Degraded,
    /// The environment is not operational.
    Unhealthy,
}

/// Contract for creating, executing, destroying, and health-checking isolated
/// environments.
///
/// The returned future is Send. Do not remove `+ Send` from the return types;
/// sgida-orchestrator spawns tokio tasks that call these methods.
///
/// Implementations must not block the async runtime.
#[cfg_attr(test, mockall::automock)]
pub trait IsolationDriver: Send + Sync + 'static {
    /// Creates an isolated environment from `spec`.
    ///
    /// ```no_run
    /// # use sgida_ports::{ContainerSpec, EnvironmentHandle, IsolationDriver, IsolationError};
    /// # use std::future::Future;
    /// # #[allow(dead_code)]
    /// fn create_signature<T: IsolationDriver>(
    ///     driver: &T,
    ///     spec: ContainerSpec,
    /// ) -> impl Future<Output = Result<EnvironmentHandle, IsolationError>> + Send {
    ///     driver.create(spec)
    /// }
    /// ```
    fn create(
        &self,
        spec: ContainerSpec,
    ) -> impl Future<Output = Result<EnvironmentHandle, IsolationError>> + Send;

    /// Executes `cmd` inside the environment referenced by `env`.
    ///
    /// ```no_run
    /// # use sgida_ports::{EnvironmentHandle, ExecOutput, IsolationDriver, IsolationError};
    /// # use std::future::Future;
    /// # #[allow(dead_code)]
    /// fn exec_signature<T: IsolationDriver>(
    ///     driver: &T,
    ///     env: &EnvironmentHandle,
    ///     cmd: &[String],
    ///     max_output_bytes: usize,
    /// ) -> impl Future<Output = Result<ExecOutput, IsolationError>> + Send {
    ///     driver.exec(env, cmd, max_output_bytes)
    /// }
    /// ```
    fn exec(
        &self,
        env: &EnvironmentHandle,
        cmd: &[String],
        max_output_bytes: usize,
    ) -> impl Future<Output = Result<ExecOutput, IsolationError>> + Send;

    /// Destroys the environment referenced by `env`.
    ///
    /// ```no_run
    /// # use sgida_ports::{EnvironmentHandle, IsolationDriver, IsolationError};
    /// # use std::future::Future;
    /// # #[allow(dead_code)]
    /// fn destroy_signature<T: IsolationDriver>(
    ///     driver: &T,
    ///     env: &EnvironmentHandle,
    /// ) -> impl Future<Output = Result<(), IsolationError>> + Send {
    ///     driver.destroy(env)
    /// }
    /// ```
    fn destroy(
        &self,
        env: &EnvironmentHandle,
    ) -> impl Future<Output = Result<(), IsolationError>> + Send;

    /// Returns the current health of the environment referenced by `env`.
    ///
    /// ```no_run
    /// # use sgida_ports::{EnvironmentHandle, Health, IsolationDriver, IsolationError};
    /// # use std::future::Future;
    /// # #[allow(dead_code)]
    /// fn health_signature<T: IsolationDriver>(
    ///     driver: &T,
    ///     env: &EnvironmentHandle,
    /// ) -> impl Future<Output = Result<Health, IsolationError>> + Send {
    ///     driver.health(env)
    /// }
    /// ```
    fn health(
        &self,
        env: &EnvironmentHandle,
    ) -> impl Future<Output = Result<Health, IsolationError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::MockIsolationDriver;

    #[test]
    fn automock_compiles() {
        let _mock = MockIsolationDriver::new();
    }
}
