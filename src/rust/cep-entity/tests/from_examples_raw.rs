//! Example: transform raw entity JSON into full CEP EntityRecord examples.
//!
//! Usage (from repo root):
//!     cargo run -p cep-entity --example dump_from_raw
//!
//! This will:
//!   - read JSON files from `examples-raw/entity/`
//!   - map them into EntityBuilderInput
//!   - call `build_entity` (which generates SNFEI, identifiers, status, etc.)
//!   - write enriched records into `examples/entity/*.json`

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json::Value;

// Use the library crate name (hyphen becomes underscore)
use cep_entity::builder::{AttestationInput, EntityBuilderInput, build_entity};

/// Shape of the raw examples in `examples-raw/entity`.
///
/// These fields are intended to match the current example files:
///
/// - municipality_01.json
/// - nonprofit_01.json
/// - pac_01.json
/// - school_district_01.json
#[derive(Debug, Deserialize)]
struct RawEntityExample {
    #[serde(rename = "entityId")]
    entity_id: String,
    #[serde(rename = "legalName")]
    legal_name: String,
    #[serde(rename = "entityType")]
    entity_type: String,
    #[serde(rename = "jurisdiction")]
    jurisdiction: String,
    #[serde(rename = "countryCode")]
    country_code: String,
    #[serde(rename = "address")]
    address: String,
    #[serde(rename = "registrationDate")]
    registration_date: Option<String>,

    /// Optional extra fields that may or may not be present in raw examples.
    // #[serde(default)]
    // ein: Option<String>,
    // #[serde(default)]
    // fec_committee_id: Option<String>,

    /// Attestation metadata is not present in the raw files today, so we
    /// supply sensible defaults via serde defaults.
    #[serde(default = "default_attested_by")]
    attested_by: String,

    #[serde(default = "default_attestation_timestamp")]
    attestation_timestamp: String,
}

fn default_attested_by() -> String {
    "CEP Demo Attestor".to_string()
}

fn default_attestation_timestamp() -> String {
    // This is just an example timestamp; implementations can override.
    "2025-01-01T00:00:00.000000Z".to_string()
}

/// Map a raw example into the library's EntityBuilderInput.
///
/// This is where we:
///   - carry over the legal name, jurisdiction, etc.
///   - attach a basic attestation input
///   - treat the source system ID as `source_id`
fn build_from_raw(raw: &RawEntityExample) -> EntityBuilderInput {
    let attestation = AttestationInput {
        attested_by: raw.attested_by.clone(),
        attestation_timestamp: raw.attestation_timestamp.clone(),
        // Extended fields are optional and may be filled in by real systems later.
        proof_type: Some("ManualAttestation".to_string()),
        proof_value: None,
        verification_method_uri: None,
        proof_purpose: None,
        anchor_uri: None,
    };

    EntityBuilderInput {
        // Treat the incoming entityId as the source system identifier.
        source_id: Some(raw.entity_id.clone()),
        legal_name: raw.legal_name.clone(),
        entity_type: Some(raw.entity_type.clone()),
        jurisdiction: Some(raw.jurisdiction.clone()),
        country_code: raw.country_code.clone(),
        address: Some(raw.address.clone()),
        registration_date: raw.registration_date.clone(),
        status: None,
        attestation,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Repo-relative paths based on the crate manifest directory.
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let raw_dir = crate_root
        .join("examples-raw")
        .join("entity");

    let out_dir = crate_root
        .join("examples")
        .join("entity");

    fs::create_dir_all(&out_dir)?;

    println!("Reading raw entities from: {}", raw_dir.display());
    println!("Writing enriched entities to: {}", out_dir.display());

    for entry in fs::read_dir(&raw_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !entry.file_type()?.is_file() {
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<unknown>");

        println!("Processing {}", file_name);

        let raw_contents = fs::read_to_string(&path)?;
        let raw: RawEntityExample = serde_json::from_str(&raw_contents)?;

        let builder_input = build_from_raw(&raw);

        // Use the library builder; if building fails, report and skip.
        let build_result = match build_entity(builder_input) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  ERROR: failed to build entity from {}: {}", file_name, e);
                continue;
            }
        };

        // We only serialize the EntityRecord itself here.
        // If EntityRecord is not directly Serialize, you can wrap or adjust as needed.
        let entity_record = build_result.entity;

        // Optionally normalize to a generic JSON Value before writing
        // to avoid any serde-specific quirks.
        let value: Value = serde_json::to_value(&entity_record)?;
        let pretty = serde_json::to_string_pretty(&value)?;

        // Derive an output file name, e.g. municipality_01.entity.json
        let out_name = if let Some(stripped) = file_name.strip_suffix(".json") {
            format!("{}.entity.json", stripped)
        } else {
            format!("{}.entity.json", file_name)
        };

        let out_path = out_dir.join(out_name);
        fs::write(&out_path, pretty)?;

        println!("  -> wrote {}", out_path.display());
    }

    Ok(())
}
