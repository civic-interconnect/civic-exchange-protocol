//! SNFEI Hash Generation.
//!
//! This module computes the final SNFEI (Sub-National Federated Entity Identifier)
//! from normalized entity attributes.
//!
//! The SNFEI formula:
//!     SNFEI = SHA256(Concatenate[
//!         legal_name_normalized,
//!         address_normalized,
//!         country_code,
//!         registration_date
//!     ])
//!
//! All inputs must pass through the Normalizing Functor before hashing.

use sha2::{Digest, Sha256};

use crate::normalizer::{build_canonical_input, CanonicalInput};
use serde::{Deserialize, Serialize};

/// A validated SNFEI (64-character lowercase hex string).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Snfei {
    value: String,
}

impl Snfei {
    /// Create from an existing hash string.
    pub fn from_hash(hash: &str) -> Option<Self> {
        if hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
            // FIX: Use struct literal syntax
            Some(Self {
                value: hash.to_lowercase(),
            })
        } else {
            None
        }
    }

    /// Get the hash value.
    pub fn value(&self) -> &str {
        // FIX: Access the named field .value
        &self.value
    }

    /// Get a shortened version for display.
    pub fn short(&self, length: usize) -> String {
        // FIX: Access the named field .value
        if self.value.len() <= length {
            // FIX: Access the named field .value
            self.value.clone()
        } else {
            // FIX: Access the named field .value
            format!("{}...", &self.value[..length])
        }
    }
}

