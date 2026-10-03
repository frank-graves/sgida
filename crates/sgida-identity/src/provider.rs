// crates/sgida-identity/src/provider.rs
//! In-memory identity provider.

use dashmap::DashMap;
use sgida_ports::{IdentityError, IdentityProvider, MasterSeed, ProfileHandle};
use std::future::ready;
use uuid::Uuid;

use crate::profile::Profile;

/// An in-memory implementation of the identity provider.
pub struct InMemoryIdentityProvider {
    profiles: DashMap<Uuid, Profile>,
}

impl InMemoryIdentityProvider {
    /// Creates a new in-memory identity provider.
    pub fn new() -> Self {
        Self {
            profiles: DashMap::new(),
        }
    }

    /// Retrieves a profile by its ID.
    pub fn get(&self, id: Uuid) -> Option<Profile> {
        self.profiles.get(&id).map(|p| p.clone())
    }

    /// Returns the number of profiles.
    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    /// Returns true if there are no profiles.
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }

    fn generate_sync(&self, seed: MasterSeed) -> Result<ProfileHandle, IdentityError> {
        let profile = Profile::from_seed(seed).map_err(IdentityError::from)?;
        let id = profile.id;
        self.profiles.insert(id, profile);
        Ok(ProfileHandle::from_uuid(id))
    }

    fn destroy_sync(&self, handle: &ProfileHandle) -> Result<(), IdentityError> {
        let id = handle.as_uuid();
        self.profiles.remove(&id);
        Ok(())
    }
}

impl Default for InMemoryIdentityProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentityProvider for InMemoryIdentityProvider {
    fn generate(
        &self,
        seed: MasterSeed,
    ) -> impl std::future::Future<Output = Result<ProfileHandle, IdentityError>> + Send {
        ready(self.generate_sync(seed))
    }

    fn destroy(
        &self,
        handle: &ProfileHandle,
    ) -> impl std::future::Future<Output = Result<(), IdentityError>> + Send {
        Box::pin(ready(self.destroy_sync(handle)))
    }
}
