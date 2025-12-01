//! # CEP Exchange
//!
//! Exchange records for the Civic Exchange Protocol (CEP).
//!
//! This crate defines the [`ExchangeRecord`] type, which represents a verifiable
//! value exchange (financial, in-kind, or informational) between entities within
//! an established relationship. This is the atomic unit of civic transparency.
//!
//! ## Example
//!
//! ```rust
//! use cep_exchange::{ExchangeRecord, ExchangeStatus, ExchangeStatusCode};
//! use cep_exchange::value::{ExchangeParty, ExchangeValue};
//! use cep_exchange::provenance::{ProvenanceChain, ExchangeCategorization};
//! use cep_core::{Attestation, Canonicalize};
//!
//! // Create source and recipient parties
//! let source = ExchangeParty::new("cep-entity:sam-uei:AGENCY12345A".to_string());
//! let recipient = ExchangeParty::new("cep-entity:sam-uei:SCHOOL67890B".to_string());
//!
//! // Create value
//! let value = ExchangeValue::usd(50000.00);
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
//! // Create status
//! let status = ExchangeStatus {
//!     status_code: ExchangeStatusCode::Completed,
//!     status_effective_timestamp: "2025-09-15T14:03:22.500000Z".parse().unwrap(),
//! };
//!
//! // Create exchange record
//! let exchange = ExchangeRecord::new(
//!     "cep-exchange:treasury:PAY_2025_001234".to_string(),
//!     "cep-relationship:usaspending:GRANT_84010_2025".to_string(),
//!     "https://example.com/exchange-type/grant-disbursement".to_string(),
//!     source,
//!     recipient,
//!     value,
//!     "2025-09-15T14:03:22.500000Z".parse().unwrap(),
//!     status,
//!     attestation,
//! );
//!
//! // Generate canonical hash
//! let hash = exchange.calculate_hash();
//! println!("Exchange hash: {}", hash);
//! ```

pub mod builder;
pub mod exchange;
pub mod provenance;
pub mod value;

// Re-export primary types
pub use builder::{
    AttestationInput, CategorizationInput, ExchangeBuildResult, ExchangeBuilderInput,
    SourceReferenceInput, build_exchange, exchange_type_uri, parse_timestamp,
};
pub use exchange::{ExchangeRecord, ExchangeStatus, ExchangeStatusCode, SourceReference};
pub use provenance::{ExchangeCategorization, IntermediaryEntity, ProvenanceChain};
pub use value::{ExchangeParty, ExchangeValue, ValueType};
