//! CEP Entity Record definition.
//!
//! The Entity Record is the foundational primitive in CEP. It represents a
//! verified civic entity (government agency, contractor, nonprofit, individual).
//! All relationships and exchanges reference attested entities.

use crate::identifiers::EntityIdentifiers;
use cep_core::canonical::{insert_if_present, insert_required, Canonicalize};
use cep_core::hash::CanonicalHash;
use cep_core::{Attestation, SCHEMA_VERSION};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Entity operational status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityStatusCode {
    Active,
    Inactive,
    Suspended,
    Dissolved,
    Merged,
}

impl EntityStatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityStatusCode::Active => "ACTIVE",
            EntityStatusCode::Inactive => "INACTIVE",
            EntityStatusCode::Suspended => "SUSPENDED",
            EntityStatusCode::Dissolved => "DISSOLVED",
            EntityStatusCode::Merged => "MERGED",
        }
    }
}

/// Entity status information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityStatus {
    pub status_code: EntityStatusCode,
    pub status_effective_date: String, // YYYY-MM-DD format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_termination_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successor_entity_id: Option<String>,
}

impl Canonicalize for EntityStatus {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        insert_required(&mut map, "statusCode", self.status_code.as_str());
        insert_required(&mut map, "statusEffectiveDate", &self.status_effective_date);
        insert_if_present(&mut map, "statusTerminationDate", self.status_termination_date.as_deref());
        insert_if_present(&mut map, "successorEntityId", self.successor_entity_id.as_deref());
        map
    }
}

/// Entity resolution confidence metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolutionConfidence {
    /// Confidence score from 0.0 to 1.0.
    pub score: f64,
    /// URI identifying the resolution method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_uri: Option<String>,
    /// Number of source records resolved to this entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_record_count: Option<u32>,
}

impl Canonicalize for ResolutionConfidence {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        insert_if_present(&mut map, "methodUri", self.method_uri.as_deref());
        // Score formatted to 2 decimal places for consistency
        insert_required(&mut map, "score", &format!("{:.2}", self.score));
        if let Some(count) = self.source_record_count {
            insert_required(&mut map, "sourceRecordCount", &count.to_string());
        }
        map
    }
}

/// A complete CEP Entity Record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityRecord {
    /// Schema version (must be "1.0.0").
    pub schema_version: String,

    /// Canonical identifier for this entity record.
    pub verifiable_id: String,

    /// All known identifiers for this entity.
    pub identifiers: EntityIdentifiers,

    /// Official legal name as registered.
    pub legal_name: String,

    /// Normalized name for matching and SNFEI generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name_normalized: Option<String>,

    /// URI referencing the entity type vocabulary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type_uri: Option<String>,

    /// ISO 3166-1/2 jurisdiction code.
    pub jurisdiction_iso: String,

    /// Entity operational status.
    pub status: EntityStatus,

    /// NAICS industry classification code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naics_code: Option<String>,

    /// Entity resolution confidence metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution_confidence: Option<ResolutionConfidence>,

    /// Cryptographic attestation.
    pub attestation: Attestation,

    /// Hash of the previous record in the revision chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_record_hash: Option<CanonicalHash>,

    /// Monotonically increasing revision number.
    pub revision_number: u32,
}

impl EntityRecord {
    /// Creates a new EntityRecord with required fields.
    pub fn new(
        verifiable_id: String,
        identifiers: EntityIdentifiers,
        legal_name: String,
        jurisdiction_iso: String,
        status: EntityStatus,
        attestation: Attestation,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION.to_string(),
            verifiable_id,
            identifiers,
            legal_name,
            legal_name_normalized: None,
            entity_type_uri: None,
            jurisdiction_iso,
            status,
            naics_code: None,
            resolution_confidence: None,
            attestation,
            previous_record_hash: None,
            revision_number: 1,
        }
    }

    /// Sets the normalized legal name.
    pub fn with_normalized_name(mut self, name: String) -> Self {
        self.legal_name_normalized = Some(name);
        self
    }

    /// Sets the entity type URI.
    pub fn with_entity_type(mut self, uri: String) -> Self {
        self.entity_type_uri = Some(uri);
        self
    }

    /// Sets the NAICS code.
    pub fn with_naics(mut self, code: String) -> Self {
        self.naics_code = Some(code);
        self
    }

    /// Sets resolution confidence.
    pub fn with_resolution_confidence(mut self, confidence: ResolutionConfidence) -> Self {
        self.resolution_confidence = Some(confidence);
        self
    }

    /// Sets the previous record hash (for revisions).
    pub fn with_previous_hash(mut self, hash: CanonicalHash) -> Self {
        self.previous_record_hash = Some(hash);
        self
    }

    /// Sets the revision number.
    pub fn with_revision(mut self, revision: u32) -> Self {
        self.revision_number = revision;
        self
    }

    /// Validates that the record has all required fields properly set.
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != SCHEMA_VERSION {
            return Err(format!(
                "Unsupported schema version: {}",
                self.schema_version
            ));
        }
        if self.verifiable_id.is_empty() {
            return Err("verifiableId is required".to_string());
        }
        if !self.identifiers.has_any() {
            return Err("At least one identifier is required".to_string());
        }
        if self.legal_name.is_empty() {
            return Err("legalName is required".to_string());
        }
        if self.jurisdiction_iso.is_empty() {
            return Err("jurisdictionIso is required".to_string());
        }
        if self.revision_number < 1 {
            return Err("revisionNumber must be >= 1".to_string());
        }
        Ok(())
    }
}

