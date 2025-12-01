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

use crate::normalizer::{CanonicalInput, build_canonical_input};
use serde::{Deserialize, Serialize};

/// A validated SNFEI (64-character lowercase hex string).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Snfei {
    pub value: String,
}

impl Snfei {
    /// Create from an existing hash string.
    pub fn from_hash(hash: &str) -> Option<Self> {
        if hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
            Some(Self {
                value: hash.to_lowercase(),
            })
        } else {
            None
        }
    }

    /// Get the hash value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get a shortened version for display.
    pub fn short(&self, length: usize) -> String {
        if self.value.len() <= length {
            self.value.clone()
        } else {
            format!("{}...", &self.value[..length])
        }
    }
}

impl std::fmt::Display for Snfei {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Result of SNFEI generation with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnfeiResult {
    /// The generated SNFEI
    pub snfei: Snfei,
    /// Canonical input used for generation
    pub canonical: CanonicalInput,
    /// Confidence score (0.0 to 1.0)
    pub confidence_score: f64,
    /// Tier classification (1, 2, or 3)
    pub tier: u8,
    /// Fields that contributed to the SNFEI
    pub fields_used: Vec<String>,
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
/// `SnfeiResult` containing the SNFEI, canonical input, and metadata
///
/// # Example
/// ```
/// use cep_snfei::generate_snfei;
///
/// let result = generate_snfei(
///     "Springfield USD #12",
///     "US",
///     Some("123 Main St"),
///     None,
/// );
/// println!("SNFEI: {}", result.snfei);
/// println!("Normalized name: {}", result.canonical.legal_name_normalized);
/// println!("Confidence: {}", result.confidence_score);
/// ```
pub fn generate_snfei(
    legal_name: &str,
    country_code: &str,
    address: Option<&str>,
    registration_date: Option<&str>,
) -> SnfeiResult {
    let canonical = build_canonical_input(legal_name, country_code, address, registration_date);
    let snfei = compute_snfei(&canonical);

    // Determine fields used
    let mut fields_used = vec!["legal_name".to_string(), "country_code".to_string()];
    if !canonical
        .address_normalized
        .as_deref()
        .unwrap_or("")
        .is_empty()
    {
        fields_used.push("address".to_string());
    }
    if !canonical.registration_date.as_deref().unwrap_or("").is_empty() {
        fields_used.push("registration_date".to_string());
    }

    // Calculate confidence score (Tier 3 logic)
    let mut confidence: f64 = 0.5; // Base score
    if !canonical.address_normalized.is_empty() {
        confidence += 0.2;
    }
    if !canonical.registration_date.is_empty() {
        confidence += 0.2;
    }
    // Bonus for longer, more specific names
    let word_count = canonical.legal_name_normalized.split_whitespace().count();
    if word_count > 3 {
        confidence += 0.1;
    }
    // Cap at 0.9 for Tier 3
    confidence = confidence.min(0.9);

    // ensure confidence is between 0.0 and 1.0
    if confidence < 0.0 {
        confidence = 0.0;
    } else if confidence > 1.0 {
        confidence = 1.0;
    }

    SnfeiResult {
        snfei,
        canonical,
        confidence_score: (confidence * 100.0).round() / 100.0,
        tier: 3,
        fields_used,
    }
}

/// Generate SNFEI as a simple hex string.
///
/// Convenience function that returns just the hash value.
pub fn generate_snfei_simple(
    legal_name: &str,
    country_code: &str,
    address: Option<&str>,
) -> String {
    let result = generate_snfei(legal_name, country_code, address, None);
    result.snfei.value
}

/// Generate SNFEI with confidence scoring and tier classification.
///
/// Tier Classification:
/// - Tier 1: Entity has LEI (global identifier) - confidence 1.0
/// - Tier 2: Entity has SAM UEI (federal identifier) - confidence 0.95
/// - Tier 3: Entity uses SNFEI (computed hash) - confidence varies
pub fn generate_snfei_with_confidence(
    legal_name: &str,
    country_code: &str,
    address: Option<&str>,
    registration_date: Option<&str>,
    lei: Option<&str>,
    sam_uei: Option<&str>,
) -> SnfeiResult {
    let canonical = build_canonical_input(legal_name, country_code, address, registration_date);
    let snfei = compute_snfei(&canonical);

    // Tier 1: LEI available
    if let Some(lei_val) = lei {
        if lei_val.len() == 20 {
            return SnfeiResult {
                snfei,
                canonical,
                confidence_score: 1.0,
                tier: 1,
                fields_used: vec![
                    "lei".to_string(),
                    "legal_name".to_string(),
                    "country_code".to_string(),
                ],
            };
        }
    }

    // Tier 2: SAM UEI available
    if let Some(uei_val) = sam_uei {
        if uei_val.len() == 12 {
            return SnfeiResult {
                snfei,
                canonical,
                confidence_score: 0.95,
                tier: 2,
                fields_used: vec![
                    "sam_uei".to_string(),
                    "legal_name".to_string(),
                    "country_code".to_string(),
                ],
            };
        }
    }

    // Tier 3: Computed SNFEI
    generate_snfei(legal_name, country_code, address, registration_date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snfei_generation() {
        let result = generate_snfei("Springfield School District", "US", None, None);
        assert_eq!(result.snfei.value.len(), 64);
        assert!(result.snfei.value.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_snfei_determinism() {
        let result1 = generate_snfei("Springfield USD", "US", None, None);
        let result2 = generate_snfei("Springfield USD", "US", None, None);
        assert_eq!(result1.snfei.value, result2.snfei.value);
    }

    #[test]
    fn test_snfei_result_fields() {
        let result = generate_snfei(
            "Springfield School District",
            "US",
            Some("123 Main St"),
            Some("1985-01-15"),
        );

        assert_eq!(result.tier, 3);
        assert!(result.confidence_score > 0.5);
        assert!(result.fields_used.contains(&"legal_name".to_string()));
        assert!(result.fields_used.contains(&"address".to_string()));
        assert!(
            result
                .fields_used
                .contains(&"registration_date".to_string())
        );
    }

    #[test]
    fn test_snfei_with_lei() {
        let result = generate_snfei_with_confidence(
            "Acme Corp",
            "US",
            None,
            None,
            Some("529900T8BM49AURSDO55"), // 20-char LEI
            None,
        );

        assert_eq!(result.tier, 1);
        assert_eq!(result.confidence_score, 1.0);
    }

    #[test]
    fn test_snfei_with_sam_uei() {
        let result = generate_snfei_with_confidence(
            "Acme Corp",
            "US",
            None,
            None,
            None,
            Some("J6H4FB3N5YK7"), // 12-char SAM UEI
        );

        assert_eq!(result.tier, 2);
        assert_eq!(result.confidence_score, 0.95);
    }

    #[test]
    fn test_snfei_simple() {
        let snfei = generate_snfei_simple("Springfield USD", "US", None);
        assert_eq!(snfei.len(), 64);
    }

    #[test]
    fn test_snfei_from_hash() {
        let valid_hash = "a".repeat(64);
        let snfei = Snfei::from_hash(&valid_hash);
        assert!(snfei.is_some());

        let invalid_hash = "too_short";
        let snfei = Snfei::from_hash(invalid_hash);
        assert!(snfei.is_none());
    }
}
