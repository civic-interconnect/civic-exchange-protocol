use std::fs;
use std::path::PathBuf;

use cep_core::schema_registry::find_repo_root;
use cep_core::{CepError, CepResult};
use cep_entity::builder::{build_entity, AttestationInput, EntityBuilderInput};
use cep_entity::entity::EntityRecord;
use serde::Deserialize;
use serde_json::{self, Map, Value};

/// Raw shape from examples-raw/entity/*.json
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
    address: String,
    #[serde(rename = "registrationDate")]
    registration_date: Option<String>,

    // // Extra optional fields for some examples; we ignore them for now
    // #[serde(default)]
    // ein: Option<String>,
    // #[serde(default, rename = "fecCommitteeId")]
    // fec_committee_id: Option<String>,
}

/// Build an EntityBuilderInput from our raw example shape.
/// This is where normalization and defaults are encoded.
fn build_from_raw(raw: RawEntityExample) -> EntityBuilderInput {
    // Basic attestation: we can refine later or make these configurable.
    let attestation = AttestationInput {
        attested_by: "cep-entity:example:ingest".to_string(),
        attestation_timestamp: "2025-12-03T00:00:00.000000Z".to_string(),
        proof_type: Some("ManualAttestation".to_string()),
        proof_value: None,
        verification_method_uri: None,
        proof_purpose: Some("assertionMethod".to_string()),
        anchor_uri: None,
    };

    EntityBuilderInput {
        source_id: Some(raw.entity_id),
        legal_name: raw.legal_name,
        entity_type: Some(raw.entity_type),
        jurisdiction: Some(raw.jurisdiction),
        country_code: raw.country_code,
        address: Some(raw.address),
        registration_date: raw.registration_date,
        status: None,
        attestation,
    }
}

/// Recursively sort JSON object keys to produce canonical JSON.
fn sort_value(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            let mut new_map = Map::new();
            for k in keys {
                let v = map.get(&k).unwrap().clone();
                new_map.insert(k, sort_value(v));
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => {
            Value::Array(arr.into_iter().map(sort_value).collect())
        }
        other => other,
    }
}

/// Produce canonical JSON string (option A) for an EntityRecord.
fn canonicalize_entity(entity: &EntityRecord) -> String {
    let value =
        serde_json::to_value(entity).expect("Failed to convert EntityRecord to serde_json::Value");
    let sorted = sort_value(value);
    serde_json::to_string(&sorted).expect("Failed to serialize canonical JSON")
}

/// Write a pretty JSON EntityRecord and a canonical JSON text file.
fn write_entity_outputs(
    base_path: &PathBuf,
    file_stem: &str,
    entity: &EntityRecord,
) -> CepResult<()> {
    // 1. Pretty JSON for docs / examples
    let pretty_json =
        serde_json::to_string_pretty(entity).expect("Failed to serialize EntityRecord");
    let json_path = base_path.join(format!("{file_stem}.json"));
    fs::write(&json_path, pretty_json).map_err(|e| {
        CepError::Configuration(format!(
            "Failed to write {}: {}",
            json_path.display(),
            e
        ))
    })?;

    // 2. Canonical JSON (single-line, sorted keys) for hash debugging
    let canonical = canonicalize_entity(entity);
    let canonical_path = base_path.join(format!("{file_stem}.canonical.txt"));
    fs::write(&canonical_path, canonical).map_err(|e| {
        CepError::Configuration(format!(
            "Failed to write {}: {}",
            canonical_path.display(),
            e
        ))
    })?;

    Ok(())
}

fn main() -> CepResult<()> {
    // 1. Find repo root
    let repo_root = find_repo_root()?;
    let raw_dir = repo_root.join("examples-raw").join("entity");
    let out_dir = repo_root.join("examples").join("entity");

    println!("Using raw dir: {}", raw_dir.display());
    println!("Using out dir: {}", out_dir.display());

    // 2. Ensure output directory exists
    fs::create_dir_all(&out_dir).unwrap_or_else(|e| {
        panic!("Failed to create {}: {}", out_dir.display(), e);
    });

    // 3. Fixed list of example files
    let files = [
        "municipality_01.json",
        "nonprofit_01.json",
        "pac_01.json",
        "school_district_01.json",
    ];

    // 4. Process each raw example
    for file in &files {
        let path = raw_dir.join(file);
        println!("=== Loading {} ===", path.display());

        let raw_json = fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!("Failed to read {}: {}", path.display(), e);
        });

        let raw: RawEntityExample = serde_json::from_str(&raw_json).unwrap_or_else(|e| {
            panic!(
                "Failed to parse {} as RawEntityExample: {}",
                path.display(),
                e
            );
        });

        let builder_input = build_from_raw(raw);
        let result = build_entity(builder_input)?;

        // derive a stem like "municipality_01" from the filename
        let stem = file.strip_suffix(".json").unwrap_or(file);
        write_entity_outputs(&out_dir, stem, &result.entity)?;
    }

    Ok(())
}
