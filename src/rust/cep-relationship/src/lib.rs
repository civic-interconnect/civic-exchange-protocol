//! # CEP Relationship
//!
//! Relationship records for the Civic Exchange Protocol (CEP).
//!
//! This crate defines the [`RelationshipRecord`] type, 
//! which represents a verifiable legal or functional
//! relationship between two or more attested entities.
//!
//! ## Bilateral Relationships
//!
//! Two-party relationships with clear directionality:
//!
//! ```rust
//! use cep_relationship::{RelationshipRecord, BilateralParties, Party, RelationshipStatus, RelationshipStatusCode};
//! use cep_core::{Attestation, Canonicalize};
//!
//! let parties = BilateralParties::new(
//!     Party::new("cep-entity:sam-uei:AGENCY123".to_string(), "role-uri".to_string()),
//!     Party::new("cep-entity:sam-uei:VENDOR456".to_string(), "role-uri".to_string()),
//! );
//!
//! // ... create relationship with parties
//! ```
//!
//! ## Multilateral Relationships
//!
//! N-ary relationships with deterministic member ordering:
//!
//! ```rust
//! use cep_relationship::{MultilateralMembers, Member};
//!
//! let mut members = MultilateralMembers::new();
//! members.add(Member::new("entity-a".to_string(), "role".to_string()));
//! members.add(Member::new("entity-b".to_string(), "role".to_string()));
//! // Members are automatically sorted by entity_id for hash stability
//! ```

pub mod bilateral;
pub mod builder;
pub mod multilateral;
pub mod relationship;

// Re-export primary types
pub use bilateral::{BilateralParties, Party};
pub use builder::{
    build_relationship, AttestationInput as RelationshipAttestationInput,
    FinancialTermsInput, RelationshipBuildResult, RelationshipBuilderInput,
    SourceReferenceInput as RelationshipSourceReferenceInput, relationship_type_uri,
};
pub use multilateral::{Member, MultilateralMembers};
pub use relationship::{
    FinancialTerms, Parties, RelationshipRecord, RelationshipStatus, RelationshipStatusCode,
    SourceReference,
};