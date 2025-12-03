/// CEP Exchange Record definition.
///
/// An Exchange Record represents a verifiable value exchange (financial, in-kind,
/// or informational) between entities within an established relationship.
/// This is the atomic unit of civic transparency.

use crate::provenance::{ExchangeCategorization, ProvenanceChain};
use crate::value::{ExchangeParty, ExchangeValue};
use cep_core::canonical::{insert_if_present, insert_required, Canonicalize};
use cep_core::hash::CanonicalHash;
use cep_core::timestamp::CanonicalTimestamp;
use cep_core::{Attestation, SCHEMA_VERSION};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Exchange operational status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExchangeStatusCode {
    Pending,
    Completed,
    Reversed,
    Canceled,
    Disputed,
}

impl ExchangeStatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExchangeStatusCode::Pending => "PENDING",
            ExchangeStatusCode::Completed => "COMPLETED",
            ExchangeStatusCode::Reversed => "REVERSED",
            ExchangeStatusCode::Canceled => "CANCELED",
            ExchangeStatusCode::Disputed => "DISPUTED",
        }
    }
}

/// Exchange status information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeStatus {
    pub status_code: ExchangeStatusCode,
    pub status_effective_timestamp: CanonicalTimestamp,
}

impl Canonicalize for ExchangeStatus {
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

/// A complete CEP Exchange Record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeRecord {
    /// Schema version.
    pub schema_version: String,

    /// Canonical identifier for this exchange.
    pub verifiable_id: String,

    /// Verifiable ID of the relationship under which this exchange occurs.
    pub relationship_id: String,

    /// URI referencing the exchange type vocabulary.
    pub exchange_type_uri: String,

    /// The entity from which value flows.
    pub source_entity: ExchangeParty,

    /// The entity to which value flows.
    pub recipient_entity: ExchangeParty,

    /// The value being exchanged.
    pub value: ExchangeValue,

    /// When the exchange actually occurred.
    pub occurred_timestamp: CanonicalTimestamp,

    /// Current status.
    pub status: ExchangeStatus,

    /// Provenance chain information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_chain: Option<ProvenanceChain>,

    /// Categorization codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorization: Option<ExchangeCategorization>,

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

impl ExchangeRecord {
    /// Creates a new ExchangeRecord with required fields.
    pub fn new(
        verifiable_id: String,
        relationship_id: String,
        exchange_type_uri: String,
        source_entity: ExchangeParty,
        recipient_entity: ExchangeParty,
        value: ExchangeValue,
        occurred_timestamp: CanonicalTimestamp,
        status: ExchangeStatus,
        attestation: Attestation,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION.to_string(),
            verifiable_id,
            relationship_id,
            exchange_type_uri,
            source_entity,
            recipient_entity,
            value,
            occurred_timestamp,
            status,
            provenance_chain: None,
            categorization: None,
            source_references: None,
            attestation,
            previous_record_hash: None,
            revision_number: 1,
        }
    }

    pub fn with_provenance(mut self, chain: ProvenanceChain) -> Self {
        self.provenance_chain = Some(chain);
        self
    }

