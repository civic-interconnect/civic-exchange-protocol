/// Relationship builder: raw data -> canonical CEP RelationshipRecord.
///
/// Transforms heterogeneous input data into fully-formed RelationshipRecord.

use cep_core::{Attestation, CanonicalTimestamp, CepError, CepResult};

use crate::{
    BilateralParties, FinancialTerms, Party, RelationshipRecord, RelationshipStatus,
    RelationshipStatusCode, SourceReference,
};

/// Input for building a relationship from raw data.
#[derive(Debug, Clone)]
pub struct RelationshipBuilderInput {
    /// Unique relationship identifier
    pub relationship_id: String,
    /// Type of relationship (e.g., "GRANT_AGREEMENT", "CONTRACT")
    pub relationship_type: String,
    /// Source entity identifier
    pub source_entity_id: String,
    /// Source entity role URI
    pub source_role: Option<String>,
    /// Recipient entity identifier
    pub recipient_entity_id: String,
    /// Recipient entity role URI
    pub recipient_role: Option<String>,
    /// Effective date of relationship
    pub effective_date: String,
    /// Optional termination date
    pub termination_date: Option<String>,
    /// Jurisdiction ISO code (e.g., "US-CA", "GB-ENG")
    pub jurisdiction_iso: String,
    /// Attestation information
    pub attestation: AttestationInput,
    /// Optional financial terms
    pub financial_terms: Option<FinancialTermsInput>,
    /// Optional source reference
    pub source_reference: Option<SourceReferenceInput>,
    /// Optional description
    pub description: Option<String>,
}

/// Input for attestation data.
#[derive(Debug, Clone)]
pub struct AttestationInput {
    /// Who attested to this record
    pub attested_by: String,
    /// When the attestation occurred (ISO 8601)
    pub attestation_timestamp: String,
}

/// Input for financial terms.
#[derive(Debug, Clone)]
pub struct FinancialTermsInput {
    /// Total obligated amount
    pub total_obligated_amount: Option<f64>,
    /// Currency code
    pub currency: String,
    /// Funding instrument type
    pub funding_instrument: Option<String>,
}

/// Input for source reference data.
#[derive(Debug, Clone)]
pub struct SourceReferenceInput {
    /// Source system URI
    pub source_system_uri: String,
    /// Record ID in source system
    pub source_record_id: String,
    /// Optional URL to source
    pub source_url: Option<String>,
}

/// Result of building a relationship.
#[derive(Debug, Clone)]
pub struct RelationshipBuildResult {
    /// The built relationship record
    pub relationship: RelationshipRecord,
    /// Warnings generated during building
    pub warnings: Vec<String>,
}

/// Known relationship type URIs.
pub fn relationship_type_uri(relationship_type: &str) -> String {
    match relationship_type.to_uppercase().as_str() {
        "GRANT_AGREEMENT" | "GRANT" => {
            "https://civic-exchange.org/relationship-types/grant-agreement".to_string()
        }
        "CONTRACT" => "https://civic-exchange.org/relationship-types/contract".to_string(),
        "COOPERATIVE_AGREEMENT" => {
            "https://civic-exchange.org/relationship-types/cooperative-agreement".to_string()
        }
        "INTERGOVERNMENTAL" => {
            "https://civic-exchange.org/relationship-types/intergovernmental".to_string()
        }
        "MEMBERSHIP" => "https://civic-exchange.org/relationship-types/membership".to_string(),
        _ => format!(
            "https://civic-exchange.org/relationship-types/{}",
            relationship_type.to_lowercase().replace('_', "-")
        ),
    }
}

/// Parse a date or datetime string to CanonicalTimestamp.
pub fn parse_timestamp(date_str: &str) -> CepResult<CanonicalTimestamp> {
    let normalized = if !date_str.contains('T') {
        format!("{}T00:00:00.000000Z", date_str)
    } else {
        date_str.to_string()
    };

    normalized
        .parse()
        .map_err(|e| CepError::InvalidTimestamp(format!("{}", e)))
}

