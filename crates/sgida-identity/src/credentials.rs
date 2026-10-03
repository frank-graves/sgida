// crates/sgida-identity/src/credentials.rs
//! Synthetic credentials.

use chrono::NaiveDate;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha12Rng;
use secrecy::{ExposeSecret, SecretString};
use std::fmt;

/// Synthetic credentials. Never serialized or leaked.
#[derive(Clone)]
pub struct Credentials {
    username: String,
    password: SecretString,
    email_local: String,
    birth_date: NaiveDate,
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &"<redacted>")
            .field("password", &"<redacted>")
            .field("email_local", &"<redacted>")
            .field("birth_date", &"<redacted>")
            .finish()
    }
}

impl Credentials {
    /// Generates credentials from a seed.
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let mut rng = ChaCha12Rng::from_seed(*seed);

        let username = format!("user_{}", rng.r#gen::<u32>());
        let password = SecretString::new(format!("pass_{}", rng.r#gen::<u64>()));
        let email_local = format!("user_{}", rng.r#gen::<u32>());

        let year = rng.gen_range(1980..=2000i32);
        let month = rng.gen_range(1..=12u32);
        let day = rng.gen_range(1..=28u32);

        // day in 1..=28 and month in 1..=12 are invariants from the
        // RNG, so from_ymd_opt cannot return None for these inputs.
        // If it somehow does, NaiveDate::MIN is used as an obviously-
        // invalid date so that if this fallback were ever hit, the
        // result would be visible on inspection.
        let birth_date = NaiveDate::from_ymd_opt(year, month, day).unwrap_or(NaiveDate::MIN);

        Self {
            username,
            password,
            email_local,
            birth_date,
        }
    }

    /// Returns the username.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Returns the email local part.
    pub fn email_local(&self) -> &str {
        &self.email_local
    }

    /// Returns the birth date.
    pub fn birth_date(&self) -> NaiveDate {
        self.birth_date
    }

    /// Exposes the password.
    ///
    /// # Security
    /// This method exposes the secret password. It should only be used
    /// when absolutely necessary, such as when filling a login form.
    #[doc(hidden)]
    pub fn expose_password(&self) -> &str {
        self.password.expose_secret()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_does_not_leak_identifiers() {
        let creds = Credentials::from_seed(&[7u8; 32]);
        let rendered = format!("{creds:?}");
        assert!(
            !rendered.contains(creds.username()),
            "Debug output leaked username: {rendered}"
        );
        assert!(
            !rendered.contains(creds.email_local()),
            "Debug output leaked email_local: {rendered}"
        );
        assert!(
            !rendered.contains(creds.expose_password()),
            "Debug output leaked password: {rendered}"
        );
    }

    #[test]
    fn accessors_are_stable_across_calls() {
        let creds = Credentials::from_seed(&[7u8; 32]);
        assert_eq!(creds.username(), creds.username());
        assert_eq!(creds.email_local(), creds.email_local());
        assert_eq!(creds.birth_date(), creds.birth_date());
    }
}
