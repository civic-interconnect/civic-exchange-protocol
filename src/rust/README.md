# Rust Implementation

civic-exchange-protocol/
├── Cargo.toml                          # Workspace manifest
├── docs/
│   └── category-theory-foundation.md   # Formal categorical semantics
├── schemas/
│   ├── cep.entity.schema.json
│   ├── cep.exchange.schema.json
│   ├── cep.relationship.schema.json
│   └── cep.vocabulary.schema.json
├── vocabulary/
│   └── relationship-type.json          # 15 relationship types
└── src/rust/
    ├── cep-core/                        # ~600 lines
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── attestation.rs           # Proof types, W3C VC alignment
    │       ├── canonical.rs             # Canonicalize trait, BTreeMap ordering
    │       ├── error.rs                 # CepError, CepResult
    │       ├── hash.rs                  # SHA-256 utilities
    │       └── timestamp.rs             # Microsecond-precision UTC
    ├── cep-entity/                      # ~740 lines
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs
    │       ├── entity.rs                # EntityRecord struct + tests
    │       └── identifiers.rs           # SAM UEI, LEI, SNFEI, etc.
    ├── cep-exchange/                    
    │   ├── Cargo.toml
    │   └── src/
    │       ├── exchange.rs
    │       ├── lib.rs
    │       ├── provenance.rs    
    │       └── value.rs           
    └── cep-relationship/                # ~810 lines
        ├── Cargo.toml
        └── src/
            ├── lib.rs
            ├── bilateral.rs             # PartyA/PartyB with roles
            ├── multilateral.rs          # BTreeSet-based member ordering
            └── relationship.rs          # RelationshipRecord struct + tests

## Design

1. Canonicalize trait in cep-core defines the contract all record types implement
2. BTreeMap/BTreeSet everywhere guarantees alphabetical field ordering for hash stability
3. Test vectors are output by test_vector_* tests. Run `cargo test -- --nocapture` to see canonical strings and hashes
4. Nested objects are serialized as their canonical strings (not JSON) to maintain control over formatting
5. Amount formatting always uses exactly 2 decimal places
6. Timestamp formatting always uses exactly 6 decimal places (microseconds)