/// Build a relationship from raw input data.
///
/// # Arguments
///
/// * `input` - Structured input containing relationship fields
///
/// # Returns
///
/// `RelationshipBuildResult` containing the relationship and any warnings.
///
/// # Errors
///
/// Returns `CepError` if required fields are missing or invalid.
///
/// # Example
///
/// ```rust
/// use cep_relationship::builder::{
///     build_relationship, RelationshipBuilderInput, AttestationInput, FinancialTermsInput,
/// };
///
/// let input = RelationshipBuilderInput {
///     relationship_id: "REL-2024-001".to_string(),
///     relationship_type: "GRANT_AGREEMENT".to_string(),
///     source_entity_id: "US-FED-ED-001".to_string(),
///     source_role: Some("grantor".to_string()),
///     recipient_entity_id: "US-CA-SD-0001".to_string(),
///     recipient_role: Some("grantee".to_string()),
///     effective_date: "2024-01-01".to_string(),
///     termination_date: Some("2024-12-31".to_string()),
///     jurisdiction_iso: "US-CA".to_string(),
///     attestation: AttestationInput {
///         attested_by: "Federal Grants Office".to_string(),
///         attestation_timestamp: "2024-01-15T09:00:00.000000Z".to_string(),
///     },
///     financial_terms: Some(FinancialTermsInput {
///         total_obligated_amount: Some(5000000.00),
///         currency: "USD".to_string(),
///         funding_instrument: Some("GRANT".to_string()),
///     }),
///     source_reference: None,
///     description: Some("Title I Education Grant FY2024".to_string()),
/// };
///
/// let result = build_relationship(input).unwrap();
/// println!("Relationship ID: {}", result.relationship.verifiable_id);
/// ```
pub fn build_relationship(input: RelationshipBuilderInput) -> CepResult<RelationshipBuildResult> {
    let mut warnings = Vec::new();

    // Validate required fields
    validate_required(&input)?;

    // Parse timestamps
    let effective_timestamp = parse_timestamp(&input.effective_date)?;
    let attestation = build_attestation(&input.attestation)?;

    // Build parties
    let source_role = input.source_role.unwrap_or_else(|| "source".to_string());
    let recipient_role = input
        .recipient_role
        .unwrap_or_else(|| "recipient".to_string());

    let source_party = Party::new(input.source_entity_id.clone(), source_role);
    let recipient_party = Party::new(input.recipient_entity_id.clone(), recipient_role);

    let bilateral_parties = BilateralParties::new(source_party, recipient_party);

    
    // Build status
    let status_code = if input.termination_date.is_some() {
        RelationshipStatusCode::Terminated
    } else {
        RelationshipStatusCode::Active
    };

    let status = RelationshipStatus {
        status_code,
        status_effective_timestamp: effective_timestamp.clone(),
    };

    // Build relationship type URI
    let relationship_type_uri = relationship_type_uri(&input.relationship_type);

    // Create base record using the bilateral constructor.
    let mut relationship = RelationshipRecord::new_bilateral(
        input.relationship_id,
        relationship_type_uri,
        bilateral_parties,
        effective_timestamp,
        status,
        input.jurisdiction_iso,
        attestation,
    );

    // Add optional financial terms
    if let Some(fin_input) = input.financial_terms {
        let financial_terms = FinancialTerms {
            total_value: None,
            obligated_value: fin_input.total_obligated_amount,
            currency_code: fin_input.currency,
        };
        relationship = relationship.with_financial_terms(financial_terms);

        if fin_input.funding_instrument.is_some() {
            warnings
                .push("funding_instrument provided but not mapped to FinancialTerms".to_string());
        }
    }

    // Add optional source reference
    if let Some(ref_input) = input.source_reference {
        let source_ref = SourceReference {
            source_system_uri: ref_input.source_system_uri,
            source_record_id: ref_input.source_record_id,
            source_url: ref_input.source_url,
        };
        relationship = relationship.with_source_reference(source_ref);
    }

    Ok(RelationshipBuildResult {
        relationship,
        warnings,
    })
}

