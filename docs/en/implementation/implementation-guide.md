# Implementation Guide

This guide provides a practical overview for developers building Civic Exchange Protocol (CEP) implementations in any language.
It complements the formal schemas and the categorical foundations by describing how to validate, construct, serialize, and verify CEP records in a deterministic and interoperable way.

## Technical Assurance

The core technical sanity of the standard is managed through two mandatory components that ensure every implementing system generates an identical cryptographic hash for the same payload:

### A. The Canonical String (The Debug Tool)

Every implementation must expose a function (e.g., getCanonicalString or generate_canonical_string) that returns the raw, unhashed, deterministic string representation of the data payload.

- This function strictly enforces field ordering, date/time precision (microseconds, UTC), and monetary formatting (fixed two decimals, invariant culture).
- By comparing this raw string across Python, Java, Rust, and C#, implementers can instantly identify byte-for-byte serialization errors that would otherwise result in a cryptic hash mismatch.

### B. The Certification Test Suite (The Compliance Gate)

This repo includes shared test vectors that all language implementations must pass.

- Any vendor or government agency implementing a CEP Node must execute their system's generateValidationHash function against the payloads provided in the /test_data directory.
- The resulting SHA-256 hash must exactly match the expected output hash provided in the test vector files. This automated process hopes to help reduce costly manual audits.


## Getting Certified

To achieve Node Certification, follow the process below:

1. Read the /specifications directory.
2. Select the folder corresponding to your primary platform (e.g., src/rust, src/python, src/java).
3. Integrate the TransactionRecord model and the generateValidationHash function into your system.
4. Execute the unit tests using the shared payloads from /test_vectors.
5. Use the getCanonicalString() debugging function to resolve any hash mismatches until all tests pass.

## Logic Organization

The core logic is divided into four main packages:

| Package Name | Domain Focus | Dependencies | Artifacts |
|---|---|---|---|
| core | Shared Utilities & Types | None | Common cryptographic helpers (SHA-256 utils), base types (UEI), and error handling definitions. |
| entity | Civic Entity (CE) | core | Defines Entity records (UEI attestation, status). |
| relationship | Civic Relationship (CR) | core, entity | Defines legal relationships between attested entities. |
| exchange | Civic Exchange (CX) | core, entity, relationship | Defines the flow between entities bound by a relationship. |

---

## 1. Implementation Goals

A correct CEP implementation MUST:

1. Produce **canonical JSON** matching the CEP schemas.
2. Achieve **hash parity** with all certified implementations.
3. Perform **attestation verification** using public keys.
4. Maintain a correct **immutable revision chain**.
5. Support **identifier interoperability** (UEI, LEI, OCD IDs, etc.).
6. Correctly handle **provenance composition** using relationships and exchanges.

---

## 2. Canonical Serialization

CEP relies on canonical JSON for:
- hash computation
- signature generation
- signature verification
- deterministic comparison across nodes

### 2.1 Requirements

An implementation MUST:

- Sort all object keys lexicographically.
- Sort key-value pairs in `termsAttributes` and `additionalSchemes`.
- Emit no trailing commas.
- Emit deterministic formatting:
  - UTF-8
  - No extra whitespace outside JSON rules
- Serialize timestamps in:
  - UTC
  - Always microsecond precision
  - Always `Z` suffix

Example: `2025-09-15T14:03:22.500000Z`


### 2.2 Canonical Field Order

Each schema clearly defines required fields and object structure.  
Field order MUST NOT vary once serialized.

This is enforced via CI **hash-parity tests** across all languages.

---

## 3. Attestation and Verification

### 3.1 Attestation Block

Every CEP Entity, Relationship, and Exchange contains an `attestation` block:
- `attestorId`  
- `attestationTimestamp`  
- `proofType`  
- `proofValue`  
- `verificationMethodUri`  
- `proofPurpose`  
- `anchorUri` (optional)

### 3.2 Verification Workflow

A verifier MUST:

