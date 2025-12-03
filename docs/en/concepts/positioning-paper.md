# Civic Interconnect Positioning Paper

> CEP and P3Tags as Interoperability Layer for Civic Data Ecosystems

## 1. Introduction
Modern civic systems rely on multiple established standards: Open Civic Data (OCD) for political geography, Popolo for people and organizations, the Open Contracting Data Standard (OCDS) for procurement, and numerous state or federal schemas for grants, budgets, elections, disclosures, and public records.

Each standard addresses its own domain well.  

**What is missing is a cross-domain, provenance-aware interoperability layer.**

The Civic Interconnect framework fills this gap by providing two complementary tools:

- **Civic Exchange Protocol (CEP)** - a lightweight model for representing *entities*, *relationships*, and *exchanges* across civic workflows.
- **P3Tags (Per-Post Provenance Tags)** - a simple, deployable metadata format that attaches provenance to individual documents, posts, filings, attachments, and AI-derived content.

Civic Interconnect does not replace existing standards.  
It enables them to interoperate.

---

## 2. Role of CEP
CEP provides neutral, JSON-first constructs that describe:

- **Entities** - people, organizations, agencies, programs, vendors  
- **Relationships** - membership, jurisdiction, oversight, affiliation  
- **Exchanges** - filings, permits, tenders, responses, amendments, reports

CEP acts as a **transport layer**, allowing domain-specific schemas (e.g., OCDS, state grant systems) to be expressed in a common shape and combined across silos.

CEP does not attempt to redefine deep domain ontologies.  
Its value is structural consistency and cross-domain interoperability.

---

## 3. Role of P3Tags
P3Tags provide per-artifact provenance:

- what the content is  
- who created or modified it  
- what transformations occurred (OCR, summarization, redaction, modeling)  
- which policies applied  
- what source(s) it derived from

Where CEP handles structured civic records, P3Tags handle the *documents and messages* that move through civic processes.

In an AI-rich environment, P3Tags help establish:

- chain of custody  
- reproducibility  
- auditability  
- trustworthiness

P3Tags can be used independently but become dramatically more powerful when linked to CEP entities and exchanges.

---

## 4. Alignment with Existing Standards
Civic Interconnect is intentionally **non-competitive** with existing civic standards:

- CEP entities may reference **OCD Division IDs** for jurisdictions.  
- CEP relationships may align with **Popolo** people/organization structures.  
- CEP exchanges may wrap **OCDS** contracting lifecycle documents.  
- P3Tags may map to **W3C PROV** concepts for semantic-web compatibility.

Each standard continues doing what it does best.  
CEP and P3Tags provide the missing glue.

---

## 5. Why This Matters
The civic ecosystem is facing three converging pressures:

1. **AI integration** - automated agents increasingly summarize, classify, route, and transform civic information.  
2. **Fragmented vendor landscapes** - public records, procurement, grants, meetings, and reporting systems rarely interoperate.  
3. **Transparency expectations** - journalists, auditors, and the public need sourceable, auditable, machine-readable records.

CEP and P3Tags offer a minimal, intuitive, and extensible way to satisfy these pressures without imposing heavy ontologies or requiring system rewrites.

---

## 6. Conclusion
The Civic Interconnect framework is a small set of primitives designed to connect everything else:

- CEP: the cross-domain record structure  
- P3Tags: the provenance header  
- Existing standards: the authoritative domain models  

This positioning enables the civic community to build interoperable, transparent, AI-ready systems that scale from small rural governments to national datasets - without displacing established standards or workflows.
