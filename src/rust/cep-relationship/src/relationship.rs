//! CEP Relationship Record definition.
//!
//! A Relationship Record represents a verifiable legal or functional relationship
//! between two or more attested entities.
//!
//! Relationships can be:
//! - **Bilateral**: Two-party relationships with clear directionality (contracts, grants)
//! - **Multilateral**: N-ary relationships (consortia, boards, joint ventures)

use crate::bilateral::BilateralParties;
use crate::multilateral::MultilateralMembers;
use cep_core::canonical::{format_amount, insert_if_present, insert_required, Canonicalize};
use cep_core::hash::CanonicalHash;
use cep_core::timestamp::CanonicalTimestamp;
use cep_core::{Attestation, SCHEMA_VERSION};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Relationship operational status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelationshipStatusCode {
    Pending,
    Active,
    Suspended,
    Completed,
    Terminated,
    Amended,
}

impl RelationshipStatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            RelationshipStatusCode::Pending => "PENDING",
            RelationshipStatusCode::Active => "ACTIVE",
            RelationshipStatusCode::Suspended => "SUSPENDED",
            RelationshipStatusCode::Completed => "COMPLETED",
            RelationshipStatusCode::Terminated => "TERMINATED",
            RelationshipStatusCode::Amended => "AMENDED",
        }
    }
}

/// Relationship status information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipStatus {
    pub status_code: RelationshipStatusCode,
    pub status_effective_timestamp: CanonicalTimestamp,
}

impl Canonicalize for RelationshipStatus {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        insert_required(&mut map, "statusCode", self.status_code.as_str());
        insert_required(
            &mut map,
            "statusEffectiveTimestamp",
            &self.status_effective_timestamp.to_canonical_string(),
        );
        map
    }
}

/// Financial terms of a relationship.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinancialTerms {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obligated_value: Option<f64>,
    #[serde(default = "default_currency")]
    pub currency_code: String,
}

fn default_currency() -> String {
    "USD".to_string()
}

impl Canonicalize for FinancialTerms {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        insert_required(&mut map, "currencyCode", &self.currency_code);
        if let Some(obligated) = self.obligated_value {
            insert_required(&mut map, "obligatedValue", &format_amount(obligated));
        }
        if let Some(total) = self.total_value {
            insert_required(&mut map, "totalValue", &format_amount(total));
        }
        map
    }
}

/// Reference to an authoritative source record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceReference {
    pub source_system_uri: String,
    pub source_record_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
}

impl Canonicalize for SourceReference {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        insert_required(&mut map, "sourceRecordId", &self.source_record_id);
        insert_required(&mut map, "sourceSystemUri", &self.source_system_uri);
        insert_if_present(&mut map, "sourceUrl", self.source_url.as_deref());
        map
    }
}

/// The type of parties in a relationship (bilateral or multilateral).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Parties {
    Bilateral(BilateralParties),
    Multilateral(MultilateralMembers),
}

impl Canonicalize for Parties {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        match self {
            Parties::Bilateral(b) => {
                let mut map = BTreeMap::new();
                map.insert("bilateralParties".to_string(), b.to_canonical_string());
                map
            }
            Parties::Multilateral(m) => {
                let mut map = BTreeMap::new();
                map.insert("multilateralMembers".to_string(), m.to_canonical_string());
                map
            }
        }
    }
}

/// A complete CEP Relationship Record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipRecord {
    /// Schema version.
    pub schema_version: String,

    /// Canonical identifier for this relationship.
    pub verifiable_id: String,

    /// URI referencing the relationship type vocabulary.
    pub relationship_type_uri: String,

    /// The parties involved in this relationship.
    pub parties: Parties,

    /// ID of the parent relationship (for subcontracts, task orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_relationship_id: Option<String>,

    /// When the relationship became legally effective.
    pub effective_timestamp: CanonicalTimestamp,

    /// When the relationship expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_timestamp: Option<CanonicalTimestamp>,

    /// Current status.
    pub status: RelationshipStatus,

    /// Financial terms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_terms: Option<FinancialTerms>,

    /// Domain-specific terms (canonically sorted by key).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_attributes: Option<BTreeMap<String, String>>,

    /// Primary jurisdiction.
    pub jurisdiction_iso: String,

    /// Source references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_references: Option<Vec<SourceReference>>,

    /// Cryptographic attestation.
    pub attestation: Attestation,

    /// Hash of the previous record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_record_hash: Option<CanonicalHash>,

    /// Revision number.
    pub revision_number: u32,
}

