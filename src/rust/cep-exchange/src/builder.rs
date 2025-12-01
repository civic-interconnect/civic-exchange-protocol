//! Exchange builder: raw data -> canonical CEP ExchangeRecord.
//!
//! Transforms heterogeneous input data into fully-formed ExchangeRecord.

use cep_core::{Attestation, CanonicalTimestamp, CepError, CepResult};

use crate::{
    ExchangeCategorization, ExchangeParty, ExchangeRecord, ExchangeStatus, ExchangeStatusCode,
    ExchangeValue, SourceReference,
};

/// Input for building an exchange from raw data.
#[derive(Debug, Clone)]
pub struct ExchangeBuilderInput {
    /// Unique exchange identifier
    pub exchange_id: String,
    /// Type of exchange (e.g., "GRANT", "CONTRACT", "PAYMENT")
    pub exchange_type: String,
    /// Source entity identifier
    pub source_entity_id: String,
    /// Recipient entity identifier
    pub recipient_entity_id: String,
    /// Exchange amount
    pub amount: f64,
    /// Currency code (e.g., "USD")
    pub currency: String,
    /// When the exchange occurred (date or datetime)
    pub occurred_date: String,
    /// Attestation information
    pub attestation: AttestationInput,
    /// Optional categorization
    pub categorization: Option<CategorizationInput>,
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

/// Input for categorization data.
#[derive(Debug, Clone, Default)]
pub struct CategorizationInput {
    /// CFDA number
    pub cfda_number: Option<String>,
    /// NAICS code
    pub naics_code: Option<String>,
    /// GTAS account code
    pub gtas_account_code: Option<String>,
    /// Local category code
    pub local_category_code: Option<String>,
    /// Local category label
    pub local_category_label: Option<String>,
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

/// Result of building an exchange.
#[derive(Debug, Clone)]
pub struct ExchangeBuildResult {
    /// The built exchange record
    pub exchange: ExchangeRecord,
    /// Warnings generated during building
    pub warnings: Vec<String>,
}

/// Known exchange type URIs.
pub fn exchange_type_uri(exchange_type: &str) -> String {
    match exchange_type.to_uppercase().as_str() {
        "GRANT" => "https://civic-exchange.org/types/grant".to_string(),
        "CONTRACT" => "https://civic-exchange.org/types/contract".to_string(),
        "PAYMENT" => "https://civic-exchange.org/types/payment".to_string(),
        "DONATION" => "https://civic-exchange.org/types/donation".to_string(),
        "FEE" => "https://civic-exchange.org/types/fee".to_string(),
        "TAX" => "https://civic-exchange.org/types/tax".to_string(),
        "TRANSFER" => "https://civic-exchange.org/types/transfer".to_string(),
        _ => format!(
            "https://civic-exchange.org/types/{}",
            exchange_type.to_lowercase()
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

/// Build an exchange from raw input data.
///
/// # Arguments
///
/// * `input` - Structured input containing exchange fields
///
/// # Returns
///
/// `ExchangeBuildResult` containing the exchange and any warnings.
///
/// # Errors
///
/// Returns `CepError` if required fields are missing or invalid.
///
/// # Example
///
/// ```rust
/// use cep_exchange::builder::{
///     build_exchange, ExchangeBuilderInput, AttestationInput, CategorizationInput,
/// };
///
/// let input = ExchangeBuilderInput {
///     exchange_id: "EX-2024-1011".to_string(),
///     exchange_type: "GRANT".to_string(),
///     source_entity_id: "US-FED-ED-001".to_string(),
///     recipient_entity_id: "US-CA-SD-0001".to_string(),
///     amount: 1250000.00,
///     currency: "USD".to_string(),
///     occurred_date: "2024-05-15".to_string(),
///     attestation: AttestationInput {
///         attested_by: "Maria Lopez".to_string(),
///         attestation_timestamp: "2024-05-15T14:02:10.491823Z".to_string(),
///     },
///     categorization: Some(CategorizationInput {
///         local_category_code: Some("ED-TITLEI".to_string()),
///         local_category_label: Some("Title I funding".to_string()),
///         ..Default::default()
///     }),
///     source_reference: None,
///     description: Some("Title I funding allocation".to_string()),
/// };
///
/// let result = build_exchange(input)?;
/// println!("Exchange ID: {}", result.exchange.verifiable_id());
/// ```
pub fn build_exchange(input: ExchangeBuilderInput) -> CepResult<ExchangeBuildResult> {
    let warnings = Vec::new();

    // Validate required fields
    validate_required(&input)?;

    // Parse timestamps
    let occurred_timestamp = parse_timestamp(&input.occurred_date)?;
    let attestation = build_attestation(&input.attestation)?;

    // Build parties
    let source_party = ExchangeParty::new(input.source_entity_id.clone());
    let recipient_party = ExchangeParty::new(input.recipient_entity_id.clone());

    // Build value
    let value = ExchangeValue::monetary(input.amount, &input.currency);

    // Build status
    let status = ExchangeStatus {
        status_code: ExchangeStatusCode::Completed,
        status_effective_timestamp: occurred_timestamp.clone(),
    };

    // Generate relationship ID
    let relationship_id = format!(
        "rel:{}:{}",
        input.source_entity_id, input.recipient_entity_id
    );

    // Build exchange type URI
    let exchange_type_uri = exchange_type_uri(&input.exchange_type);

    // Create base record
    let mut exchange = ExchangeRecord::new(
        input.exchange_id,
        relationship_id,
        exchange_type_uri,
        source_party,
        recipient_party,
        value,
        occurred_timestamp,
        status,
        attestation,
    );

    // Add optional categorization
    if let Some(cat_input) = input.categorization {
        if let Some(categorization) = build_categorization(&cat_input, input.description.as_deref())
        {
            exchange = exchange.with_categorization(categorization);
        }
    } else if let Some(ref desc) = input.description {
        // If no categorization but description provided, create minimal categorization
        let categorization = ExchangeCategorization {
            cfda_number: None,
            naics_code: None,
            gtas_account_code: None,
            local_category_code: None,
            local_category_label: Some(desc.clone()),
        };
        if categorization.has_any() {
            exchange = exchange.with_categorization(categorization);
        }
    }

    // Add optional source reference
    if let Some(ref_input) = input.source_reference {
        let source_ref = SourceReference {
            source_system_uri: ref_input.source_system_uri,
            source_record_id: ref_input.source_record_id,
            source_url: ref_input.source_url,
        };
        exchange = exchange.with_source_reference(source_ref);
    }

    Ok(ExchangeBuildResult { exchange, warnings })
}

fn validate_required(input: &ExchangeBuilderInput) -> CepResult<()> {
    if input.exchange_id.trim().is_empty() {
        return Err(CepError::MissingField("exchange_id".to_string()));
    }
    if input.exchange_type.trim().is_empty() {
        return Err(CepError::MissingField("exchange_type".to_string()));
    }
    if input.source_entity_id.trim().is_empty() {
        return Err(CepError::MissingField("source_entity_id".to_string()));
    }
    if input.recipient_entity_id.trim().is_empty() {
        return Err(CepError::MissingField("recipient_entity_id".to_string()));
    }
    if input.currency.trim().is_empty() {
        return Err(CepError::MissingField("currency".to_string()));
    }
    if input.occurred_date.trim().is_empty() {
        return Err(CepError::MissingField("occurred_date".to_string()));
    }
    if input.attestation.attested_by.trim().is_empty() {
        return Err(CepError::MissingField("attestation.attested_by".to_string()));
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

fn build_categorization(
    input: &CategorizationInput,
    description: Option<&str>,
) -> Option<ExchangeCategorization> {
    let label = input
        .local_category_label
        .clone()
        .or_else(|| description.map(String::from));

    let categorization = ExchangeCategorization {
        cfda_number: input.cfda_number.clone(),
        naics_code: input.naics_code.clone(),
        gtas_account_code: input.gtas_account_code.clone(),
        local_category_code: input.local_category_code.clone(),
        local_category_label: label,
    };

    if categorization.has_any() {
        Some(categorization)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> ExchangeBuilderInput {
        ExchangeBuilderInput {
            exchange_id: "EX-2024-1011".to_string(),
            exchange_type: "GRANT".to_string(),
            source_entity_id: "US-FED-ED-001".to_string(),
            recipient_entity_id: "US-CA-SD-0001".to_string(),
            amount: 1250000.00,
            currency: "USD".to_string(),
            occurred_date: "2024-05-15".to_string(),
            attestation: AttestationInput {
                attested_by: "Maria Lopez".to_string(),
                attestation_timestamp: "2024-05-15T14:02:10.491823Z".to_string(),
            },
            categorization: None,
            source_reference: None,
            description: None,
        }
    }

    #[test]
    fn test_build_exchange_basic() {
        let input = sample_input();
        let result = build_exchange(input).unwrap();

        assert_eq!(result.exchange.verifiable_id(), "EX-2024-1011");
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_build_exchange_with_categorization() {
        let mut input = sample_input();
        input.categorization = Some(CategorizationInput {
            local_category_code: Some("ED-TITLEI".to_string()),
            local_category_label: Some("Title I funding".to_string()),
            ..Default::default()
        });

        let result = build_exchange(input).unwrap();
        assert!(result.exchange.categorization().is_some());
    }

    #[test]
    fn test_build_exchange_missing_exchange_id() {
        let mut input = sample_input();
        input.exchange_id = "".to_string();

        let result = build_exchange(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_exchange_type_uri_known() {
        assert_eq!(
            exchange_type_uri("GRANT"),
            "https://civic-exchange.org/types/grant"
        );
        assert_eq!(
            exchange_type_uri("grant"),
            "https://civic-exchange.org/types/grant"
        );
    }

    #[test]
    fn test_exchange_type_uri_unknown() {
        assert_eq!(
            exchange_type_uri("CUSTOM"),
            "https://civic-exchange.org/types/custom"
        );
    }

    #[test]
    fn test_parse_timestamp_date_only() {
        let ts = parse_timestamp("2024-05-15").unwrap();
        let canonical = ts.to_canonical_string();
        assert!(canonical.contains("2024-05-15"));
        assert!(canonical.contains("T00:00:00"));
    }

    #[test]
    fn test_parse_timestamp_full() {
        let ts = parse_timestamp("2024-05-15T14:02:10.491823Z").unwrap();
        let canonical = ts.to_canonical_string();
        assert!(canonical.contains("2024-05-15"));
        assert!(canonical.contains("14:02:10"));
    }

    #[test]
    fn test_build_attestation() {
        let input = AttestationInput {
            attested_by: "Maria Lopez".to_string(),
            attestation_timestamp: "2024-05-15T14:02:10.491823Z".to_string(),
        };

        let attestation = build_attestation(&input).unwrap();
        assert!(attestation
            .verification_method_uri()
            .contains("maria-lopez"));
    }
}