// crates/sgida-identity/src/system.rs
//! System profile attributes.

use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;

/// System-level attributes.
#[derive(Debug, Clone)]
pub struct SystemProfile {
    /// The number of logical processors.
    pub hardware_concurrency: u8,
    /// The device memory in GB.
    pub device_memory: f32,
    /// The IANA timezone name.
    pub timezone: String,
    /// The BCP-47 locale.
    pub locale: String,
    /// The screen resolution (width, height).
    pub screen_resolution: (u32, u32),
    /// The available screen dimensions (width, height).
    pub available_screen: (u32, u32),
    /// The color gamut.
    pub color_gamut: String,
}

impl SystemProfile {
    /// Generates a system profile from a seed, constrained by platform.
    pub fn from_seed(seed: &[u8; 32], platform: &str) -> Self {
        let mut rng = ChaCha12Rng::from_seed(*seed);

        let concurrencies = [2, 4, 8, 16];
        let hardware_concurrency = *concurrencies.choose(&mut rng).unwrap_or(&8);

        let memories = [0.25f32, 0.5, 1.0, 2.0, 4.0, 8.0];
        let device_memory = *memories.choose(&mut rng).unwrap_or(&4.0);

        let locales: &[&str] = match platform {
            "Win32" => &["en-US", "es-ES", "en-GB"],
            "MacIntel" => &["en-US", "ja-JP", "es-ES"],
            "Linux x86_64" => &["en-US", "en-GB"],
            _ => &["en-US"],
        };
        let locale = *locales.choose(&mut rng).unwrap_or(&"en-US");

        let timezone = match locale {
            "en-US" => *["America/New_York", "America/Los_Angeles", "America/Chicago"]
                .choose(&mut rng)
                .unwrap_or(&"America/New_York"),
            "es-ES" => *["Europe/Madrid", "Atlantic/Canary"]
                .choose(&mut rng)
                .unwrap_or(&"Europe/Madrid"),
            "ja-JP" => "Asia/Tokyo",
            _ => "Europe/London",
        };

        let screen_resolution = (1920, 1080);
        let available_screen = (1920, 1040);

        let gamuts = ["srgb", "p3", "rec2020"];
        let color_gamut = gamuts.choose(&mut rng).unwrap_or(&"srgb").to_string();

        Self {
            hardware_concurrency,
            device_memory,
            timezone: timezone.to_string(),
            locale: locale.to_string(),
            screen_resolution,
            available_screen,
            color_gamut,
        }
    }
}
