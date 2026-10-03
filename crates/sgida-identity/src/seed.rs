// crates/sgida-identity/src/seed.rs
//! HKDF-based seed derivation.

use hkdf::Hkdf;
use sgida_ports::MasterSeed;
use sha2::Sha256;

use crate::error::InternalError;

/// Derived sub-seeds for each subsystem.
pub struct DerivedSeeds {
    /// Seed for the profile ID.
    pub profile_id: [u8; 16],
    /// Seed for the browser profile.
    pub browser: [u8; 32],
    /// Seed for the system profile.
    pub system: [u8; 32],
    /// Seed for the network profile.
    pub network: [u8; 32],
    /// Seed for the behavior profile.
    pub behavior: [u8; 32],
    /// Seed for the canvas profile.
    pub canvas: [u8; 32],
    /// Seed for the credentials.
    pub credentials: [u8; 32],
}

/// Derives sub-seeds from a master seed using HKDF-SHA256.
pub fn derive(master: &MasterSeed) -> Result<DerivedSeeds, InternalError> {
    let hkdf = Hkdf::<Sha256>::new(None, master.expose_to_provider());

    let mut profile_id = [0u8; 16];
    hkdf.expand(b"sgida.profile_id", &mut profile_id)
        .map_err(|_| InternalError::Hkdf("sgida.profile_id".into()))?;

    let mut browser = [0u8; 32];
    hkdf.expand(b"sgida.browser", &mut browser)
        .map_err(|_| InternalError::Hkdf("sgida.browser".into()))?;

    let mut system = [0u8; 32];
    hkdf.expand(b"sgida.system", &mut system)
        .map_err(|_| InternalError::Hkdf("sgida.system".into()))?;

    let mut network = [0u8; 32];
    hkdf.expand(b"sgida.network", &mut network)
        .map_err(|_| InternalError::Hkdf("sgida.network".into()))?;

    let mut behavior = [0u8; 32];
    hkdf.expand(b"sgida.behavior", &mut behavior)
        .map_err(|_| InternalError::Hkdf("sgida.behavior".into()))?;

    let mut canvas = [0u8; 32];
    hkdf.expand(b"sgida.canvas", &mut canvas)
        .map_err(|_| InternalError::Hkdf("sgida.canvas".into()))?;

    let mut credentials = [0u8; 32];
    hkdf.expand(b"sgida.credentials", &mut credentials)
        .map_err(|_| InternalError::Hkdf("sgida.credentials".into()))?;

    Ok(DerivedSeeds {
        profile_id,
        browser,
        system,
        network,
        behavior,
        canvas,
        credentials,
    })
}