impl std::fmt::Display for Snfei {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Compute SNFEI from canonical input.
pub fn compute_snfei(canonical: &CanonicalInput) -> Snfei {
    let hash_input = canonical.to_hash_string();
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    let result = hasher.finalize();
    Snfei {
        value: format!("{:x}", result),
    }
}

/// Generate an SNFEI from raw entity attributes.
///
/// This is the main entry point for SNFEI generation. It applies the
/// Normalizing Functor to all inputs before hashing.
///
/// # Arguments
/// * `legal_name` - Raw legal name from source system
/// * `country_code` - ISO 3166-1 alpha-2 country code
/// * `address` - Optional primary street address
/// * `registration_date` - Optional formation/registration date
///
/// # Returns
/// Tuple of (SNFEI, CanonicalInput) for verification
///
/// # Example
/// ```
/// use cep_snfei::generate_snfei;
///
/// let (snfei, inputs) = generate_snfei(
///     "Springfield USD #12",
///     "US",
///     Some("123 Main St"),
///     None,
/// );
/// println!("SNFEI: {}", snfei);
/// println!("Normalized name: {}", inputs.legal_name_normalized);
/// ```
pub fn generate_snfei(
    legal_name: &str,
    country_code: &str,
    address: Option<&str>,
    registration_date: Option<&str>,
) -> (Snfei, CanonicalInput) {
    let canonical = build_canonical_input(legal_name, country_code, address, registration_date);
    let snfei = compute_snfei(&canonical);
    (snfei, canonical)
}

/// Generate SNFEI as a simple hex string.
///
/// Convenience function that returns just the hash value.
///
/// # Arguments
/// * `legal_name` - Raw legal name
/// * `country_code` - ISO 3166-1 alpha-2 country code
/// * `address` - Optional primary street address
///
/// # Returns
pub fn generate_snfei_simple(
    legal_name: &str,
    country_code: &str,
    address: Option<&str>,
) -> String {
    let (snfei, _) = generate_snfei(legal_name, country_code, address, None);
    snfei.value
}


// =============================================================================
// CONFIDENCE SCORING
// =============================================================================

/// Result of SNFEI generation with confidence metadata.
#[derive(Debug, Clone)]
pub struct SnfeiResult {
    /// The generated SNFEI
    pub snfei: Snfei,
    /// Canonical inputs used
    pub canonical: CanonicalInput,
    /// Confidence score (0.0 to 1.0)
    pub confidence_score: f64,
    /// Identifier tier (1, 2, or 3)
    pub tier: u8,
    /// Which fields contributed to the SNFEI
    pub fields_used: Vec<String>,
}

impl SnfeiResult {
    /// Convert to a simple dictionary-like structure.
    pub fn to_map(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        map.insert("snfei".to_string(), self.snfei.value().to_string());
        map.insert(
            "confidence_score".to_string(),
            format!("{:.2}", self.confidence_score),
        );
        map.insert("tier".to_string(), self.tier.to_string());
        map.insert("fields_used".to_string(), self.fields_used.join(","));
        map.insert(
            "canonical_string".to_string(),
            self.canonical.to_hash_string(),
        );
        map
    }
}

/// Generate SNFEI with confidence scoring and tier classification.
///
/// # Tier Classification
/// - Tier 1: Entity has LEI (global identifier) - confidence 1.0
/// - Tier 2: Entity has SAM UEI (federal identifier) - confidence 0.95
/// - Tier 3: Entity uses SNFEI (computed hash) - confidence varies
///
/// # Tier 3 Confidence Scoring
/// - Base: 0.5 (name + country only)
/// - +0.2 if address is provided
/// - +0.2 if registration_date is provided
/// - +0.1 if name is reasonably long (>3 words)
/// - Capped at 0.9 for Tier 3
///
/// # Arguments
/// * `legal_name` - Raw legal name
/// * `country_code` - ISO 3166-1 alpha-2 country code
/// * `address` - Optional primary street address
/// * `registration_date` - Optional formation/registration date
/// * `lei` - Optional LEI (Legal Entity Identifier)
/// * `sam_uei` - Optional SAM.gov Unique Entity Identifier
///
/// # Returns
/// SnfeiResult with SNFEI, confidence score, and metadata
pub fn generate_snfei_with_confidence(
    legal_name: &str,
    country_code: &str,
    address: Option<&str>,
    registration_date: Option<&str>,
    lei: Option<&str>,
    sam_uei: Option<&str>,
) -> SnfeiResult {
    let mut fields_used = vec!["legal_name".to_string(), "country_code".to_string()];

    // Tier 1: LEI available
    if let Some(lei_value) = lei {
        if lei_value.len() == 20 {
            let canonical =
                build_canonical_input(legal_name, country_code, address, registration_date);
            let snfei = compute_snfei(&canonical);
            fields_used.insert(0, "lei".to_string());
            return SnfeiResult {
                snfei,
                canonical,
                confidence_score: 1.0,
                tier: 1,
                fields_used,
            };
        }
    }

    // Tier 2: SAM UEI available
    if let Some(uei_value) = sam_uei {
        if uei_value.len() == 12 {
            let canonical =
                build_canonical_input(legal_name, country_code, address, registration_date);
            let snfei = compute_snfei(&canonical);
            fields_used.insert(0, "sam_uei".to_string());
            return SnfeiResult {
                snfei,
                canonical,
                confidence_score: 0.95,
                tier: 2,
                fields_used,
            };
        }
    }

    // Tier 3: Compute SNFEI from attributes
    let canonical = build_canonical_input(legal_name, country_code, address, registration_date);
    let snfei = compute_snfei(&canonical);

    // Calculate confidence score
    let mut confidence: f64 = 0.5; // Base score

    if address.is_some() {
        fields_used.push("address".to_string());
        confidence += 0.2;
    }

    if registration_date.is_some() {
        fields_used.push("registration_date".to_string());
        confidence += 0.2;
    }

    // Bonus for longer, more specific names
    let word_count = canonical.legal_name_normalized.split_whitespace().count();
    if word_count > 3 {
        confidence += 0.1;
    }

    // Cap at 0.9 for Tier 3
    confidence = confidence.min(0.9);

    SnfeiResult {
        snfei,
        canonical,
        confidence_score: confidence,
        tier: 3,
        fields_used,
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snfei_from_hash() {
        let valid = "a".repeat(64);
        assert!(Snfei::from_hash(&valid).is_some());

        let invalid_length = "a".repeat(63);
        assert!(Snfei::from_hash(&invalid_length).is_none());

        let invalid_chars = "g".repeat(64);
        assert!(Snfei::from_hash(&invalid_chars).is_none());
    }

    #[test]
    fn test_generate_snfei_deterministic() {
        let (snfei1, _) = generate_snfei("Springfield USD", "US", None, None);
        let (snfei2, _) = generate_snfei("Springfield USD", "US", None, None);
        assert_eq!(snfei1, snfei2);
    }

    #[test]
    fn test_generate_snfei_normalization_equivalence() {
        let (snfei1, _) = generate_snfei("Springfield USD", "US", None, None);
        let (snfei2, _) = generate_snfei("SPRINGFIELD USD", "US", None, None);
        let (snfei3, _) = generate_snfei("springfield usd", "US", None, None);
        assert_eq!(snfei1, snfei2);
        assert_eq!(snfei2, snfei3);
    }

    #[test]
    fn test_generate_snfei_with_address() {
        let (snfei1, _) = generate_snfei("Acme Corp", "US", None, None);
        let (snfei2, _) = generate_snfei("Acme Corp", "US", Some("123 Main St"), None);
        assert_ne!(snfei1, snfei2);
    }

    #[test]
    fn test_generate_snfei_with_date() {
        let (snfei1, _) = generate_snfei("Acme Corp", "US", None, None);
        let (snfei2, _) = generate_snfei("Acme Corp", "US", None, Some("2020-01-01"));
        assert_ne!(snfei1, snfei2);
    }

    #[test]
    fn test_snfei_result_tier_3_base() {
        let result = generate_snfei_with_confidence("Acme Corp", "US", None, None, None, None);
        assert_eq!(result.tier, 3);
        assert!((result.confidence_score - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_snfei_result_tier_3_with_address() {
        let result = generate_snfei_with_confidence(
            "Acme Corp",
            "US",
            Some("123 Main St"),
            None,
            None,
            None,
        );
        assert_eq!(result.tier, 3);
        assert!((result.confidence_score - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_snfei_result_tier_3_max() {
        let result = generate_snfei_with_confidence(
            "Springfield Regional Medical Center Inc",
            "US",
            Some("500 Hospital Dr"),
            Some("1990-01-01"),
            None,
            None,
        );
        assert_eq!(result.tier, 3);
        assert!((result.confidence_score - 0.9).abs() < 0.01);
    }

    #[test]
    fn test_snfei_result_tier_2() {
        let result = generate_snfei_with_confidence(
            "Acme Corp",
            "US",
            None,
            None,
            None,
            Some("ABC123456789"),
        );
        assert_eq!(result.tier, 2);
        assert!((result.confidence_score - 0.95).abs() < 0.01);
    }

    #[test]
    fn test_snfei_result_tier_1() {
        let result = generate_snfei_with_confidence(
            "Acme Corp",
            "US",
            None,
            None,
            Some("12345678901234567890"),
            None,
        );
        assert_eq!(result.tier, 1);
        assert!((result.confidence_score - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_canonical_string_format() {
        let (_, canonical) =
            generate_snfei("Acme Corp", "US", Some("123 Main St"), Some("2020-01-01"));
        let hash_string = canonical.to_hash_string();
        assert!(hash_string.contains("|"));
        let parts: Vec<&str> = hash_string.split('|').collect();
        assert_eq!(parts.len(), 4);
    }
}
