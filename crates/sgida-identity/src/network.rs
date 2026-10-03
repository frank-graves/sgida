// crates/sgida-identity/src/network.rs
//! Network profile attributes.

use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;

/// Network-level attributes.
#[derive(Debug, Clone)]
pub struct NetworkProfile {
    /// The ISO 3166-1 alpha-2 region code.
    pub region: String,
}

impl NetworkProfile {
    /// Generates a network profile from a seed.
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let mut rng = ChaCha12Rng::from_seed(*seed);
        let regions = ["ES", "US", "JP", "GB", "DE"];
        let region = regions.choose(&mut rng).unwrap_or(&"ES").to_string();
        Self { region }
    }
}