impl Canonicalize for EntityRecord {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();

        // All fields in alphabetical order
        
        // Attestation is a nested object - serialize its canonical form
        let attestation_canonical = self.attestation.to_canonical_string();
        insert_required(&mut map, "attestation", &attestation_canonical);

        insert_if_present(&mut map, "entityTypeUri", self.entity_type_uri.as_deref());

        // Identifiers is a nested object
        let identifiers_canonical = self.identifiers.to_canonical_string();
        if !identifiers_canonical.is_empty() {
            insert_required(&mut map, "identifiers", &identifiers_canonical);
        }

        insert_required(&mut map, "jurisdictionIso", &self.jurisdiction_iso);
        insert_required(&mut map, "legalName", &self.legal_name);
        insert_if_present(&mut map, "legalNameNormalized", self.legal_name_normalized.as_deref());
        insert_if_present(&mut map, "naicsCode", self.naics_code.as_deref());

        if let Some(ref hash) = self.previous_record_hash {
            insert_required(&mut map, "previousRecordHash", hash.as_hex());
        }

        // Resolution confidence is a nested object
        if let Some(ref confidence) = self.resolution_confidence {
            let confidence_canonical = confidence.to_canonical_string();
            insert_required(&mut map, "resolutionConfidence", &confidence_canonical);
        }

        insert_required(&mut map, "revisionNumber", &self.revision_number.to_string());
        insert_required(&mut map, "schemaVersion", &self.schema_version);

        // Status is a nested object
        let status_canonical = self.status.to_canonical_string();
        insert_required(&mut map, "status", &status_canonical);

        insert_required(&mut map, "verifiableId", &self.verifiable_id);

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifiers::SamUei;

    fn test_attestation() -> Attestation {
        Attestation::new(
            "cep-entity:sam-uei:ATTESTOR123A".to_string(),
            "2025-11-28T14:30:00.000000Z".parse().unwrap(),
            "Ed25519Signature2020".to_string(),
            "z3FXQqFwbZxKBxGxqFpCDabcdef1234567890".to_string(),
            "did:web:example.gov#key-1".to_string(),
        )
    }

    fn test_entity() -> EntityRecord {
        let identifiers = EntityIdentifiers::new()
            .with_sam_uei(SamUei::new("J6H4FB3N5YK7").unwrap());

        let status = EntityStatus {
            status_code: EntityStatusCode::Active,
            status_effective_date: "2020-01-15".to_string(),
            status_termination_date: None,
            successor_entity_id: None,
        };

        EntityRecord::new(
            "cep-entity:sam-uei:J6H4FB3N5YK7".to_string(),
            identifiers,
            "Acme Consulting LLC".to_string(),
            "US-CA".to_string(),
            status,
            test_attestation(),
        )
    }

    #[test]
    fn test_entity_creation() {
        let entity = test_entity();
        assert_eq!(entity.schema_version, "1.0.0");
        assert_eq!(entity.revision_number, 1);
        assert!(entity.validate().is_ok());
    }

    #[test]
    fn test_canonical_string_determinism() {
        let e1 = test_entity();
        let e2 = test_entity();
        
        assert_eq!(e1.to_canonical_string(), e2.to_canonical_string());
        assert_eq!(e1.calculate_hash(), e2.calculate_hash());
    }

    #[test]
    fn test_canonical_field_order() {
        let entity = test_entity();
        let fields = entity.canonical_fields();
        let keys: Vec<&String> = fields.keys().collect();

        // Verify alphabetical ordering
        let mut sorted_keys = keys.clone();
        sorted_keys.sort();
        assert_eq!(keys, sorted_keys);
    }

    #[test]
    fn test_validation_missing_identifier() {
        let entity = EntityRecord::new(
            "test-id".to_string(),
            EntityIdentifiers::new(), // Empty - should fail
            "Test Entity".to_string(),
            "US".to_string(),
            EntityStatus {
                status_code: EntityStatusCode::Active,
                status_effective_date: "2020-01-01".to_string(),
                status_termination_date: None,
                successor_entity_id: None,
            },
            test_attestation(),
        );

        assert!(entity.validate().is_err());
    }

    #[test]
    fn test_hash_changes_with_content() {
        let e1 = test_entity();
        let e2 = test_entity().with_naics("541512".to_string());

        assert_ne!(e1.calculate_hash(), e2.calculate_hash());
    }

    #[test]
    fn test_revision_chain() {
        let e1 = test_entity();
        let hash1 = e1.calculate_hash();

        let e2 = test_entity()
            .with_previous_hash(hash1.clone())
            .with_revision(2)
            .with_naics("541512".to_string());

        assert_eq!(e2.previous_record_hash, Some(hash1));
        assert_eq!(e2.revision_number, 2);
    }

    // ========================================
    // TEST VECTOR OUTPUT
    // ========================================
    // This test outputs the canonical string and hash that other
    // implementations (Python, Java, C#, etc.) MUST match.

    #[test]
    fn test_vector_basic_entity() {
        let entity = test_entity();
        
        let canonical = entity.to_canonical_string();
        let hash = entity.calculate_hash();

        println!("\n========================================");
        println!("TEST VECTOR: Basic Entity Record");
        println!("========================================");
        println!("\nCanonical String:\n{}", canonical);
        println!("\nSHA-256 Hash:\n{}", hash);
        println!("========================================\n");

        // These values become the test vector for other implementations
        // After running once, uncomment and update these assertions:
        // assert_eq!(hash.as_hex(), "expected_hash_here");
    }
}