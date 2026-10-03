// crates/sgida-identity/src/canvas.rs
//! Canvas profile attributes.

use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;

/// Canvas and WebGL attributes.
#[derive(Debug, Clone)]
pub struct CanvasProfile {
    /// The noise seed for canvas fingerprinting.
    pub noise_seed: u32,
    /// The WebGL vendor string.
    pub webgl_vendor: String,
    /// The WebGL renderer string.
    pub webgl_renderer: String,
}

impl CanvasProfile {
    /// Generates a canvas profile from a seed, using catalog-provided WebGL strings.
    pub fn from_seed(seed: &[u8; 32], webgl_vendor: String, webgl_renderer: String) -> Self {
        let mut rng = ChaCha12Rng::from_seed(*seed);
        let noise_seed = rng.gen();

        Self {
            noise_seed,
            webgl_vendor,
            webgl_renderer,
        }
    }
}
