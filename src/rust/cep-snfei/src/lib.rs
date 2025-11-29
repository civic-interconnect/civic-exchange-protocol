//! CEP SNFEI: Entity Resolution and SNFEI Generation.
//!
//! This crate implements the Normalizing Functor architecture for generating
//! deterministic entity identifiers (SNFEIs) from heterogeneous source data.
//!
//! # Architecture
//!
//! ```text
//!     ┌──────────────┐     ┌────────────────┐     ┌─────────────┐
//!     │  Raw Entity  │     │  Intermediate  │     │  Canonical  │
//!     │    Data      │────>│    Canonical   │────>│   Entity    │
//!     │              │  L  │                │  N  │             │
//!     └──────────────┘     └────────────────┘     └─────────────┘
//!                                                         │
//!                                                         │ SHA-256
//!                                                         V
//!                                                 ┌──────────────┐
//!                                                 │    SNFEI     │
//!                                                 │  (64-char)   │
//!                                                 └──────────────┘
//!
//!     L = Localization Functor (jurisdiction-specific transforms)
//!     N = Normalizing Functor (universal normalization)
//! ```
//!
//! # Usage
//!
//! ```rust
//! use cep_snfei::{generate_snfei, normalize_legal_name, apply_localization};
//!
//! // Simple SNFEI generation
//! let (snfei, inputs) = generate_snfei(
//!     "Springfield USD #12",
//!     "US",
//!     Some("123 Main St"),
//!     None,
//! );
//!
//! // With jurisdiction-specific localization
//! let localized = apply_localization("MTA", "US/NY");
//! assert_eq!(localized, "metropolitan transportation authority");
//! ```

mod generator;
mod localization;
mod normalizer;

// Re-export generator types
pub use generator::{
    compute_snfei,
    generate_snfei,
    generate_snfei_simple,
    generate_snfei_with_confidence,
    Snfei,
    SnfeiResult,
};

// Re-export normalization types
pub use normalizer::{
    build_canonical_input,
    normalize_address,
    normalize_legal_name,
    normalize_registration_date,
    CanonicalInput,
};

// Re-export localization types
pub use localization::{
    apply_localization,
    get_localization_config,
    LocalizationConfig,
    LocalizationRegistry,
    LocalizationRule,
};