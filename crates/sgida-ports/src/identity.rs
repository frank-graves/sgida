use std::future::Future;

use crate::{error::IdentityError, handle::ProfileHandle, seed::MasterSeed};

/// Contract for deterministic profile generation and destruction.
///
/// The returned future is Send. Do not remove `+ Send` from the return types;
/// sgida-orchestrator spawns tokio tasks that call these methods.
///
/// Implementations must not block the async runtime.
#[cfg_attr(test, mockall::automock)]
pub trait IdentityProvider: Send + Sync + 'static {
    /// Generates a profile from `seed` and returns an opaque handle.
    ///
    /// ```no_run
    /// # use sgida_ports::{IdentityError, IdentityProvider, MasterSeed, ProfileHandle};
    /// # use std::future::Future;
    /// # #[allow(dead_code)]
    /// fn generate_signature<T: IdentityProvider>(
    ///     provider: &T,
    ///     seed: MasterSeed,
    /// ) -> impl Future<Output = Result<ProfileHandle, IdentityError>> + Send {
    ///     provider.generate(seed)
    /// }
    /// ```
    fn generate(
        &self,
        seed: MasterSeed,
    ) -> impl Future<Output = Result<ProfileHandle, IdentityError>> + Send;

    /// Destroys the profile referenced by `handle`.
    ///
    /// ```no_run
    /// # use sgida_ports::{IdentityError, IdentityProvider, ProfileHandle};
    /// # use std::future::Future;
    /// # #[allow(dead_code)]
    /// fn destroy_signature<T: IdentityProvider>(
    ///     provider: &T,
    ///     handle: &ProfileHandle,
    /// ) -> impl Future<Output = Result<(), IdentityError>> + Send {
    ///     provider.destroy(handle)
    /// }
    /// ```
    fn destroy(
        &self,
        handle: &ProfileHandle,
    ) -> impl Future<Output = Result<(), IdentityError>> + Send;
}

#[cfg(test)]
mod tests {
    use super::MockIdentityProvider;

    #[test]
    fn automock_compiles() {
        let _mock = MockIdentityProvider::new();
    }
}
