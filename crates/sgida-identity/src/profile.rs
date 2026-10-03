// crates/sgida-identity/src/profile.rs
//! The core Profile structure.

use std::fmt;
use uuid::Uuid;

use crate::behavior::BehaviorProfile;
use crate::browser::BrowserProfile;
use crate::canvas::CanvasProfile;
use crate::credentials::Credentials;
use crate::error::InternalError;
use crate::network::NetworkProfile;
use crate::seed;
use crate::system::SystemProfile;
use crate::validator;
use sgida_ports::MasterSeed;

/// A complete, internally coherent synthetic profile.
#[derive(Clone)]
pub struct Profile {
    /// The unique identifier for this profile.
    pub id: Uuid,
    /// The master seed used to generate this profile.
    pub seed: [u8; 32],
    /// Browser-specific attributes.
    pub browser: BrowserProfile,
    /// System-level attributes.
    pub system: SystemProfile,
    /// Network-level attributes.
    pub network: NetworkProfile,
    /// Behavioral attributes.
    pub behavior: BehaviorProfile,
    /// Canvas and WebGL attributes.
    pub canvas: CanvasProfile,
    /// Synthetic credentials.
    pub credentials: Credentials,
}

impl fmt::Debug for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Profile")
            .field("id", &self.id)
            .field("seed", &self.seed)
            .field("browser", &self.browser)
            .field("system", &self.system)
            .field("network", &self.network)
            .field("behavior", &self.behavior)
            .field("canvas", &self.canvas)
            .field("credentials", &"<redacted>")
            .finish()
    }
}

impl Profile {
    /// Generates a deterministic profile from a master seed.
    pub fn from_seed(seed: MasterSeed) -> Result<Self, InternalError> {
        let derived = seed::derive(&seed)?;
        let id = Uuid::from_bytes(derived.profile_id);

        let browser = BrowserProfile::from_seed(&derived.browser)?;
        let system = SystemProfile::from_seed(&derived.system, &browser.platform);
        let network = NetworkProfile::from_seed(&derived.network);
        let behavior = BehaviorProfile::from_seed(&derived.behavior);
        let canvas = CanvasProfile::from_seed(
            &derived.canvas,
            browser.webgl_vendor.clone(),
            browser.webgl_renderer.clone(),
        );
        let credentials = Credentials::from_seed(&derived.credentials);

        let seed_arr: [u8; 32] = *seed.expose_to_provider();

        let profile = Self {
            id,
            seed: seed_arr,
            browser,
            system,
            network,
            behavior,
            canvas,
            credentials,
        };

        validator::validate(&profile)?;

        Ok(profile)
    }

    /// Generates a random profile using OS randomness.
    pub fn random() -> Result<Self, InternalError> {
        let mut seed = [0u8; 32];
        use rand::RngCore;
        rand::rngs::OsRng.fill_bytes(&mut seed);
        let master = MasterSeed::from_bytes(seed);
        Self::from_seed(master)
    }
}
