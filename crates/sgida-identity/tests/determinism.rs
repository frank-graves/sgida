#![allow(clippy::unwrap_used)]
//! Determinism tests for sgida-identity.

use proptest::prop_assert_eq;
use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;
use sgida_identity::Profile;
use sgida_ports::MasterSeed;

#[test]
fn rng_determinism_property() {
    let seed = [42u8; 32];
    let mut rng1 = ChaCha12Rng::from_seed(seed);
    let mut rng2 = ChaCha12Rng::from_seed(seed);

    for _ in 0..100 {
        assert_eq!(rng1.next_u64(), rng2.next_u64());
    }
}

#[test]
fn profile_determinism_same_seed() {
    let p1 = Profile::from_seed(MasterSeed::from_bytes([1u8; 32])).unwrap();
    let p2 = Profile::from_seed(MasterSeed::from_bytes([1u8; 32])).unwrap();

    assert_eq!(p1.id, p2.id);
    assert_eq!(p1.browser.user_agent, p2.browser.user_agent);
    assert_eq!(p1.system.timezone, p2.system.timezone);
    assert_eq!(p1.credentials.username(), p2.credentials.username());
}

#[test]
fn profile_determinism_different_seeds() {
    let p1 = Profile::from_seed(MasterSeed::from_bytes([1u8; 32])).unwrap();
    let p2 = Profile::from_seed(MasterSeed::from_bytes([2u8; 32])).unwrap();

    assert_ne!(p1.id, p2.id);
    assert_ne!(p1.browser.user_agent, p2.browser.user_agent);
}

#[test]
fn profile_id_is_derived() {
    let p1 = Profile::from_seed(MasterSeed::from_bytes([3u8; 32])).unwrap();
    let p2 = Profile::from_seed(MasterSeed::from_bytes([3u8; 32])).unwrap();
    assert_eq!(p1.id, p2.id);
}

proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig::with_cases(10_000))]

    #[test]
    fn proptest_determinism(seed in proptest::collection::vec(proptest::arbitrary::any::<u8>(), 32)) {
        let mut seed_arr = [0u8; 32];
        seed_arr.copy_from_slice(&seed);
        let master = MasterSeed::from_bytes(seed_arr);

        let p1 = Profile::from_seed(master).unwrap();
        let p2 = Profile::from_seed(master).unwrap();

        prop_assert_eq!(p1.id, p2.id);
        prop_assert_eq!(p1.browser.user_agent, p2.browser.user_agent);
    }
}
