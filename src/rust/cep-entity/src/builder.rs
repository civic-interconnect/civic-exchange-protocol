/// Entity builder: raw data -> canonical CEP Entity.
///
/// Transforms heterogeneous input data into fully-formed EntityRecord
/// with SNFEI generation.
use cep_core::{Attestation, CanonicalTimestamp, CepError, CepResult};
use cep_snfei::{SnfeiResult, apply_localization, generate_snfei};

use crate::{EntityIdentifiers, EntityRecord, EntityStatus, EntityStatusCode};

/// Input for building an entity from raw data.
#[derive(Debug, Clone)]
pub struct EntityBuilderInput {
    /// Source system's entity identifier
    pub source_id: Option<String>,
    /// Legal name of the entity
    pub legal_name: String,
    /// Entity type (e.g., "MUNICIPALITY", "SCHOOL_DISTRICT")
    pub entity_type: Option<String>,
    /// Jurisdiction code (e.g., "US-CA", "US-IL")
    pub jurisdiction: Option<String>,
    /// ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// Street address
    pub address: Option<String>,
    /// Registration/formation date
    pub registration_date: Option<String>,
    /// Optional status input (used by examples and adapters).
    pub status: Option<EntityStatusInput>,
    /// Attestation information
    pub attestation: AttestationInput,
}

/// Input shape for status used by builders and examples.
///
/// This is a lightweight, string-based view that gets mapped into
/// the concrete `EntityStatus` type used on the record.
#[derive(Debug, Clone)]
pub struct EntityStatusInput {
    /// High-level status code (e.g., "ACTIVE").
    pub status_code: String,
    /// When this status became effective (YYYY-MM-DD).
    pub status_effective_date: String,
    /// Optional termination date (YYYY-MM-DD).
    pub status_termination_date: Option<String>,
    /// Optional successor entity identifier.
    pub successor_entity_id: Option<String>,
}

/// Input for attestation data used by builders.
///
/// This is intentionally a *superset* of what early examples need,
/// and it mirrors the JSON attestation block:
///
/// {
///   "attestorId": "cep-entity:demo:attestor-1",
///   "attestationTimestamp": "2025-11-28T14:30:00.000000Z",
///   "proofType": "Ed25519Signature2020",
///   "proofValue": "BASE64_SIGNATURE_EXAMPLE",
///   "verificationMethodUri": "https://example.org/keys/attestor-1#primary",
///   "proofPurpose": "assertionMethod",
///   "anchorUri": null
/// }
#[derive(Debug, Clone)]
pub struct AttestationInput {
    /// Human-readable / logical attestor ID.
    /// This will usually map directly to `attestorId` on the record.
    pub attested_by: String,

    /// When the attestation occurred (ISO 8601, UTC, microsecond precision).
    pub attestation_timestamp: String,

    /// Cryptographic suite or profile.
    /// Example: "Ed25519Signature2020", "JcsEd25519", "ManualAttestation".
    pub proof_type: Option<String>,

    /// Signature or proof payload (base64 / JWS / JSON, depending on suite).
    pub proof_value: Option<String>,

    /// URI to the verification method (DID URL, HTTPS key location, etc.).
    pub verification_method_uri: Option<String>,

    /// Proof purpose (e.g., "assertionMethod", "authentication").
    pub proof_purpose: Option<String>,

    /// Optional anchor (ledger URI, block hash, etc.).
    pub anchor_uri: Option<String>,
}

/// Result of building an entity.
#[derive(Debug, Clone)]
pub struct EntityBuildResult {
    /// The built entity record
    pub entity: EntityRecord,
    /// SNFEI generation result with confidence metadata
    pub snfei_result: SnfeiResult,
    /// Warnings generated during building
    pub warnings: Vec<String>,
}

/// Normalized entity type codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityTypeCode {
    Municipality,
    County,
    State,
    Federal,
    SchoolDistrict,
    SpecialDistrict,
    Other,
}

