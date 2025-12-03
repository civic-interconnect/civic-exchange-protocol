# Compatibility Matrix

> Alignment of CEP and P3Tags with Existing Civic Standards and Ecosystems

This matrix summarizes how Civic Interconnect integrates with leading civic data standards and common government systems.  
CEP provides the cross-domain structure; P3Tags provide the provenance layer.

---

## Overview Table

| Standard / System | What It Covers | How CEP Integrates | How P3Tags Integrate | Value Added |
|-------------------|----------------|--------------------|----------------------|-------------|
| **Open Civic Data (OCD)** | Jurisdictions, political divisions, IDs | CEP entities reference OCD division IDs; relationships anchored to geography | P3Tags reference jurisdiction identifiers when provenance is location-based | Shared geography layer; consistent identifiers |
| **Popolo** | People, organizations, memberships | CEP entities map to Popolo-style person/organization structures | P3Tags reference actors involved in document creation or modification | Unified identity layer compatible with many civic tools |
| **OCDS (Open Contracting Data Standard)** | Contracting lifecycle (tender > award > contract > implementation) | CEP exchanges wrap OCDS documents or link to specific lifecycle stages | P3Tags attach to RFPs, bids, contracts, amendments | End-to-end traceability across procurement |
| **W3C PROV** | General-purpose provenance ontology | CEP aligns conceptually via entities/activities | P3Tags act as a lightweight PROV profile | Semantic-web compatibility without complexity |
| **FOIA / Public Records Vendor Systems** | Submission, routing, review, redaction, release | CEP exchanges represent request/response workflows | P3Tags track document lineage and redactions | Zero-friction integration; better audit trails |
| **Procurement / ERP Systems** | Contracts, POs, invoices, amendments | CEP structures the entity/relationship context; exchanges represent filings | P3Tags document transformations, OCR, AI assistance | Stronger compliance and cross-system linking |
| **Grant Reporting Systems** | Awards, periodic reports, audits | CEP models multi-stage reporting as exchanges | P3Tags attach to each submitted artifact | Simplified compliance, especially for small jurisdictions |
| **AI / LLM Systems** | Retrieval, summarization, classification, transformation | CEP provides structured context and stable IDs | P3Tags record chain-of-transformation for AI outputs | Transparent AI workflows; verifiable lineage |

---

## Notes

- CEP is intentionally **schema-light and integrative**, not a replacement for existing domain standards.  
- P3Tags remain **artifact-level** and can be used independently.  
- Combined, they enable transparent, interoperable, AI-ready civic data pipelines across institutions, vendors, and research ecosystems.