fn validate_required(input: &RelationshipBuilderInput) -> CepResult<()> {
    if input.relationship_id.trim().is_empty() {
        return Err(CepError::MissingField("relationship_id".to_string()));
    }
    if input.relationship_type.trim().is_empty() {
        return Err(CepError::MissingField("relationship_type".to_string()));
    }
    if input.source_entity_id.trim().is_empty() {
        return Err(CepError::MissingField("source_entity_id".to_string()));
    }
    if input.recipient_entity_id.trim().is_empty() {
        return Err(CepError::MissingField("recipient_entity_id".to_string()));
    }
    if input.effective_date.trim().is_empty() {
        return Err(CepError::MissingField("effective_date".to_string()));
    }
    if input.jurisdiction_iso.trim().is_empty() {
        return Err(CepError::MissingField("jurisdiction_iso".to_string()));
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

    let verification_method_uri = format!(
        "urn:cep:attestor:{}",
        input.attested_by.to_lowercase().replace(' ', "-")
    );

    Ok(Attestation::new(
        input.attested_by.clone(),
        timestamp,
        "ManualAttestation".to_string(),
        String::new(),
        verification_method_uri,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> RelationshipBuilderInput {
        RelationshipBuilderInput {
            relationship_id: "REL-2024-001".to_string(),
            relationship_type: "GRANT_AGREEMENT".to_string(),
            source_entity_id: "US-FED-ED-001".to_string(),
            source_role: Some("grantor".to_string()),
            recipient_entity_id: "US-CA-SD-0001".to_string(),
            recipient_role: Some("grantee".to_string()),
            effective_date: "2024-01-01".to_string(),
            termination_date: None,
            jurisdiction_iso: "US-CA".to_string(),
            attestation: AttestationInput {
                attested_by: "Federal Grants Office".to_string(),
                attestation_timestamp: "2024-01-15T09:00:00.000000Z".to_string(),
            },
            financial_terms: None,
            source_reference: None,
            description: None,
        }
    }

    #[test]
    fn test_build_relationship_basic() {
        let input = sample_input();
        let result = build_relationship(input).unwrap();

        assert_eq!(result.relationship.verifiable_id, "REL-2024-001");
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_build_relationship_with_financial_terms() {
        let mut input = sample_input();
        input.financial_terms = Some(FinancialTermsInput {
            total_obligated_amount: Some(5000000.00),
            currency: "USD".to_string(),
            funding_instrument: Some("GRANT".to_string()),
        });

        let result = build_relationship(input).unwrap();
        assert!(result.relationship.financial_terms.is_some());
    }

    #[test]
    fn test_build_relationship_missing_id() {
        let mut input = sample_input();
        input.relationship_id = "".to_string();

        let result = build_relationship(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_relationship_type_uri_known() {
        assert_eq!(
            relationship_type_uri("GRANT_AGREEMENT"),
            "https://civic-exchange.org/relationship-types/grant-agreement"
        );
        assert_eq!(
            relationship_type_uri("CONTRACT"),
            "https://civic-exchange.org/relationship-types/contract"
        );
    }

    #[test]
    fn test_relationship_type_uri_unknown() {
        assert_eq!(
            relationship_type_uri("CUSTOM_TYPE"),
            "https://civic-exchange.org/relationship-types/custom-type"
        );
    }

    #[test]
    fn test_terminated_relationship() {
        let mut input = sample_input();
        input.termination_date = Some("2024-12-31".to_string());

        let result = build_relationship(input).unwrap();
        assert_eq!(
            result.relationship.status.status_code,
            RelationshipStatusCode::Terminated
        );
    }
}
