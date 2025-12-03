/// Entity identifier types for CEP.
///
/// CEP supports multiple identifier schemes organized into tiers:
///
/// - **Tier 1 (Global)**: LEI (Legal Entity Identifier)
/// - **Tier 2 (Federal)**: SAM.gov UEI
/// - **Tier 3 (Sub-National)**: SNFEI (generated hash-based identifier)
/// - **Extended**: Canadian BN, UK Companies House, etc.
///
/// # SNFEI Generation
///
/// For full SNFEI generation with normalization and localization, use the
/// `cep_snfei` crate directly:
///
/// ```rust
/// use cep_snfei::{generate_snfei, apply_localization};
///
/// let result = generate_snfei(
///     "Springfield USD #12",
///     "US",
///     Some("123 Main St"),
///     None,
/// );
/// let snfei = result.snfei;
/// ```
use cep_core::canonical::{Canonicalize, insert_if_present};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// Re-export Snfei from cep-snfei for convenience
// Users can access full generation via cep_snfei::{generate_snfei, normalize_legal_name, ...}
pub use cep_snfei::Snfei;

/// SAM.gov Unique Entity Identifier (12 alphanumeric characters).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SamUei(String);

impl SamUei {
    /// Creates a new SAM UEI, validating the format.
    pub fn new(value: &str) -> Option<Self> {
        if value.len() == 12
            && value
                .chars()
                .all(|c| c.is_ascii_alphanumeric() && c.is_ascii_uppercase() || c.is_ascii_digit())
        {
            Some(Self(value.to_string()))
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Legal Entity Identifier per ISO 17442 (20 alphanumeric characters).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Lei(String);

impl Lei {
    /// Creates a new LEI, validating the format.
    pub fn new(value: &str) -> Option<Self> {
        if value.len() == 20 && value.chars().all(|c| c.is_ascii_alphanumeric()) {
            Some(Self(value.to_uppercase()))
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Canadian Business Number with program account.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanadianBn(String);

impl CanadianBn {
    /// Creates a new Canadian BN (format: 123456789RC0001).
    pub fn new(value: &str) -> Option<Self> {
        // Pattern: 9 digits + 2 letters + 4 digits
        if value.len() == 15 {
            let (digits1, rest) = value.split_at(9);
            let (letters, digits2) = rest.split_at(2);
            if digits1.chars().all(|c| c.is_ascii_digit())
                && letters.chars().all(|c| c.is_ascii_uppercase())
                && digits2.chars().all(|c| c.is_ascii_digit())
            {
                return Some(Self(value.to_string()));
            }
        }
        None
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An additional identifier scheme not explicitly defined in the schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalScheme {
    /// URI identifying the identifier scheme.
    pub scheme_uri: String,
    /// The identifier value.
    pub value: String,
}

/// Collection of all known identifiers for an entity.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityIdentifiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sam_uei: Option<SamUei>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<Lei>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub snfei: Option<Snfei>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub canadian_bn: Option<CanadianBn>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_schemes: Option<Vec<AdditionalScheme>>,
}

impl EntityIdentifiers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sam_uei(mut self, uei: SamUei) -> Self {
        self.sam_uei = Some(uei);
        self
    }

    pub fn with_lei(mut self, lei: Lei) -> Self {
        self.lei = Some(lei);
        self
    }

    pub fn with_snfei(mut self, snfei: Snfei) -> Self {
        self.snfei = Some(snfei);
        self
    }

    /// Returns true if at least one identifier is present.
    pub fn has_any(&self) -> bool {
        self.sam_uei.is_some()
            || self.lei.is_some()
            || self.snfei.is_some()
            || self.canadian_bn.is_some()
            || self
                .additional_schemes
                .as_ref()
                .map_or(false, |v| !v.is_empty())
    }

    /// Returns the "best" identifier for use as the verifiable ID.
    /// Priority: LEI > SAM UEI > SNFEI > Canadian BN > first additional
    pub fn primary_identifier(&self) -> Option<String> {
        if let Some(ref lei) = self.lei {
            return Some(format!("cep-entity:lei:{}", lei.as_str()));
        }
        if let Some(ref uei) = self.sam_uei {
            return Some(format!("cep-entity:sam-uei:{}", uei.as_str()));
        }
        if let Some(ref snfei) = self.snfei {
            return Some(format!("cep-entity:snfei:{}", snfei.value()));
        }
        if let Some(ref bn) = self.canadian_bn {
            return Some(format!("cep-entity:canadian-bn:{}", bn.as_str()));
        }
        if let Some(ref schemes) = self.additional_schemes {
            if let Some(first) = schemes.first() {
                return Some(format!("cep-entity:other:{}", first.value));
            }
        }
        None
    }
}

impl Canonicalize for EntityIdentifiers {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();

        // Additional schemes serialized as JSON array string for canonical form
        if let Some(ref schemes) = self.additional_schemes {
            if !schemes.is_empty() {
                // Sort by scheme_uri for determinism
                let mut sorted: Vec<_> = schemes.iter().collect();
                sorted.sort_by(|a, b| a.scheme_uri.cmp(&b.scheme_uri));
                let json = serde_json::to_string(&sorted).unwrap_or_default();
                map.insert("additionalSchemes".to_string(), json);
            }
        }

        insert_if_present(
            &mut map,
            "canadianBn",
            self.canadian_bn.as_ref().map(|x| x.as_str()),
        );
        insert_if_present(&mut map, "lei", self.lei.as_ref().map(|x| x.as_str()));
        insert_if_present(
            &mut map,
            "samUei",
            self.sam_uei.as_ref().map(|x| x.as_str()),
        );
        insert_if_present(&mut map, "snfei", self.snfei.as_ref().map(|x| x.value()));

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cep_snfei::generate_snfei;

    #[test]
    fn test_sam_uei_valid() {
        assert!(SamUei::new("J6H4FB3N5YK7").is_some());
        assert!(SamUei::new("ABC123DEF456").is_some());
    }

    #[test]
    fn test_sam_uei_invalid() {
        assert!(SamUei::new("abc123def456").is_none()); // lowercase
        assert!(SamUei::new("J6H4FB3N5YK").is_none()); // too short
        assert!(SamUei::new("J6H4FB3N5YK78").is_none()); // too long
    }

    #[test]
    fn test_lei_valid() {
        assert!(Lei::new("5493001KJTIIGC8Y1R12").is_some());
    }

    #[test]
    fn test_snfei_from_cep_snfei() {
        let result = generate_snfei("Acme Consulting LLC", "US", None, None);
        let snfei = result.snfei;
        assert_eq!(snfei.value().len(), 64);

        // Same input should produce same output
        let result2 = generate_snfei("Acme Consulting LLC", "US", None, None);
        let snfei2 = result2.snfei;
        assert_eq!(snfei, snfei2);

        // Different input should produce different output
        let result3 = generate_snfei("Acme Consulting LLC", "CA", None, None);
        let snfei3 = result3.snfei;
        assert_ne!(snfei, snfei3);
    }

    #[test]
    fn test_snfei_normalization_equivalence() {
        // Different surface forms should normalize to same SNFEI
        let result1 = generate_snfei("Springfield USD", "US", None, None);
        let snfei1 = result1.snfei;

        let result2 = generate_snfei("SPRINGFIELD USD", "US", None, None);
        let snfei2 = result2.snfei;

        let result3 = generate_snfei("springfield usd", "US", None, None);
        let snfei3 = result3.snfei;

        assert_eq!(snfei1, snfei2);
        assert_eq!(snfei2, snfei3);
    }

    #[test]
    fn test_canadian_bn_valid() {
        assert!(CanadianBn::new("123456789RC0001").is_some());
    }

    #[test]
    fn test_primary_identifier_priority() {
        let result = generate_snfei("test", "US", None, None);
        let snfei = result.snfei;

        let ids = EntityIdentifiers::new()
            .with_sam_uei(SamUei::new("J6H4FB3N5YK7").unwrap())
            .with_snfei(snfei);

        // SAM UEI should take priority over SNFEI
        let primary = ids.primary_identifier().unwrap();
        assert!(primary.starts_with("cep-entity:sam-uei:"));
    }

    #[test]
    fn test_canonical_field_order() {
        let ids = EntityIdentifiers::new()
            .with_sam_uei(SamUei::new("J6H4FB3N5YK7").unwrap())
            .with_lei(Lei::new("5493001KJTIIGC8Y1R12").unwrap());

        let fields = ids.canonical_fields();
        let keys: Vec<&String> = fields.keys().collect();

        // Should be alphabetical
        assert_eq!(keys, vec!["lei", "samUei"]);
    }
}
