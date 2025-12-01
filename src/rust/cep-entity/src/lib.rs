//! # CEP Entity
//!
//! Entity records for the Civic Exchange Protocol (CEP).
//!
//! This crate defines the [`EntityRecord`] type, which represents a verified
//! civic entity. Entities are the foundational primitive in CEP—all relationships
//! and exchanges reference attested entities.
//!
//! ## Example
//!
//! ```rust
//! use cep_entity::{EntityRecord, EntityIdentifiers, EntityStatus, EntityStatusCode};
//! use cep_entity::identifiers::SamUei;
//! use cep_core::{Attestation, Canonicalize};
//!
//! // Create identifiers
//! let identifiers = EntityIdentifiers::new()
//!     .with_sam_uei(SamUei::new("J6H4FB3N5YK7").unwrap());
//!
//! // Create status
//! let status = EntityStatus {
//!     status_code: EntityStatusCode::Active,
//!     status_effective_date: "2020-01-15".to_string(),
//!     status_termination_date: None,
//!     successor_entity_id: None,
//! };
//!
//! // Create attestation
//! let attestation = Attestation::new(
//!     "cep-entity:sam-uei:ATTESTOR123A".to_string(),
//!     "2025-11-28T14:30:00.000000Z".parse().unwrap(),
//!     "Ed25519Signature2020".to_string(),
//!     "z3FXQq...".to_string(),
//!     "did:web:example.gov#key-1".to_string(),
//! );
//!
//! // Create entity record
//! let entity = EntityRecord::new(
//!     "cep-entity:sam-uei:J6H4FB3N5YK7".to_string(),
//!     identifiers,
//!     "Acme Consulting LLC".to_string(),
//!     "US-CA".to_string(),
//!     status,
//!     attestation,
//! );
//!
//! // Generate canonical hash
//! let hash = entity.calculate_hash();
//! println!("Entity hash: {}", hash);
//! ```

pub mod builder;
pub mod entity;
pub mod identifiers;

// Re-export primary types
pub use builder::{
    AttestationInput, EntityBuildResult, EntityBuilderInput, EntityTypeCode, build_entity,
};
pub use entity::{EntityRecord, EntityStatus, EntityStatusCode, ResolutionConfidence};
pub use identifiers::{AdditionalScheme, CanadianBn, EntityIdentifiers, Lei, SamUei, Snfei};