impl RelationshipRecord {
    /// Creates a new bilateral RelationshipRecord.
    pub fn new_bilateral(
        verifiable_id: String,
        relationship_type_uri: String,
        parties: BilateralParties,
        effective_timestamp: CanonicalTimestamp,
        status: RelationshipStatus,
        jurisdiction_iso: String,
        attestation: Attestation,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION.to_string(),
            verifiable_id,
            relationship_type_uri,
            parties: Parties::Bilateral(parties),
            parent_relationship_id: None,
            effective_timestamp,
            expiration_timestamp: None,
            status,
            financial_terms: None,
            terms_attributes: None,
            jurisdiction_iso,
            source_references: None,
            attestation,
            previous_record_hash: None,
            revision_number: 1,
        }
    }

    /// Creates a new multilateral RelationshipRecord.
    pub fn new_multilateral(
        verifiable_id: String,
        relationship_type_uri: String,
        members: MultilateralMembers,
        effective_timestamp: CanonicalTimestamp,
        status: RelationshipStatus,
        jurisdiction_iso: String,
        attestation: Attestation,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION.to_string(),
            verifiable_id,
            relationship_type_uri,
            parties: Parties::Multilateral(members),
            parent_relationship_id: None,
            effective_timestamp,
            expiration_timestamp: None,
            status,
            financial_terms: None,
            terms_attributes: None,
            jurisdiction_iso,
            source_references: None,
            attestation,
            previous_record_hash: None,
            revision_number: 1,
        }
    }

    pub fn with_parent(mut self, parent_id: String) -> Self {
        self.parent_relationship_id = Some(parent_id);
        self
    }

    pub fn with_expiration(mut self, timestamp: CanonicalTimestamp) -> Self {
        self.expiration_timestamp = Some(timestamp);
        self
    }

    pub fn with_financial_terms(mut self, terms: FinancialTerms) -> Self {
        self.financial_terms = Some(terms);
        self
    }

    pub fn with_terms_attribute(mut self, key: String, value: String) -> Self {
        self.terms_attributes
            .get_or_insert_with(BTreeMap::new)
            .insert(key, value);
        self
    }

    pub fn with_source_reference(mut self, reference: SourceReference) -> Self {
        self.source_references
            .get_or_insert_with(Vec::new)
            .push(reference);
        self
    }

    pub fn with_previous_hash(mut self, hash: CanonicalHash) -> Self {
        self.previous_record_hash = Some(hash);
        self
    }

    pub fn with_revision(mut self, revision: u32) -> Self {
        self.revision_number = revision;
        self
    }
}

impl Canonicalize for RelationshipRecord {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();

        // All fields in alphabetical order
        insert_required(&mut map, "attestation", &self.attestation.to_canonical_string());
        insert_required(
            &mut map,
            "effectiveTimestamp",
            &self.effective_timestamp.to_canonical_string(),
        );
        if let Some(ref exp) = self.expiration_timestamp {
            insert_required(&mut map, "expirationTimestamp", &exp.to_canonical_string());
        }
        if let Some(ref terms) = self.financial_terms {
            insert_required(&mut map, "financialTerms", &terms.to_canonical_string());
        }
        insert_required(&mut map, "jurisdictionIso", &self.jurisdiction_iso);
        insert_if_present(&mut map, "parentRelationshipId", self.parent_relationship_id.as_deref());

        // Parties (bilateral or multilateral)
        for (k, v) in self.parties.canonical_fields() {
            map.insert(k, v);
        }

        if let Some(ref hash) = self.previous_record_hash {
            insert_required(&mut map, "previousRecordHash", hash.as_hex());
        }
        insert_required(&mut map, "relationshipTypeUri", &self.relationship_type_uri);
        insert_required(&mut map, "revisionNumber", &self.revision_number.to_string());
        insert_required(&mut map, "schemaVersion", &self.schema_version);

