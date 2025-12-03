/// # CEP Core
///
/// Core primitives for the Civic Exchange Protocol (CEP).
///
/// This crate provides the foundational types and traits used by all CEP record types:
///
/// - [`CanonicalTimestamp`]: Microsecond-precision UTC timestamps
/// - [`CanonicalHash`]: SHA-256 hash values
/// - [`Canonicalize`]: Trait for deterministic serialization
/// - [`Attestation`]: Cryptographic proof of record integrity
///
/// ## Canonicalization
///
/// All CEP records must be serializable to a deterministic canonical string for hashing.
/// This ensures that the same logical record produces the same hash across all implementations
/// (Rust, Python, Java, C#, TypeScript, Go).
///
/// ```rust
/// use cep_core::canonical::Canonicalize;
/// use cep_core::hash::CanonicalHash;
///
/// // Any type implementing Canonicalize can be hashed
/// // let hash = my_record.calculate_hash();
/// ```
///
pub mod assets;
pub mod attestation;
pub mod canonical;
pub mod error;
pub mod hash;
pub mod schema_registry;
pub mod timestamp;
pub mod version;

// Re-export primary types
pub use assets::{get_schema, get_vocab, get_test_vector};
pub use attestation::{Attestation, ProofPurpose};
pub use canonical::Canonicalize;
pub use error::{CepError, CepResult};
pub use hash::CanonicalHash;
pub use schema_registry::{find_repo_root, SchemaRegistry};
pub use timestamp::CanonicalTimestamp;
pub use version::SCHEMA_VERSION;