1. Resolve the public key from `verificationMethodUri`.
2. Recompute canonical JSON excluding the `attestation` block.
3. Validate the signature using the declared `proofType`.
4. Confirm that the signature matches the hash of the canonical record.

If verification fails:
- the record MUST be rejected
- the node MUST NOT include it in its dataset

---

## 4. Revision and Hash Chain

### 4.1 Record Lifecycle

For each entity or relationship:

```
revision 1: previousRecordHash = null
revision 2+: hash of previous canonical record
```


### 4.2 Requirements

Implementations MUST:

- Enforce monotonic revision numbers.
- Reject any record with an incorrect previousRecordHash.
- Treat each change as a new revision (even minor corrections).

### 4.3 Why This Matters

This chain forms:
- a tamper-evident audit log
- the categorical “amendment” morphism chain
- a verified provenance trail

---

## 5. Identifier Interoperability

CEP Entity identifiers include:

- `samUei`
- `lei`
- `snfei`
- `canadianBn`
- **`additionalSchemes`**: the main interop surface

### 5.1 Best Practice

Implementations SHOULD:

- Enforce scheme URIs contained in the `identifier-scheme` vocabulary.
- Perform validation on well-known schemes (UEI, LEI, OCD IDs).
- Allow permissive acceptance of unknown schemes if format is valid.

---

## 6. Provenance Composition

Relationships and exchanges form directed edges.

Implementations MUST:

- Validate `relationshipId` links when constructing exchanges.
- Build `provenanceChain` deterministically.
- Support `parentRelationshipId` and `parentExchangeId`.

### 6.1 Funding Chain

Funding chain tags MUST:
- use uppercase segments
- be separated by `>`
- match the derived provenance graph

Example: `FEDERAL>STATE>LOCAL`


---

## 7. Vocabulary Integration

Each URI reference MUST map to a term in the appropriate vocabulary:

- `relationshipTypeUri` → `relationship-type` vocabulary  
- `exchangeTypeUri` → `exchange-type` vocabulary  
- `party-role` and `exchange-role` vocabularies  
- `identifier-scheme` for external IDs

Implementations SHOULD cache vocabularies locally to avoid network dependencies.

---

## 8. Source References

A record's `sourceReferences` connects CEP data to external systems:

- Open Civic Data (bills, votes, events)
- USAspending
- state procurement portals
- HSDS registries
- campaign finance systems
- XBRL financial filings

Implementation SHOULD:
- validate URI format
- ensure IDs are nonempty
- verify URLs resolve when possible

---

## 9. Example Implementation Pattern

### 9.1 Pseudocode Workflow

```
load_schemas()
load_vocabularies()

record = parse_input_json()
validate_schema(record)
validate_vocabulary_uris(record)

canonical = canonicalize_json(record without attestation)
validate_previous_record_hash()
verify_attestation(canonical, record.attestation)

store_record(record)
```


---

## 10. Language-Specific Notes

### Python
- Use `json.dumps(..., separators=(',', ':'), ensure_ascii=False)`
- Ensure key ordering via `sort_keys=True`

### TypeScript
- Avoid `JSON.stringify` without a stable stringify library

### Rust
- Use `serde_json::to_writer` with sorted maps

### Java / C#
- Use custom serializers enforcing sorted keys

All languages MUST produce identical byte output for canonical JSON.

---

## 11. Conformance Levels

| Level | Meaning |
|-------|---------|
| **Basic** | Validates schemas + vocabularies |
| **Full** | Also validates attestations + hash chains |
| **Verifying Node** | Maintains complete verified subcategory |
| **Authoritative Node** | Can generate new attestations |

---

## 12. Summary

A complete CEP implementation MUST:

✔ Validate schemas  
✔ Canonicalize JSON deterministically  
✔ Verify all cryptographic attestations  
✔ Maintain immutable hash chains  
✔ Interpret vocabulary URIs as semantic types  
✔ Support provenance composition  
✔ Preserve hash parity with all certified nodes  

Following this guide helps ensure an implementation will integrate into the global **Civic Graph** and remain interoperable, verifiable, and future-proof.

---
