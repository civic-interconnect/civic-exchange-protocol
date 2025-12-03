# Implementation Guide

This guide provides a practical overview for developers building Civic Exchange Protocol (CEP) implementations in any language.  
It complements the formal schemas and the categorical foundations by describing how to validate, construct, serialize, and verify CEP records in a deterministic and interoperable way.

---

## Technical Assurance

CEP ensures technical correctness through two mandatory components:

### A. The Canonical String (The Debug Tool)

Every implementation must expose a function (e.g., `getCanonicalString`, `to_canonical_string`, or `generate_canonical_string`) that returns the **raw, unhashed, deterministic** string representation of a CEP record.

- Strict field ordering  
- UTC timestamps with microsecond precision  
- Deterministic numeric formatting  
- No locale or OS artifacts  

This is the ground truth for resolving cross-language hash mismatches.

### B. The Certification Test Suite (The Compliance Gate)

All implementations must pass the cross-language hash-parity suite in `/test_vectors`.

Any system that computes a different SHA-256 hash for a canonical test vector is non-conforming.

---

## Getting Certified

1. Read `/specifications`.  
2. Select the implementation folder for your platform (Rust, Python, etc.).  
3. Integrate `TransactionRecord` and `generateValidationHash`.  
4. Run tests with `/test_vectors`.  
5. Use canonical debugging strings to correct mismatch sources.  

---

## Logic Organization

| Package | Focus | Depends On | Artifacts |
|--------|--------|------------|-----------|
| core | Shared utilities | none | hashing, canonicalization, timestamps, errors |
| entity | Entity records | core | `EntityRecord` |
| relationship | Bilateral links | core, entity | `RelationshipRecord` |
| exchange | Flows between entities | core, entity, relationship | `ExchangeRecord` |

---

## 1. Implementation Goals

A correct CEP implementation MUST:

1. Produce canonical JSON matching the schemas  
2. Achieve full hash parity across languages  
3. Verify attestations  
4. Maintain immutable hash-linked revision chains  
5. Support multi-scheme identifiers  
6. Compose provenance deterministically  

---

## 2. Canonical Serialization

CEP uses canonical JSON for:

- hash computation  
- digital signatures  
- verification  
- cross-node equality  

### 2.1 Requirements

Canonical JSON MUST:

- Sort all object keys lexicographically  
- Use UTF-8  
- Serialize timestamps as UTC with microsecond precision and trailing `Z`  
- Avoid superfluous whitespace  
- Use stable ordering inside arrays where applicable  

Example: `2025-09-15T14:03:22.500000Z`

### 2.2 Canonical Field Order

Field order is enforced via CI and cross-language tests.  
Any deviation produces a hash mismatch.

---

## 3. Record Model and Encapsulation Philosophy

CEP defines **record-shaped data**, not object-oriented domain objects.  
The goal is **interoperable, predictable, schema-driven** structures that behave identically in:

- Rust  
- Python  
- TypeScript  
- Java / C#  
- SQL and NoSQL databases  

### 3.1 Why CEP Records Use Public Fields

CEP records are transparent because:

- Auditors must inspect them directly  
- Schemas define their shape exactly  
- Canonicalization requires predictable visibility  
- Multi-language parity demands structural simplicity  
- Hidden or computed fields would break determinism  

Thus CEP avoids private state and getters/setters.

### 3.2 Where Logic Belongs: Builders and Validators

CEP enforces correctness *outside* the record struct:

- **Builders** (e.g., `EntityBuilder`, `RelationshipBuilder`)  
  - enforce invariants  
  - normalize input  
  - generate identifiers (SNFEI)  
  - ensure field completeness  

- **Validators**  
  - validate schema compliance  
  - enforce vocabulary correctness  
  - verify signatures  
  - enforce revision chain rules  

- **Canonicalization**  
  - enforces deterministic ordering  
  - produces canonical strings for hashing  

### 3.3 When Methods Are Appropriate

Methods are acceptable when they:

- produce **derived** values (e.g., `canonical_string()`)  
- do not mutate underlying data  
- increase clarity without altering canonical shape  

### 3.4 Takeaway

> **CEP records are stable public data structures.  
> Builders and validators enforce correctness.  
> Canonicalization enforces determinism.  
> This ensures interoperability, auditability, and future-proof evolution.**

---

## 4. Attestation and Verification

### 4.1 Attestation Block

Each CEP record includes:

- `attestorId`  
- `attestationTimestamp`  
- `proofType`  
- `proofValue`  
- `verificationMethodUri`  
- `proofPurpose`  
- `anchorUri` (optional)  

### 4.2 Verification Workflow

1. Resolve public key from `verificationMethodUri`  
2. Recompute canonical JSON excluding attestation block  
3. Verify signature using `proofType`  
4. Check signature matches the canonical hash  

Failures MUST cause rejection.

---

## 5. Revision and Hash Chain

### 5.1 Lifecycle

```
revision 1: previousRecordHash = null
revision 2+: previousRecordHash = SHA256(canonical previous)
```

### 5.2 Requirements

Implementations MUST:

- Enforce monotonic revision numbers  
- Reject incorrect previousRecordHash values  
- Treat any modification as a new revision  

This forms a tamper-evident chain.

---

## 6. Identifier Interoperability

CEP supports:

- UEI  
- LEI  
- SNFEI  
- Canadian BN  
- Additional scheme-based identifiers  

### 6.1 Best Practices

- Validate URIs using the identifier-scheme vocabulary  
- Validate known schemes strictly  
- Allow unknown schemes if structurally valid  

---

## 7. Provenance Composition

Relationships and exchanges form a directed provenance graph.

Implementations MUST:

- Validate relationship links  
- Build provenance chains deterministically  
- Support parent relationships/exchanges  

### 7.1 Funding Chain Convention

`FEDERAL>STATE>LOCAL`

Segments must be uppercase, separated by `>`.

---

## 8. Vocabulary Integration

Vocabulary URIs MUST resolve to known terms:

- relationship-type  
- exchange-type  
- party-role  
- exchange-role  
- identifier-scheme  

Implementations SHOULD cache vocabularies locally.

---

## 9. Source References

Source references link CEP to external datasets.

Implementations SHOULD:

- validate URI syntax  
- enforce nonempty IDs  
- optionally verify URL resolvability  

---

## 10. Example Implementation Pattern

```
load_schemas()
load_vocabularies()

record = parse_input_json()
validate_schema(record)
validate_vocabularies(record)

canonical = canonicalize_json(record without attestation)
check_revision_chain(canonical, record)
verify_attestation(canonical, record.attestation)

store(record)
```

---

## 11. Language-Specific Notes

### Python
`json.dumps(..., separators=(',', ':'), sort_keys=True)`

### TypeScript
Use deterministic-stringify libraries.

### Rust
Use `serde_json::to_writer` with sorted maps.

### Java / C#
Use custom deterministic serializers.

All languages MUST yield identical canonical bytes.

---

## 12. Conformance Levels

| Level | Meaning |
|-------|---------|
| **Basic** | Validates schemas + vocabularies |
| **Full** | Validates attestations + revision chains |
| **Verifying Node** | Maintains verified subcategory |
| **Authoritative Node** | Issues new attestations |

---

## 13. Summary

A complete CEP implementation MUST:

✔ Validate schemas  
✔ Canonicalize deterministically  
✔ Verify cryptographic attestations  
✔ Maintain hash-linked revisions  
✔ Interpret vocabularies correctly  
✔ Support provenance graph construction  
✔ Achieve hash parity across languages  

This ensures global interoperability within the Civic Graph.

---