impl EntityTypeCode {
    /// Parse from raw string.
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "MUNICIPALITY" => Self::Municipality,
            "COUNTY" => Self::County,
            "STATE" => Self::State,
            "FEDERAL" => Self::Federal,
            "SCHOOL_DISTRICT" | "SCHOOLDISTRICT" => Self::SchoolDistrict,
            "SPECIAL_DISTRICT" | "SPECIALDISTRICT" => Self::SpecialDistrict,
            _ => Self::Other,
        }
    }

    /// Convert to canonical string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Municipality => "municipality",
            Self::County => "county",
            Self::State => "state",
            Self::Federal => "federal",
            Self::SchoolDistrict => "school_district",
            Self::SpecialDistrict => "special_district",
            Self::Other => "other",
        }
    }
}

/// Build an entity from raw input data.
///
/// # Arguments
///
/// * `input` - Structured input containing entity fields
///
/// # Returns
///
/// `EntityBuildResult` containing the entity, SNFEI result, and any warnings.
///
/// # Errors
///
/// Returns `CepError` if required fields are missing or invalid.
///
/// let result = build_entity(input)?;
/// println!("SNFEI: {}", result.snfei_result.snfei.value);
/// ```
pub fn build_entity(input: EntityBuilderInput) -> CepResult<EntityBuildResult> {
    let mut warnings = Vec::new();

    // Validate required fields
    validate_required(&input)?;

    // Apply localization
    let jurisdiction = input.jurisdiction.as_deref().unwrap_or(&input.country_code);
    let localized_name = apply_localization(&input.legal_name, jurisdiction);

    // Generate SNFEI
    let snfei_result = generate_snfei(
        &localized_name,
        &input.country_code,
        input.address.as_deref(),
        input.registration_date.as_deref(),
    );

    // Build attestation
    let attestation = build_attestation(&input.attestation)?;

    // Build identifiers with SNFEI
    let identifiers = EntityIdentifiers::new().with_snfei(snfei_result.snfei.clone());

    // Determine entity type
    let entity_type = input
        .entity_type
        .as_deref()
        .map(EntityTypeCode::from_str)
        .unwrap_or(EntityTypeCode::Other);

    // Build status (default to active)
    let status = if let Some(ref status_input) = input.status {
        // For now we only recognize "ACTIVE" explicitly.
        // Everything else is treated as ACTIVE until we add more codes.
        let _code = status_input.status_code.to_uppercase();
        let status_code = EntityStatusCode::Active;

        EntityStatus {
            status_code,
            status_effective_date: status_input.status_effective_date.clone(),
            status_termination_date: status_input.status_termination_date.clone(),
            successor_entity_id: status_input.successor_entity_id.clone(),
        }
    } else {
        // Default to active, using registration date or a baseline.
        EntityStatus {
            status_code: EntityStatusCode::Active,
            status_effective_date: input
                .registration_date
                .clone()
                .unwrap_or_else(|| "1900-01-01".to_string()),
            status_termination_date: None,
            successor_entity_id: None,
        }
    };

    // Build verifiable ID from SNFEI
    let verifiable_id = format!("cep-entity:snfei:{}", snfei_result.snfei.value);

    // Build entity record
    let entity = EntityRecord::new(
        verifiable_id,
        identifiers,
        localized_name,
        jurisdiction.to_string(),
        status,
        attestation,
    );

    // Add warning for unknown entity type
    if input.entity_type.is_some() && entity_type == EntityTypeCode::Other {
        warnings.push(format!(
            "Unknown entity type '{}' normalized to 'other'",
            input.entity_type.as_ref().unwrap()
        ));
    }

    Ok(EntityBuildResult {
        entity,
        snfei_result,
        warnings,
    })
}

fn validate_required(input: &EntityBuilderInput) -> CepResult<()> {
    if input.legal_name.trim().is_empty() {
        return Err(CepError::MissingField("legal_name".to_string()));
    }
    if input.country_code.trim().is_empty() {
        return Err(CepError::MissingField("country_code".to_string()));
    }
    if input.attestation.attested_by.trim().is_empty() {
        return Err(CepError::MissingField(
            "attestation.attested_by".to_string(),
        ));
    }
    if input.attestation.attestation_timestamp.trim().is_empty() {
        return Err(CepError::MissingField(
            "attestation.attestation_timestamp".to_string(),
        ));
    }
    Ok(())
}

