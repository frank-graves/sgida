// crates/sgida-identity/src/validator.rs
//! Coherence validators.

use chrono_tz::Tz;
use std::str::FromStr;

use crate::error::InternalError;
use crate::profile::Profile;

/// Validates the coherence of a profile.
pub fn validate(profile: &Profile) -> Result<(), InternalError> {
    // 1. If platform == "Win32", user_agent must contain "Windows".
    if profile.browser.platform == "Win32" && !profile.browser.user_agent.contains("Windows") {
        return Err(InternalError::Coherence(
            "rule 1: Win32 platform requires Windows in user agent".into(),
        ));
    }

    // 2. If platform == "MacIntel", timezone must be plausible for locale.
    if profile.browser.platform == "MacIntel" {
        let valid = match profile.system.locale.as_str() {
            "en-US" => !profile.system.timezone.starts_with("Asia/"),
            "es-ES" => {
                profile.system.timezone.starts_with("Europe/")
                    || profile.system.timezone.starts_with("Atlantic/")
            }
            "ja-JP" => profile.system.timezone == "Asia/Tokyo",
            _ => true,
        };
        if !valid {
            return Err(InternalError::Coherence(
                "rule 2: implausible timezone for locale".into(),
            ));
        }
    }

    // 3. hardware_concurrency must be in {2, 4, 8, 16}.
    if ![2, 4, 8, 16].contains(&profile.system.hardware_concurrency) {
        return Err(InternalError::Coherence(
            "rule 3: invalid hardware_concurrency".into(),
        ));
    }

    // 4. device_memory must be in {0.25, 0.5, 1.0, 2.0, 4.0, 8.0}.
    let valid_memories = [0.25f32, 0.5, 1.0, 2.0, 4.0, 8.0];
    if !valid_memories.contains(&profile.system.device_memory) {
        return Err(InternalError::Coherence(
            "rule 4: invalid device_memory".into(),
        ));
    }

    // 5. viewport.0 < screen_resolution.0 AND viewport.1 < screen_resolution.1.
    if profile.browser.viewport.0 >= profile.system.screen_resolution.0
        || profile.browser.viewport.1 >= profile.system.screen_resolution.1
    {
        return Err(InternalError::Coherence(
            "rule 5: viewport must be smaller than screen resolution".into(),
        ));
    }

    // 6. timezone string must parse with chrono_tz::Tz::from_str.
    if Tz::from_str(&profile.system.timezone).is_err() {
        return Err(InternalError::Coherence(
            "rule 6: invalid timezone string".into(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::Profile;
    use sgida_ports::MasterSeed;

    fn base() -> Profile {
        Profile::from_seed(MasterSeed::from_bytes([42u8; 32]))
            .unwrap_or_else(|e| panic!("fixture must build: {e}"))
    }

    // ---- rule 1: Win32 requires "Windows" in the UA ----------------------
    #[test]
    fn rule1_positive() {
        let mut p = base();
        p.browser.platform = "Win32".to_string();
        p.browser.user_agent =
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Firefox/128.0".to_string();
        assert!(validate(&p).is_ok());
    }

    #[test]
    fn rule1_negative() {
        let mut p = base();
        p.browser.platform = "Win32".to_string();
        p.browser.user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)".to_string();
        let err = validate(&p).unwrap_err();
        assert!(err.to_string().contains("rule 1"));
    }

    // ---- rule 2: MacIntel timezone must be plausible for locale ----------
    #[test]
    fn rule2_positive() {
        let mut p = base();
        p.browser.platform = "MacIntel".to_string();
        p.system.locale = "ja-JP".to_string();
        p.system.timezone = "Asia/Tokyo".to_string();
        assert!(validate(&p).is_ok());
    }

    #[test]
    fn rule2_negative() {
        let mut p = base();
        p.browser.platform = "MacIntel".to_string();
        p.system.locale = "ja-JP".to_string();
        p.system.timezone = "America/New_York".to_string();
        let err = validate(&p).unwrap_err();
        assert!(err.to_string().contains("rule 2"));
    }

    // ---- rule 3: hardware_concurrency in {2, 4, 8, 16} -------------------
    #[test]
    fn rule3_positive() {
        let mut p = base();
        p.system.hardware_concurrency = 8;
        assert!(validate(&p).is_ok());
    }

    #[test]
    fn rule3_negative() {
        let mut p = base();
        p.system.hardware_concurrency = 7;
        let err = validate(&p).unwrap_err();
        assert!(err.to_string().contains("rule 3"));
    }

    // ---- rule 4: device_memory in {0.25, 0.5, 1, 2, 4, 8} ----------------
    #[test]
    fn rule4_positive() {
        let mut p = base();
        p.system.device_memory = 4.0;
        assert!(validate(&p).is_ok());
    }

    #[test]
    fn rule4_negative() {
        let mut p = base();
        p.system.device_memory = 3.5;
        let err = validate(&p).unwrap_err();
        assert!(err.to_string().contains("rule 4"));
    }

    // ---- rule 5: viewport strictly smaller than screen -------------------
    #[test]
    fn rule5_positive() {
        let mut p = base();
        p.browser.viewport = (1280, 720);
        p.system.screen_resolution = (1920, 1080);
        assert!(validate(&p).is_ok());
    }

    #[test]
    fn rule5_negative() {
        let mut p = base();
        p.browser.viewport = (1920, 1080);
        p.system.screen_resolution = (1920, 1080);
        let err = validate(&p).unwrap_err();
        assert!(err.to_string().contains("rule 5"));
    }

    // ---- rule 6: timezone must parse via chrono_tz -----------------------
    #[test]
    fn rule6_positive() {
        let mut p = base();
        p.system.timezone = "Europe/Madrid".to_string();
        assert!(validate(&p).is_ok());
    }

    #[test]
    fn rule6_negative() {
        let mut p = base();
        p.system.timezone = "Invalid/Timezone".to_string();
        let err = validate(&p).unwrap_err();
        assert!(err.to_string().contains("rule 6"));
    }
}
