//! # CEP Core
//!
//! Core primitives for the Civic Exchange Protocol (CEP).
//!
//! This crate provides the foundational types and traits used by all CEP record types:
//!
//! - [`CanonicalTimestamp`]: Microsecond-precision UTC timestamps
//! - [`CanonicalHash`]: SHA-256 hash values
//! - [`Canonicalize`]: Trait for deterministic serialization
//! - [`Attestation`]: Cryptographic proof of record integrity
//!
//! ## Canonicalization
//!
//! All CEP records must be serializable to a deterministic canonical string for hashing.
//! This ensures that the same logical record produces the same hash across all implementations
//! (Rust, Python, Java, C#, TypeScript, Go).
//!
//! ```rust
//! use cep_core::canonical::Canonicalize;
//! use cep_core::hash::CanonicalHash;
//!
//! // Any type implementing Canonicalize can be hashed
//! // let hash = my_record.calculate_hash();
//! ```
//!
//! ## Schema Version
//!
//! The current schema version for all CEP records.
pub const SCHEMA_VERSION: &str = "1.0.0";

pub mod attestation;
pub mod canonical;
pub mod error;
pub mod hash;
pub mod timestamp;

// Re-export primary types
pub use attestation::{Attestation, ProofPurpose};
pub use canonical::Canonicalize;
pub use error::{CepError, CepResult};
pub use hash::CanonicalHash;
pub use timestamp::CanonicalTimestamp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_version() {
        assert_eq!(SCHEMA_VERSION, "1.0.0");
    }
}