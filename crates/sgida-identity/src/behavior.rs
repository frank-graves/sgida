// crates/sgida-identity/src/behavior.rs
//! Behavior profile attributes.

/// Behavioral attributes.
#[derive(Debug, Clone)]
pub struct BehaviorProfile {
    /// The seed for behavioral models.
    pub seed: [u8; 32],
}

impl BehaviorProfile {
    /// Generates a behavior profile from a seed.
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self { seed: *seed }
    }
}