    pub fn with_categorization(mut self, cat: ExchangeCategorization) -> Self {
        self.categorization = Some(cat);
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

impl Canonicalize for ExchangeRecord {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();

        // All fields in alphabetical order
        insert_required(&mut map, "attestation", &self.attestation.to_canonical_string());

        if let Some(ref cat) = self.categorization {
            if cat.has_any() {
                insert_required(&mut map, "categorization", &cat.to_canonical_string());
            }
        }

        insert_required(&mut map, "exchangeTypeUri", &self.exchange_type_uri);
        insert_required(&mut map, "occurredTimestamp", &self.occurred_timestamp.to_canonical_string());

        if let Some(ref hash) = self.previous_record_hash {
            insert_required(&mut map, "previousRecordHash", hash.as_hex());
        }

        if let Some(ref chain) = self.provenance_chain {
            if chain.has_any() {
                insert_required(&mut map, "provenanceChain", &chain.to_canonical_string());
            }
        }

        insert_required(&mut map, "recipientEntity", &self.recipient_entity.to_canonical_string());
        insert_required(&mut map, "relationshipId", &self.relationship_id);
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

        insert_required(&mut map, "sourceEntity", &self.source_entity.to_canonical_string());
        insert_required(&mut map, "status", &self.status.to_canonical_string());
        insert_required(&mut map, "value", &self.value.to_canonical_string());
        insert_required(&mut map, "verifiableId", &self.verifiable_id);

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::IntermediaryEntity;

    fn test_attestation() -> Attestation {
        Attestation::new(
            "cep-entity:sam-uei:ATTESTOR123A".to_string(),
            "2025-11-28T14:30:00.000000Z".parse().unwrap(),
            "Ed25519Signature2020".to_string(),
            "z3FXQqFwbZxKBxGxqFpCDabcdef1234567890".to_string(),
            "did:web:example.gov#key-1".to_string(),
        )
    }

    fn test_exchange() -> ExchangeRecord {
        let source = ExchangeParty::new("cep-entity:sam-uei:AGENCY12345A".to_string())
            .with_role("https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/exchange-role.json#disbursing-agency".to_string());

        let recipient = ExchangeParty::new("cep-entity:sam-uei:SCHOOL67890B".to_string())
            .with_role("https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/exchange-role.json#grantee".to_string());

        let value = ExchangeValue::usd(50000.00);

        let status = ExchangeStatus {
            status_code: ExchangeStatusCode::Completed,
            status_effective_timestamp: "2025-09-15T14:03:22.500000Z".parse().unwrap(),
        };

        ExchangeRecord::new(
            "cep-exchange:treasury:PAY_2025_001234".to_string(),
            "cep-relationship:usaspending:GRANT_84010_2025".to_string(),
            "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/exchange-type.json#grant-disbursement".to_string(),
            source,
            recipient,
            value,
            "2025-09-15T14:03:22.500000Z".parse().unwrap(),
            status,
            test_attestation(),
        )
    }

    #[test]
    fn test_exchange_creation() {
        let exchange = test_exchange();
        assert_eq!(exchange.schema_version, "1.0.0");
        assert_eq!(exchange.revision_number, 1);
    }

    #[test]
    fn test_canonical_determinism() {
        let e1 = test_exchange();
        let e2 = test_exchange();

        assert_eq!(e1.to_canonical_string(), e2.to_canonical_string());
        assert_eq!(e1.calculate_hash(), e2.calculate_hash());
    }

    #[test]
    fn test_with_provenance() {
        let exchange = test_exchange().with_provenance(
            ProvenanceChain::new()
                .with_funding_chain_tag("FEDERAL>STATE>SCHOOL_DISTRICT".to_string())
                .with_ultimate_source("cep-entity:sam-uei:USDOE12345AB".to_string())
                .with_intermediary(IntermediaryEntity::new("cep-entity:sam-uei:STATEED123A".to_string())),
        );

        let canonical = exchange.to_canonical_string();
        assert!(canonical.contains("provenanceChain"));
        assert!(canonical.contains("FEDERAL>STATE>SCHOOL_DISTRICT"));
    }

    #[test]
    fn test_with_categorization() {
        let exchange = test_exchange().with_categorization(
            ExchangeCategorization::new()
                .with_cfda("84.010".to_string())
                .with_local_category("TITLE1".to_string(), "Title I Funds".to_string()),
        );

        let canonical = exchange.to_canonical_string();
        assert!(canonical.contains("categorization"));
        assert!(canonical.contains("84.010"));
    }

    // ========================================
    // TEST VECTOR OUTPUT
    // ========================================

    #[test]
    fn test_vector_basic_exchange() {
        let exchange = test_exchange()
            .with_provenance(
                ProvenanceChain::new()
                    .with_funding_chain_tag("FEDERAL>STATE>SCHOOL_DISTRICT".to_string())
                    .with_ultimate_source("cep-entity:sam-uei:USDOE12345AB".to_string()),
            )
            .with_categorization(
                ExchangeCategorization::new()
                    .with_cfda("84.010".to_string()),
            );

        let canonical = exchange.to_canonical_string();
        let hash = exchange.calculate_hash();

        println!("\n========================================");
        println!("TEST VECTOR: Basic Exchange Record");
        println!("========================================");
        println!("\nCanonical String:\n{}", canonical);
        println!("\nSHA-256 Hash:\n{}", hash);
        println!("========================================\n");
    }
}