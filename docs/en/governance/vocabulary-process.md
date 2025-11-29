# Vocabulary Governance Process

This document defines how controlled vocabularies in the Civic Exchange Protocol (CEP) are created, reviewed, versioned, and published. CEP vocabularies include:

- `entity-type`
- `relationship-type`
- `exchange-type`
- `exchange-role`
- `party-role`
- `identifier-scheme`
- `source-system`
- `resolution-method`
- `value-type`

These vocabularies provide the stable semantic backbone for CEP interoperability across government systems, academic research, and third-party implementations.

---

## 1. Purpose of Vocabularies

CEP vocabularies serve three critical functions:

1. **Interoperability:**  
   Terms map to external standards (Popolo, Open Civic Data, OCDS, HSDS, Schema.org, XBRL, W3C PROV).

2. **Semantic Stability:**  
   Terms provide canonical meaning for entity types, role types, relationship types, exchange types, etc.

3. **Verifiable Behavior:**  
   Terms are referenced in canonical URI form inside attested CEP records, affecting hash parity and revision integrity.

---

## 2. Versioning

Vocabulary files follow **independent semantic versioning**:

`<name>.vMAJOR.MINOR.PATCH.json`

Examples:  

```
entity-type.v1.0.0.json
identifier-scheme.v1.0.0.json
```

---

### 2.1 MAJOR
Changes that break compatibility or reinterpret existing terms:
- renaming or removing terms
- redefining meanings
- altering term URIs
- significant structural changes

Requires **unanimous ISB approval** + 6-month deprecation period.

### 2.2 MINOR
Backward-compatible enhancements:
- adding new terms
- adding new mappings
- adding optional properties

Requires **simple majority vote**.

### 2.3 PATCH
No semantic change:
- fixing typos
- adding external references in `seeAlso`
- clarifying descriptions

May be approved by ISB Chair alone.

---

## 3. Adding, Updating, or Deprecating Terms

All vocabulary modifications follow this structured workflow:

### Step 1 — Proposal Submission
A contributor submits a PR that includes:
- updated vocabulary file
- accompanying rationale
- impact assessment on interoperability
- references to external standards (if relevant)

### Step 2 — Technical Review
The Interconnect Standards Board (ISB) reviews:
- term clarity and definition
- hierarchy (`parentTermUri`)
- mappings to external standards (SKOS-style)
- URI stability
- potential namespace collisions
- hash-parity implications

### Step 3 — Vote
Version bump category determines voting requirements (Section 2).

### Step 4 — Merge & Release
Upon approval:
- CI validates JSON format, unique URIs, and mapping integrity
- The vocabulary is merged into `main`
- A new tag is created:
  
  `vocab/<name>/vMAJOR.MINOR.PATCH`

### Step 5 — Deprecation Notices
If a term becomes deprecated:
- it is retained with `"status": "deprecated"`
- `"deprecationNote"` must point to its replacement
- canonical JSON schemas continue accepting it unless a MAJOR bump occurs

---

## 4. Design Principles for Vocabulary Terms

### 4.1 Term URIs MUST be globally stable  
URIs never change after publication.

### 4.2 Labels are human-friendly; codes are machine-friendly  
Examples:
- code: `prime-contract`
- label: `Prime Contract`

### 4.3 Definitions MUST be unambiguous  
Definitions MUST avoid jurisdiction-specific assumptions unless explicitly scoped.

### 4.4 Hierarchies SHOULD be used where meaningful  
Example:
- `subgrant` → parent: `grant-award`
- `subcontract` → parent: `prime-contract`

### 4.5 External mappings SHOULD be included  
Supported mapping types: `exactMatch`, `broadMatch`, `narrowMatch`, `relatedMatch`.

---

## 5. File Placement and Structure

All vocabulary files live under `vocabularies/`.



Each file follows the canonical `cep.vocabulary.schema.json`.

---

## 6. Lifecycle Summary

| Stage | Description | Output |
|-------|-------------|--------|
| Proposal | Contributor suggests new/updated term | Pull Request |
| Review | ISB evaluates definition + mappings | Comments |
| Vote | Approve/reject based on version category | Decision |
| Release | Publish new vocabulary version | New `vX.Y.Z` tag |
| Deprecation | Old terms marked deprecated | Maintained until next MAJOR |

---

## 7. Guiding Objective

The primary objective of CEP vocabulary governance is to **stabilize meaning**, **maximize interoperability**, and **ensure backward compatibility** while allowing the ecosystem to grow with new policy types, relationship structures, data standards, and domain models.


