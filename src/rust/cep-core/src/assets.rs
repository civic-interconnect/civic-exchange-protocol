// src/rust/cep-core/src/assets.rs

// When compiled normally (cargo build, tests, etc), pull in the generated file.
#[cfg(not(rust_analyzer))]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/assets_generated.rs"));
}

// When rust-analyzer is indexing the code (in VS Code / editors), it
// does not run build.rs or set OUT_DIR, so we give it empty stubs.
// This avoids bogus "OUT_DIR not set" macro errors in the editor.
#[cfg(rust_analyzer)]
mod generated {
    pub static SCHEMAS: &[(&str, &str)] = &[];
    pub static VOCABULARIES: &[(&str, &str)] = &[];
    pub static TEST_VECTORS: &[(&str, &str)] = &[];
}

// Re-export raw tables in case advanced callers want them.
pub use generated::{SCHEMAS, VOCABULARIES, TEST_VECTORS};

/// Get a JSON Schema by key.
///
/// For core CEP schemas:
/// - \"cep.entity\"
/// - \"cep.relationship\"
/// - \"cep.exchange\"
/// - \"cep.entity.identifier-scheme\"
/// - \"cep.vocabulary\"
///
/// For any other schemas under `schemas/`, the key is the relative path
/// without `.json`. Examples:
/// - `schemas/entity/examples/minimal.json` -> \"entity/examples/minimal\"
pub fn get_schema(key: &str) -> Option<&'static str> {
    SCHEMAS
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
}

/// Get a vocabulary JSON document by key.
///
/// Keys are the relative paths under `vocabularies/` without `.json`.
/// Example:
/// - `vocabularies/entity-type.json` -> \"entity-type\"
pub fn get_vocab(key: &str) -> Option<&'static str> {
    VOCABULARIES
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
}

/// Get a test vector JSON document by key.
///
/// Keys are the relative paths under `test_vectors/` without `.json`.
pub fn get_test_vector(key: &str) -> Option<&'static str> {
    TEST_VECTORS
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
}
