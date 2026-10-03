// crates/sgida-identity/src/behavior.rs
//! Behavior profile attributes.

use std::fmt;

/// Behavioral attributes.
#[derive(Clone)]
pub struct BehaviorProfile {
    /// The seed for behavioral models.
    // Retained for potential future re-derivation; Debug output redacts it.
    #[expect(dead_code)]
    pub(crate) seed: [u8; 32],
}

impl fmt::Debug for BehaviorProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BehaviorProfile")
            .field("seed", &"<redacted>")
            .finish()
    }
}

impl BehaviorProfile {
    /// Generates a behavior profile from a seed.
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self { seed: *seed }
    }
}
