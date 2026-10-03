//! Coherence validation tests.

use proptest::prop_assert;
use sgida_identity::error::InternalError;
use sgida_identity::validator;
use sgida_identity::Profile;
use sgida_ports::MasterSeed;

fn get_valid_profile() -> Profile {
    Profile::from_seed(MasterSeed::from_bytes([99u8; 32])).unwrap()
}

#[test]
fn rule_1_win32_requires_windows_ua() {
    let mut p = get_valid_profile();
    p.browser.platform = "Win32".to_string();
    p.browser.user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X)".to_string();
    let res = validator::validate(&p);
    assert!(matches!(res, Err(InternalError::Coherence(msg)) if msg.contains("rule 1")));
}

#[test]
fn rule_2_macintel_timezone_locale_mismatch() {
    let mut p = get_valid_profile();
    p.browser.platform = "MacIntel".to_string();
    p.system.locale = "ja-JP".to_string();
    p.system.timezone = "America/New_York".to_string();
    let res = validator::validate(&p);
    assert!(matches!(res, Err(InternalError::Coherence(msg)) if msg.contains("rule 2")));
}

#[test]
fn rule_3_invalid_hardware_concurrency() {
    let mut p = get_valid_profile();
    p.system.hardware_concurrency = 7;
    let res = validator::validate(&p);
    assert!(matches!(res, Err(InternalError::Coherence(msg)) if msg.contains("rule 3")));
}

#[test]
fn rule_4_invalid_device_memory() {
    let mut p = get_valid_profile();
    p.system.device_memory = 3.5;
    let res = validator::validate(&p);
    assert!(matches!(res, Err(InternalError::Coherence(msg)) if msg.contains("rule 4")));
}

#[test]
fn rule_5_viewport_larger_than_screen() {
    let mut p = get_valid_profile();
    p.browser.viewport = (3840, 2160);
    p.system.screen_resolution = (1920, 1080);
    let res = validator::validate(&p);
    assert!(matches!(res, Err(InternalError::Coherence(msg)) if msg.contains("rule 5")));
}

#[test]
fn rule_6_invalid_timezone_string() {
    let mut p = get_valid_profile();
    p.system.timezone = "Invalid/Timezone".to_string();
    let res = validator::validate(&p);
    assert!(matches!(res, Err(InternalError::Coherence(msg)) if msg.contains("rule 6")));
}

proptest::proptest! {
    #[test]
    fn proptest_always_valid(seed in proptest::collection::vec(proptest::arbitrary::any::<u8>(), 32)) {
        let mut seed_arr = [0u8; 32];
        seed_arr.copy_from_slice(&seed);
        let master = MasterSeed::from_bytes(seed_arr);
        let p = Profile::from_seed(master).unwrap();
        prop_assert!(validator::validate(&p).is_ok());
    }
}
