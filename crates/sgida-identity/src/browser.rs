// crates/sgida-identity/src/browser.rs
//! Browser profile attributes.

use rand::SeedableRng;
use rand::seq::SliceRandom;
use rand_chacha::ChaCha12Rng;
use serde::Deserialize;
use std::sync::LazyLock;

use crate::error::InternalError;

/// Browser-specific attributes.
#[derive(Debug, Clone)]
pub struct BrowserProfile {
    /// The user agent string.
    pub user_agent: String,
    /// The viewport dimensions (width, height).
    pub viewport: (u32, u32),
    /// The color depth.
    pub color_depth: u8,
    /// The device pixel ratio.
    pub pixel_ratio: f32,
    /// The preferred languages.
    pub languages: Vec<String>,
    /// The platform string.
    pub platform: String,
    /// The oscpu string.
    pub oscpu: Option<String>,
    /// The product sub string.
    pub product_sub: String,
    /// The vendor string.
    pub vendor: String,
    /// The WebGL vendor string.
    pub webgl_vendor: String,
    /// The WebGL renderer string.
    pub webgl_renderer: String,
}

#[derive(Deserialize)]
struct CatalogEntry {
    user_agent: String,
    platform: String,
    oscpu: Option<String>,
    vendor: String,
    product_sub: String,
    webgl_vendor: String,
    webgl_renderer: String,
}

#[derive(Deserialize)]
struct Catalog {
    entries: Vec<CatalogEntry>,
}

const CATALOG_TOML: &str = include_str!("../data/user_agents.toml");

static CATALOG: LazyLock<Result<Catalog, InternalError>> = LazyLock::new(|| {
    toml::from_str(CATALOG_TOML).map_err(|e| InternalError::CatalogParse(e.to_string()))
});

impl BrowserProfile {
    /// Generates a browser profile from a seed.
    ///
    /// # Errors
    ///
    /// Returns [`InternalError::EmptyCatalog`] if the embedded user
    /// agent catalog contains no entries, or [`InternalError::CatalogParse`]
    /// if the catalog failed to parse.
    pub fn from_seed(seed: &[u8; 32]) -> Result<Self, InternalError> {
        let mut rng = ChaCha12Rng::from_seed(*seed);

        let catalog = CATALOG
            .as_ref()
            .map_err(|e| InternalError::CatalogParse(e.to_string()))?;
        if catalog.entries.is_empty() {
            return Err(InternalError::EmptyCatalog);
        }

        let entry = catalog
            .entries
            .choose(&mut rng)
            .ok_or(InternalError::EmptyCatalog)?;

        let viewports = [(1366, 768), (1536, 864), (1440, 900)];
        let viewport = *viewports
            .choose(&mut rng)
            .ok_or_else(|| InternalError::Coherence("viewport choice failed".into()))?;

        let depths = [24, 32];
        let color_depth = *depths
            .choose(&mut rng)
            .ok_or_else(|| InternalError::Coherence("depth choice failed".into()))?;

        let ratios = [1.0f32, 1.25, 1.5, 2.0];
        let pixel_ratio = *ratios
            .choose(&mut rng)
            .ok_or_else(|| InternalError::Coherence("ratio choice failed".into()))?;

        let languages = vec!["en-US".to_string(), "en".to_string()];

        Ok(Self {
            user_agent: entry.user_agent.clone(),
            viewport,
            color_depth,
            pixel_ratio,
            languages,
            platform: entry.platform.clone(),
            oscpu: entry.oscpu.clone(),
            product_sub: entry.product_sub.clone(),
            vendor: entry.vendor.clone(),
            webgl_vendor: entry.webgl_vendor.clone(),
            webgl_renderer: entry.webgl_renderer.clone(),
        })
    }
}
