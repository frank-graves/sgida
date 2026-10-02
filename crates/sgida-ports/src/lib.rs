#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(
    test,
    allow(missing_docs, reason = "generated mocks and tests are not public API")
)]
#![cfg_attr(
    test,
    allow(
        clippy::all,
        clippy::pedantic,
        clippy::nursery,
        reason = "test and mockall-generated code are verified by compilation"
    )
)]
#![allow(
    clippy::doc_markdown,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    reason = "port contracts use fixed domain wording and ownership-based signatures"
)]
//! SGIDA ports contains only trait contracts and shared types.
//!
//! This crate defines the stable boundaries between SGIDA modules. It contains
//! no business logic, no I/O, and no mutable state. Concrete implementations
//! live in downstream crates and are wired together by the `sgidad` binary.

mod error;
mod handle;
mod identity;
mod isolation;
mod seed;
mod state;

pub use error::{IdentityError, IsolationError, PortError, StateError};
pub use handle::{EnvironmentHandle, ProfileHandle};
pub use identity::IdentityProvider;
pub use isolation::{
    ContainerSpec, ExecOutput, Health, IsolationDriver, Mount, NetworkMode, ResourceLimits,
};
pub use seed::MasterSeed;
pub use state::{IdentityState, Label, LabelKey, StateSnapshot, StateStore};
