# Category Theory Foundation for CEP

## Overview

This document provides a formal categorical semantics for the Civic Exchange Protocol (CEP).
The goal is to prove that the system's design is mathematically sound, ensuring that compositions of civic relationships and exchanges behave predictably and that the Verifiable ID system constitutes a proper universal construction.

## 1. The Category **Civic**

We define a category **Civic** as follows:

### Objects

The objects of **Civic** are **attested civic entities**.
Each object E represents:
- A verified legal entity (government agency, contractor, nonprofit, individual)
- With a canonical Verifiable ID
- At a specific point in time (revision number)

Formally: `Ob(Civic) = { E | E is a valid CEP entity record }`

### Morphisms

The morphisms in **Civic** are **relationships and exchanges** between entities.
A morphism `f: A → B` represents a directed flow of legal obligation, value, or authority from entity A to entity B.

There are two kinds of morphisms:

1. **Relationship morphisms**: Represent the legal basis for interaction
   - `grant: FederalAgency → StateAgency`
   - `contract: Agency → Contractor`
   - `subcontract: PrimeContractor → Subcontractor`

2. **Exchange morphisms**: Represent actual value flows within relationships
   - `disburse: GrantingAgency → Grantee`
   - `pay: ContractingOffice → Vendor`

### Identity Morphism

For each entity E, the identity morphism `id_E: E → E` represents the entity's self-attestation—the record where the entity attests to its own existence and status.
This is the initial entity record with `previousRecordHash = null`.

### Composition

Given morphisms `f: A → B` and `g: B → C`, the composition `g ∘ f: A → C` represents the **compositional provenance chain**.

**Example**: Federal grant flows through a state to a local school district:

```
f: FederalDOE → StateEducationDept    (grant-award relationship)
g: StateEducationDept → LocalDistrict  (subgrant relationship)

g ∘ f: FederalDOE → LocalDistrict      (derived provenance)
```

This composition is captured in the `provenanceChain.fundingChainTag` field as `FEDERAL>STATE>LOCAL`.

### Associativity

Composition must be associative: `(h ∘ g) ∘ f = h ∘ (g ∘ f)`

For provenance chains:
```
FED → STATE → COUNTY → CONTRACTOR

((FED → STATE) → COUNTY) → CONTRACTOR = (FED → STATE) → (COUNTY → CONTRACTOR)
```

Both yield the same ultimate provenance: funds originating from FED, flowing through STATE and COUNTY, to CONTRACTOR.
The `fundingChainTag` is identical regardless of grouping.

**Implementation guarantee**: The `parentRelationshipId` and `parentExchangeId` fields create an explicit linked list that ensures associativity.
You can trace back from any exchange to its ultimate source by following the chain.

## 2. The Verifiable ID as a Universal Property

### The Problem: Multiple Representations

A single real-world entity may appear in many source systems with different identifiers:
- SAM.gov: `J6H4FB3N5YK7`
- State campaign finance: `VENDOR-2024-0093`
- County procurement: `Acme Consulting LLC` (no ID, just a name)
- Federal LEI system: `5493001KJTIIGC8Y1R12`

Each source system defines its own category of records, call them **SAM**, **StateCF**, **CountyProc**, **LEI**.

### The Diagram to Commute

We have partial identity mappings (entity resolution functions) between these systems:
```
        SAM
         ↑
         |  φ_sam
         |
      ENTITY  ←---------- LEI
         |      φ_lei
         |
         ↓  φ_county
    CountyProc
```

Each `φ_X` is a functor from the source category to **Civic** that maps source records to entities.

### The Limit Construction

The Entity `verifiableId` acts as the **limit** (specifically, a **product** in the relevant slice category) of the diagram of source identifiers.

**Definition**: For an entity E appearing in source systems S₁, S₂, ..., Sₙ with identifiers id₁, id₂, ..., idₙ respectively, the Verifiable ID is the unique object V equipped with projections:
```
π_1: V → S₁   (maps V to id₁)
π_2: V → S₂   (maps V to id₂)
...
π_n: V → Sₙ   (maps V to idₙ)
```

Such that for any other object W with maps to all Sᵢ, there exists a unique map `u: W → V` making all triangles commute.

**In CEP terms**: The `identifiers` object in entity is exactly this limit, it holds all known identifiers for the entity:

```json
"identifiers": {
  "samUei": "J6H4FB3N5YK7",
  "lei": "5493001KJTIIGC8Y1R12",
  "snfei": "a3b2c1d4e5f6...",
  "additionalSchemes": [
    {"schemeUri": "https://state.gov/cf", "value": "VENDOR-2024-0093"}
  ]
}
```

### The Universal Property Guarantee

The `verifiableId` (`entity:{scheme}:{value}`) provides the universal property:

1. **Existence**: For any entity in any source system, there exists an entity record with a verifiableId that subsumes all its known identifiers.

2. **Uniqueness**: If two entity records claim to represent the same entity (have overlapping source identifiers), they MUST have the same verifiableId—or one is an error requiring resolution.

3. **Confidence-Weighted Resolution**: The `resolutionConfidence.score` field acknowledges that in practice, entity resolution is probabilistic. A score of 1.0 indicates authoritative identity (the entity self-attested or the source is canonical). Lower scores indicate probabilistic matching.

## 3. Functors: The Bridge to External Standards

### The XBRL Functor

Define `F_xbrl: Civic → XBRL` as a functor mapping:
- entities → XBRL reporting entities
- exchanges → XBRL fact instances