fn build_attestation(input: &AttestationInput) -> CepResult<Attestation> {
    let timestamp: CanonicalTimestamp = input
        .attestation_timestamp
        .parse()
        .map_err(|e| CepError::InvalidTimestamp(format!("{}", e)))?;

    // Sensible defaults for examples and non-crypto environments
    let proof_type = input
        .proof_type
        .clone()
        .unwrap_or_else(|| "ManualAttestation".to_string());

    let proof_value = input.proof_value.clone().unwrap_or_default();

    let verification_method_uri = input.verification_method_uri.clone().unwrap_or_else(|| {
        format!(
            "urn:cep:attestor:{}",
            input.attested_by.to_lowercase().replace(' ', "-")
        )
    });

    Ok(Attestation::new(
        input.attested_by.clone(),
        timestamp,
        proof_type,
        proof_value,
        verification_method_uri,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> EntityBuilderInput {
        EntityBuilderInput {
            source_id: Some("US-IL-MUNI-0012".to_string()),
            legal_name: "City of Springfield".to_string(),
            entity_type: Some("MUNICIPALITY".to_string()),
            jurisdiction: Some("US-IL".to_string()),
            country_code: "US".to_string(),
            address: Some("200 Main Street".to_string()),
            registration_date: None,
            status: None,
            attestation: AttestationInput {
                attested_by: "John Smith".to_string(),
                attestation_timestamp: "2024-05-15T14:02:10.491823Z".to_string(),
                proof_type: None,
                proof_value: None,
                verification_method_uri: None,
                proof_purpose: None,
                anchor_uri: None,
            },
        }
    }

    #[test]
    fn test_build_entity_basic() {
        let input = sample_input();
        let result = build_entity(input).unwrap();

        assert!(!result.snfei_result.snfei.value.is_empty());
        assert_eq!(result.snfei_result.snfei.value.len(), 64);
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_build_entity_missing_legal_name() {
        let mut input = sample_input();
        input.legal_name = "".to_string();

        let result = build_entity(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_entity_missing_country_code() {
        let mut input = sample_input();
        input.country_code = "".to_string();

        let result = build_entity(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_entity_unknown_type_warns() {
        let mut input = sample_input();
        input.entity_type = Some("UNKNOWN_TYPE".to_string());

        let result = build_entity(input).unwrap();
        assert_eq!(result.warnings.len(), 1);
        assert!(result.warnings[0].contains("Unknown entity type"));
    }

    #[test]
    fn test_entity_type_parsing() {
        assert_eq!(
            EntityTypeCode::from_str("MUNICIPALITY"),
            EntityTypeCode::Municipality
        );
        assert_eq!(
            EntityTypeCode::from_str("school_district"),
            EntityTypeCode::SchoolDistrict
        );
        assert_eq!(
            EntityTypeCode::from_str("SCHOOLDISTRICT"),
            EntityTypeCode::SchoolDistrict
        );
        assert_eq!(EntityTypeCode::from_str("random"), EntityTypeCode::Other);
    }

    #[test]
    fn test_attestation_building() {
        let input = AttestationInput {
            attested_by: "Maria Lopez".to_string(),
            attestation_timestamp: "2024-05-15T14:02:10.491823Z".to_string(),
            proof_type: None,
            proof_value: None,
            verification_method_uri: None,
            proof_purpose: None,
            anchor_uri: None,
        };

        let attestation = build_attestation(&input).unwrap();
        assert!(attestation.verification_method_uri.contains("maria-lopez"));
    }

    #[test]
    fn test_deterministic_snfei() {
        let input1 = sample_input();
        let input2 = sample_input();

        let result1 = build_entity(input1).unwrap();
        let result2 = build_entity(input2).unwrap();

        assert_eq!(
            result1.snfei_result.snfei.value,
            result2.snfei_result.snfei.value
        );
    }
}
