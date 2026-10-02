use std::fmt;

/// Master seed used for deterministic identity generation.
///
/// The `Debug` implementation deliberately hides the seed bytes to prevent
/// accidental leakage into logs, traces, or error messages. The type is
/// intentionally not `Serialize` or `Deserialize`: persisting a master seed
/// would defeat the purpose of keeping it ephemeral.
#[derive(Clone, Copy)]
pub struct MasterSeed([u8; 32]);

impl MasterSeed {
    /// Creates a master seed from its raw byte representation.
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// # SECURITY
    /// Exposes the raw seed. Only `sgida-identity` is expected to call this.
    /// Never log, print, or persist the return value.
    #[doc(hidden)]
    pub fn expose_to_provider(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for MasterSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MasterSeed").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::MasterSeed;

    #[test]
    fn debug_does_not_reveal_seed_bytes() {
        let seed = MasterSeed::from_bytes([0xA5; 32]);
        let rendered = format!("{seed:?}");

        assert!(!rendered.contains("a5"));
        assert!(!rendered.contains("A5"));
        assert!(!rendered.contains("165"));
    }
}