This functor must preserve composition: if `g ∘ f` is a provenance chain in **Civic**, then `F(g) ∘ F(f)` must be a valid XBRL reporting chain.

**Implementation**: The `categorization.gtasAccountCode` field in exchange records provides the data needed for this functor to operate—it maps CEP exchanges to Treasury reporting concepts.

### The W3C PROV Functor

Define `F_prov: Civic → PROV` mapping:
- entities → `prov:Entity`
- relationships → `prov:Activity` (the act of establishing a relationship)
- exchanges → `prov:Activity` (the act of exchanging value)
- Attestations → `prov:Agent` + `prov:wasAttributedTo`

The `attestation` block in every CEP record provides exactly the data needed for this mapping:
- `attestorId` → `prov:Agent`
- `attestationTimestamp` → `prov:atTime`
- `proofValue` → provenance integrity proof

## 4. The Amendment Chain as a Categorical Construction

### The Category of Revisions

For a given entity E, define the category **Rev(E)** where:
- Objects are revisions: E₁, E₂, E₃, ...
- Morphisms are amendment relationships: `amend: Eᵢ → Eᵢ₊₁`

This forms a **total order** (a thin category where there's at most one morphism between any two objects).

### Hash Chains as Functors

The `previousRecordHash` field defines a functor `H: Rev(E) → HashChain` where **HashChain** is the category of SHA-256 hash values with "derived-from" morphisms.

**Preservation property**: If `Eᵢ →amend Eᵢ₊₁`, then `H(Eᵢ)` is embedded in `Eᵢ₊₁.previousRecordHash`, creating an immutable audit trail.

This is the categorical equivalent of a blockchain's hash chain, but without requiring distributed consensus—the attesting node is responsible for chain integrity.

## 5. The Slice Category for Jurisdictional Scoping

### Jurisdictional Restriction

For a given jurisdiction J (e.g., `US-CA`), define the **slice category** `Civic/J` where:
- Objects are entities with `jurisdictionIso` compatible with J
- Morphisms are relationships/exchanges within that jurisdiction

This allows queries like "show me all contracts in California" to be formalized as working within `Civic/US-CA` rather than searching all of **Civic**.

### The Inclusion Functor

The forgetful functor `U: Civic/J → Civic` embeds jurisdictional data back into the global category, enabling cross-jurisdictional queries while preserving local structure.

## 6. Verification: The Proof Subcategory

### Defining Verification

A **verified** object or morphism is one where:
1. The `attestation.proofValue` is cryptographically valid
2. The `attestation.verificationMethodUri` resolves to a valid public key
3. If `anchorUri` is provided, the anchor can be independently verified

### The Subcategory of Verified Records

Define **Civic_verified** ⊂ **Civic** as the full subcategory of verified records.

The inclusion `I: Civic_verified → Civic` is faithful (injective on morphisms), meaning verification status is preserved under composition.

**Practical implication**: If both `f: A → B` and `g: B → C` are verified, then `g ∘ f: A → C` can be marked as having verified provenance—the entire chain is trustworthy.

## 7. Summary: Why Category Theory?

| Categorical Concept | CEP Implementation | Benefit |
|---------------------|---------------------|---------|
| Objects | entities | Formalized identity |
| Morphisms | relationships, exchanges | Typed, directed flows |
| Composition | `parentRelationshipId`, `fundingChainTag` | Provenance tracing |
| Identity | Self-attestation (revision 1) | Entity lifecycle start |
| Limit/Universal Property | Verifiable ID + `identifiers` | Canonical identity resolution |
| Functors | XBRL, PROV mappings | Interoperability |
| Slice categories | Jurisdictional scoping | Efficient local queries |
| Hash chain functor | `previousRecordHash` | Immutable audit trail |

### The Core Theorem

**Theorem**: The CEP system, as defined by the entity, relationship, and exchange schemas with their attestation and hash chain requirements, forms a well-defined category **Civic** with:
1. A universal property for entity identity (the Verifiable ID limit)
2. Associative composition for provenance chains
3. Faithful functors to external standards (XBRL, PROV)
4. A verified subcategory preserving cryptographic integrity

**Corollary**: Any implementation that correctly generates canonical strings and validates attestations will produce records that compose correctly in the categorical sense—provenance chains will be traceable, amendments will be auditable, and cross-system identity will be resolvable.

---

## Appendix: Diagrammatic Notation

### Basic Composition
```
    grant           subgrant
FED -----→ STATE ----------→ LOCAL

         composed to:

         federal-to-local
FED ------------------------→ LOCAL
```

### The Limit Diagram for Verifiable ID
```
                  ┌─── SAM.gov record
                  │
                  │    π_sam
                  ↓
    LEI record ───→ Entity (Verifiable ID) ←─── County record
                  ↑
                  │    π_state  
                  │
                  └─── State CF record
```

### The Amendment Chain
```
E₁ ──amend──→ E₂ ──amend──→ E₃ ──amend──→ E₄
│              │              │              │
│ hash         │ hash         │ hash         │
↓              ↓              ↓              ↓
H₁ ─────────→ H₂ ─────────→ H₃ ─────────→ H₄
     (H₁ in        (H₂ in        (H₃ in
      E₂)           E₃)           E₄)
```

This completes the categorical foundation for CEP. The schemas implement these abstract structures concretely, and the test vectors verify that implementations preserve the categorical properties.