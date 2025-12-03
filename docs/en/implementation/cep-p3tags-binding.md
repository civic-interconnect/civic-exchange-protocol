# CEP P3Tags Binding

> How Exchanges and Provenance Connect Across Civic Systems

## 1. Purpose
CEP and P3Tags serve different but complementary roles:

- **CEP** structures civic information: entities, relationships, and exchanges.  
- **P3Tags** attach provenance to individual artifacts such as documents, filings, attachments, posts, and AI-generated outputs.

The binding defines **how P3Tags reference CEP records**, and how CEP exchanges may embed or associate provenance metadata.

---

## 2. Binding Principles

### 2.1 Loose Coupling
CEP remains usable without P3Tags.  
P3Tags remain usable without CEP.  
The binding is optional but highly recommended for systems involving document workflows or AI-assisted transformations.

### 2.2 Stable Identifiers
Whenever possible, P3Tags reference:

- `entityId` — a CEP entity  
- `exchangeId` — a specific CEP exchange  
- `relationshipId` — if provenance is tied to a structural relationship  

Identifiers allow provenance to persist even as systems evolve.

### 2.3 Provenance at the Artifact Level
CEP describes structured civic records; P3Tags describe **the artifacts flowing through those records**:

- submitted forms  
- attachments  
- FOIA responses  
- meeting packets  
- contracts and amendments  
- AI-generated summaries or assessments  

CEP and P3Tags complement each other by covering different layers of the civic process.

---

## 3. Embedding P3Tags Within CEP Exchanges

A CEP exchange MAY include a `provenance` array:

```json
{
  "exchangeId": "EXCH-2025-000944",
  "exchangeTypeUri": "https://example.org/vocab/exchange/foia-response",
  "content": { "...": "..." },

  "provenance": [
    {
      "p3tagVersion": "1.0.0",
      "timestamp": "2025-01-01T10:03:22Z",
      "createdBy": "US-MN-COUNTY-048",
      "sourceUri": "https://example.gov/records/123.pdf",
      "hash": "sha256-abc123...",
      "transform": "ocr"
    }
  ]
}
```

This does not embed the entire P3Tag schema inside CEP.
It simply references a P3Tag-shaped object.

---

## 4. P3Tags Referencing CEP

A P3Tag MAY reference CEP identifiers:

```json
{
  "p3tagVersion": "1.0.0",
  "entityId": "US-MN-COUNTY-048",
  "exchangeId": "EXCH-2025-000944",
  "hash": "sha256-xyz789...",
  "sourceUri": "...",
  "timestamp": "2025-02-10T15:22:01Z",
  "transform": "summarization"
}
```

This enables:

- document lineage
- audit trails
- linking analysis results back to their civic origin
- verifying whether an artifact corresponds to a known exchange
- connecting AI outputs back to official sources

---

## 5. AI Workflow Considerations

AI systems often:

- extract
- summarize
- classify
- translate
- redact
- cluster
- evaluate

documents and records.

P3Tags allow each transformation to be recorded, while CEP provides:

- the context (entity, exchange type, relationship)
- the structural meaning of the document
- stable IDs to anchor provenance

This enables transparent, reproducible, and auditable AI-assisted civic workflows.

---

## 6. Summary

- CEP structures civic information.
- P3Tags describe the provenance of individual artifacts.
- The binding is minimal, optional, and powerful.
- Together, they support transparency, automation, and accountability across civic systems.


---
