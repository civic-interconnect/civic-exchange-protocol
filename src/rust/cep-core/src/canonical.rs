//! Canonical serialization for CEP records.
//!
//! This module provides the core trait and utilities for generating
//! deterministic canonical strings from CEP records. The canonical string
//! is the input to SHA-256 hashing for record integrity verification.
//!
//! # Canonicalization Rules
//!
//! 1. **Field Order**: Fields MUST be serialized in a defined, alphabetical order.
//!    Use `BTreeMap` for key-value pairs and `BTreeSet` for collections.
//!
//! 2. **Null/Empty Omission**: Fields with null, None, or empty string values
//!    MUST be omitted entirely from the canonical string.
//!
//! 3. **Timestamp Format**: All timestamps MUST use `YYYY-MM-DDTHH:MM:SS.ffffffZ`
//!    with exactly 6 decimal places for microseconds.
//!
//! 4. **Numeric Format**: Monetary amounts MUST use exactly 2 decimal places.
//!    Integers MUST NOT have decimal points.
//!
//! 5. **String Escaping**: Strings are NOT JSON-escaped in the canonical form.
//!    The canonical string is a simple key:value concatenation.
//!
//! 6. **Encoding**: The canonical string MUST be UTF-8 encoded.

use crate::hash::CanonicalHash;
use std::collections::BTreeMap;

/// Trait for types that can be serialized to a canonical string for hashing.
pub trait Canonicalize {
    /// Returns the ordered map of field names to their canonical string values.
    ///
    /// Fields with None/null/empty values should NOT be included in the map.
    fn canonical_fields(&self) -> BTreeMap<String, String>;

    /// Generates the canonical string representation for hashing.
    ///
    /// Format: `"field1":"value1","field2":"value2",...`
    ///
    /// Fields are ordered alphabetically by key.
    fn to_canonical_string(&self) -> String {
        let fields = self.canonical_fields();
        fields
            .into_iter()
            .map(|(k, v)| format!("\"{}\":\"{}\"", k, v))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Computes the SHA-256 hash of the canonical string.
    fn calculate_hash(&self) -> CanonicalHash {
        CanonicalHash::from_canonical_string(&self.to_canonical_string())
    }
}

/// Helper function to format a monetary amount with exactly 2 decimal places.
///
/// This ensures consistent formatting across all implementations:
/// - 100 becomes "100.00"
/// - 100.5 becomes "100.50"
/// - 100.756 becomes "100.76" (rounded)
pub fn format_amount(amount: f64) -> String {
    format!("{:.2}", amount)
}

/// Helper function to add a field to a BTreeMap only if the value is Some and non-empty.
pub fn insert_if_present(map: &mut BTreeMap<String, String>, key: &str, value: Option<&str>) {
    if let Some(v) = value {
        if !v.is_empty() {
            map.insert(key.to_string(), v.to_string());
        }
    }
}

/// Helper function to add a required field to a BTreeMap.
pub fn insert_required(map: &mut BTreeMap<String, String>, key: &str, value: &str) {
    map.insert(key.to_string(), value.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestRecord {
        alpha: String,
        beta: Option<String>,
        gamma: String,
    }

    impl Canonicalize for TestRecord {
        fn canonical_fields(&self) -> BTreeMap<String, String> {
            let mut map = BTreeMap::new();
            insert_required(&mut map, "alpha", &self.alpha);
            insert_if_present(&mut map, "beta", self.beta.as_deref());
            insert_required(&mut map, "gamma", &self.gamma);
            map
        }
    }

    #[test]
    fn test_field_ordering() {
        let record = TestRecord {
            alpha: "first".to_string(),
            beta: Some("second".to_string()),
            gamma: "third".to_string(),
        };

        let canonical = record.to_canonical_string();
        // Fields should be alphabetically ordered
        assert_eq!(
            canonical,
            r#""alpha":"first","beta":"second","gamma":"third""#
        );
    }

    #[test]
    fn test_null_omission() {
        let record = TestRecord {
            alpha: "first".to_string(),
            beta: None, // Should be omitted
            gamma: "third".to_string(),
        };

        let canonical = record.to_canonical_string();
        assert_eq!(canonical, r#""alpha":"first","gamma":"third""#);
    }

    #[test]
    fn test_empty_string_omission() {
        let record = TestRecord {
            alpha: "first".to_string(),
            beta: Some("".to_string()), // Should be omitted
            gamma: "third".to_string(),
        };

        let canonical = record.to_canonical_string();
        assert_eq!(canonical, r#""alpha":"first","gamma":"third""#);
    }

    #[test]
    fn test_format_amount() {
        assert_eq!(format_amount(100.0), "100.00");
        assert_eq!(format_amount(100.5), "100.50");
        assert_eq!(format_amount(100.756), "100.76");
        assert_eq!(format_amount(0.0), "0.00");
        assert_eq!(format_amount(1234567.89), "1234567.89");
    }

    #[test]
    fn test_hash_determinism() {
        let record1 = TestRecord {
            alpha: "a".to_string(),
            beta: Some("b".to_string()),
            gamma: "c".to_string(),
        };

        let record2 = TestRecord {
            alpha: "a".to_string(),
            beta: Some("b".to_string()),
            gamma: "c".to_string(),
        };

        assert_eq!(record1.calculate_hash(), record2.calculate_hash());
    }
}