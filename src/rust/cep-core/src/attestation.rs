//! Attestation and cryptographic proof types for CEP records.
//!
//! Every CEP record includes an attestation block that proves:
//! - Who attested to the record (attestorId)
//! - When it was attested (attestationTimestamp)
//! - Cryptographic proof of integrity (proofType, proofValue, verificationMethodUri)

use crate::canonical::{insert_if_present, insert_required, Canonicalize};
use crate::timestamp::CanonicalTimestamp;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The purpose of a cryptographic proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ProofPurpose {
    /// The proof asserts the truth of a claim.
    AssertionMethod,
    /// The proof authenticates the identity of the attestor.
    Authentication,
    /// The proof delegates a capability to another party.
    CapabilityDelegation,
}

impl ProofPurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProofPurpose::AssertionMethod => "assertionMethod",
            ProofPurpose::Authentication => "authentication",
            ProofPurpose::CapabilityDelegation => "capabilityDelegation",
        }
    }
}

impl Default for ProofPurpose {
    fn default() -> Self {
        ProofPurpose::AssertionMethod
    }
}

/// Cryptographic attestation proving record authenticity and integrity.
///
/// This structure aligns with W3C Verifiable Credentials Data Integrity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attestation {
    /// Verifiable ID of the entity or node attesting to this record.
    pub attestor_id: String,

    /// When the attestation was created.
    pub attestation_timestamp: CanonicalTimestamp,

    /// The proof algorithm identifier.
    /// Examples: "Ed25519Signature2020", "EcdsaSecp256k1Signature2019", "DataIntegrityProof"
    pub proof_type: String,

    /// The cryptographic signature or proof value.
    pub proof_value: String,

    /// URI resolving to the public key or DID document for verification.
    pub verification_method_uri: String,

    /// The purpose of the proof.
    #[serde(default)]
    pub proof_purpose: ProofPurpose,

    /// Optional URI to a timestamping authority or DLT anchor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_uri: Option<String>,
}

impl Attestation {
    /// Creates a new Attestation with required fields.
    pub fn new(
        attestor_id: String,
        attestation_timestamp: CanonicalTimestamp,
        proof_type: String,
        proof_value: String,
        verification_method_uri: String,
    ) -> Self {
        Self {
            attestor_id,
            attestation_timestamp,
            proof_type,
            proof_value,
            verification_method_uri,
            proof_purpose: ProofPurpose::default(),
            anchor_uri: None,
        }
    }

    /// Sets the proof purpose.
    pub fn with_purpose(mut self, purpose: ProofPurpose) -> Self {
        self.proof_purpose = purpose;
        self
    }

    /// Sets the anchor URI.
    pub fn with_anchor(mut self, uri: String) -> Self {
        self.anchor_uri = Some(uri);
        self
    }
}

impl Canonicalize for Attestation {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();

        // Fields in alphabetical order
        insert_if_present(&mut map, "anchorUri", self.anchor_uri.as_deref());
        insert_required(
            &mut map,
            "attestationTimestamp",
            &self.attestation_timestamp.to_canonical_string(),
        );
        insert_required(&mut map, "attestorId", &self.attestor_id);
        insert_required(&mut map, "proofPurpose", self.proof_purpose.as_str());
        insert_required(&mut map, "proofType", &self.proof_type);
        insert_required(&mut map, "proofValue", &self.proof_value);
        insert_required(&mut map, "verificationMethodUri", &self.verification_method_uri);

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_attestation() -> Attestation {
        Attestation::new(
            "cep-entity:sam-uei:J6H4FB3N5YK7".to_string(),
            "2025-11-28T14:30:00.000000Z".parse().unwrap(),
            "Ed25519Signature2020".to_string(),
            "z3FXQqFwbZxKBxGxqFpCD...".to_string(),
            "did:web:example.gov#key-1".to_string(),
        )
    }

    #[test]
    fn test_canonical_field_order() {
        let attestation = test_attestation();
        let fields = attestation.canonical_fields();

        let keys: Vec<&String> = fields.keys().collect();
        // Verify alphabetical order
        assert_eq!(
            keys,
            vec![
                "attestationTimestamp",
                "attestorId",
                "proofPurpose",
                "proofType",
                "proofValue",
                "verificationMethodUri"
            ]
        );
    }

    #[test]
    fn test_canonical_string() {
        let attestation = test_attestation();
        let canonical = attestation.to_canonical_string();

        assert!(canonical.starts_with(r#""attestationTimestamp":"2025-11-28T14:30:00.000000Z""#));
        assert!(canonical.contains(r#""attestorId":"cep-entity:sam-uei:J6H4FB3N5YK7""#));
        assert!(canonical.contains(r#""proofPurpose":"assertionMethod""#));
    }

    #[test]
    fn test_with_anchor() {
        let attestation = test_attestation()
            .with_anchor("https://blockchain.example.com/tx/abc123".to_string());

        let fields = attestation.canonical_fields();
        assert!(fields.contains_key("anchorUri"));
    }

    #[test]
    fn test_hash_stability() {
        let a1 = test_attestation();
        let a2 = test_attestation();

        assert_eq!(a1.calculate_hash(), a2.calculate_hash());
    }
}