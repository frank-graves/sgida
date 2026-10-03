// crates/sgida-identity/src/lib.rs
#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! sgida-identity: deterministic synthetic profile generation.

pub mod behavior;
pub mod browser;
pub mod canvas;
pub mod credentials;
pub mod error;
pub mod network;
pub mod profile;
pub mod provider;
pub mod seed;
pub mod system;
pub mod validator;

pub use profile::Profile;
pub use provider::InMemoryIdentityProvider;