        // Source references sorted by sourceSystemUri then sourceRecordId
        if let Some(ref refs) = self.source_references {
            if !refs.is_empty() {
                let mut sorted: Vec<_> = refs.iter().collect();
                sorted.sort_by(|a, b| {
                    (&a.source_system_uri, &a.source_record_id)
                        .cmp(&(&b.source_system_uri, &b.source_record_id))
                });
                let json: Vec<String> = sorted.iter().map(|r| r.to_canonical_string()).collect();
                map.insert("sourceReferences".to_string(), format!("[{}]", json.join(",")));
            }
        }

        insert_required(&mut map, "status", &self.status.to_canonical_string());

        // Terms attributes already in a BTreeMap, so sorted
        if let Some(ref attrs) = self.terms_attributes {
            if !attrs.is_empty() {
                let json = serde_json::to_string(attrs).unwrap_or_default();
                map.insert("termsAttributes".to_string(), json);
            }
        }

        insert_required(&mut map, "verifiableId", &self.verifiable_id);

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bilateral::Party;

    fn test_attestation() -> Attestation {
        Attestation::new(
            "cep-entity:sam-uei:ATTESTOR123A".to_string(),
            "2025-11-28T14:30:00.000000Z".parse().unwrap(),
            "Ed25519Signature2020".to_string(),
            "z3FXQqFwbZxKBxGxqFpCDabcdef1234567890".to_string(),
            "did:web:example.gov#key-1".to_string(),
        )
    }

    fn test_bilateral_relationship() -> RelationshipRecord {
        let parties = BilateralParties::new(
            Party::new(
                "cep-entity:sam-uei:AGENCY12345A".to_string(),
                "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/party-role.json#grantor".to_string(),
            ),
            Party::new(
                "cep-entity:sam-uei:VENDOR67890B".to_string(),
                "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/party-role.json#grantee".to_string(),
            ),
        );

        let status = RelationshipStatus {
            status_code: RelationshipStatusCode::Active,
            status_effective_timestamp: "2025-01-01T00:00:00.000000Z".parse().unwrap(),
        };

        RelationshipRecord::new_bilateral(
            "cep-relationship:usaspending:CONT_AWD_12345".to_string(),
            "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/relationship-type.json#prime-contract".to_string(),
            parties,
            "2025-01-01T00:00:00.000000Z".parse().unwrap(),
            status,
            "US".to_string(),
            test_attestation(),
        )
    }

    #[test]
    fn test_bilateral_creation() {
        let rel = test_bilateral_relationship();
        assert_eq!(rel.schema_version, "1.0.0");
        assert_eq!(rel.revision_number, 1);
    }

    #[test]
    fn test_canonical_determinism() {
        let r1 = test_bilateral_relationship();
        let r2 = test_bilateral_relationship();

        assert_eq!(r1.to_canonical_string(), r2.to_canonical_string());
        assert_eq!(r1.calculate_hash(), r2.calculate_hash());
    }

    #[test]
    fn test_with_financial_terms() {
        let rel = test_bilateral_relationship().with_financial_terms(FinancialTerms {
            total_value: Some(1000000.00),
            obligated_value: Some(500000.00),
            currency_code: "USD".to_string(),
        });

        let canonical = rel.to_canonical_string();
        assert!(canonical.contains("financialTerms"));
        assert!(canonical.contains("1000000.00"));
    }

    #[test]
    fn test_subcontract_parent_reference() {
        let prime = test_bilateral_relationship();
        let _prime_hash = prime.calculate_hash();

        let sub = test_bilateral_relationship()
            .with_parent("cep-relationship:usaspending:CONT_AWD_12345".to_string());

        assert_eq!(
            sub.parent_relationship_id,
            Some("cep-relationship:usaspending:CONT_AWD_12345".to_string())
        );
    }

    // ========================================
    // TEST VECTOR OUTPUT
    // ========================================

    #[test]
    fn test_vector_bilateral_relationship() {
        let rel = test_bilateral_relationship().with_financial_terms(FinancialTerms {
            total_value: Some(500000.00),
            obligated_value: Some(250000.00),
            currency_code: "USD".to_string(),
        });

        let canonical = rel.to_canonical_string();
        let hash = rel.calculate_hash();

        println!("\n========================================");
        println!("TEST VECTOR: Bilateral Relationship Record");
        println!("========================================");
        println!("\nCanonical String:\n{}", canonical);
        println!("\nSHA-256 Hash:\n{}", hash);
        println!("========================================\n");
    }
}