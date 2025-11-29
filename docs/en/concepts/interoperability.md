# Interoperability

CEP is designed to integrate with: 

- Popolo - the entity + political relationships backbone of nearly every open civics project worldwide (OCD, OpenStates, etc.) includes models for Person, Organization, Membership, 
Post / Role, Area, Event, Motion / Vote, Legislative Activity and works for elected officials, models boards, committees, councils, includes membership graph.
- Open Civic Data
- Open Contracting Data Standard (OCDS) - international standard for public procurement, contracts, tenders, awards, suppliers, implementation and milestone tracking.
- Schema.org - includes Organization, Person, Place, GovernmentOrganization, Grant, MonetaryGrant, 
Legislation, VoteAction and enables interoperability with Google datasets, machine readability.
- Financial Taxonomies (XBRL) - good for financial reporting, auditing, interoperability with Treasury and state finance systems
- W3C PROV - good for academics and auditors, PROV guides revision chains, attestation blocks, canonical timestamps, and hash-based integrity. CEP attestation maps to PROV wasGeneratedBy, wasAttributedTo, wasDerivedFrom and CEP previousRecordHash maps to PROV wasRevisionOf.
- Open Referral Human Services Data Specification (HSDS) - has Organization, Location, Service, Funding - enables connecting public funding to outcomes
- Financial Industry Business Ontology (FIBO) - map to loan-agreement, grant-award (loosely), subsidiary.



## CEP Concept Mapping Table

| CEP Concept    | CEP Term / Field     | CEP Vocabulary / Schema    | External Standard | External Concept / Class / Field   | Mapping Type | Notes      |
| ---------- | ------------ | ---------- | ---------- | ---------- | ------------ | --------- |
| Entity (organization)      | `entityTypeUri = #government-jurisdiction`   | `entity-type.json`  | Popolo       | `Organization`      | exactMatch   | Government bodies (legislatures, councils, agencies). |
| Entity (organization)      | `entityTypeUri = #government-jurisdiction`   | `entity-type.json`  | OCD    | `ocd-jurisdiction`  | exactMatch   | Use in `identifiers.additionalSchemes` with OCD jurisdiction IDs.       |
| Entity (organization)      | `entityTypeUri = #government-jurisdiction`   | `entity-type.json`  | Schema.org   | `GovernmentOrganization`      | exactMatch   | For web/SEO and generic data consumers.    |
| Entity (organization)      | `entityTypeUri = #school-district`      | `entity-type.json`  | OCD    | `ocd-division` + `ocd-jurisdiction` (school districts)     | broadMatch   | Division/jurisdiction pair identifies school districts.      |
| Entity (organization)      | `entityTypeUri = #school-district`      | `entity-type.json`  | Schema.org   | `EducationalOrganization` / `SchoolDistrict` (where available)  | exactMatch   | For education analytics and public-facing data.   |
| Entity (organization)      | `entityTypeUri = #nonprofit-organization`    | `entity-type.json`  | Popolo       | `Organization` with classification `nonprofit`  | broadMatch   | Nonprofits providing services, fiscal sponsors, etc.  |
| Entity (organization)      | `entityTypeUri = #nonprofit-organization`    | `entity-type.json`  | Schema.org   | `NGO` / `Organization`   | relatedMatch | Web-compatible representation of nonprofits.      |
| Entity (organization)      | `entityTypeUri = #vendor`    | `entity-type.json`  | OCDS   | `Party` with role `supplier`  | exactMatch   | Contractors / vendors in procurement.  |
| Entity (natural person)    | `entityTypeUri = #natural-person`       | `entity-type.json`  | Popolo       | `Person`      | exactMatch   | Politicians, officials, natural-person donors, etc.   |
| Entity (natural person)    | `entityTypeUri = #elected-official`     | `entity-type.json`  | Popolo       | `Person` + `Membership` in `Organization`       | exactMatch   | Derived from Popolo membership in a legislature / council.   |
| Entity (natural person)    | `entityTypeUri = #natural-person` / `#elected-official` | `entity-type.json`  | Schema.org   | `Person`      | exactMatch   | Generic consumer-friendly alignment.   |
| Entity (division)     | `identifiers.additionalSchemes`  | `cep.entity.schema.json`  | OCD    | `ocd-division/...`  | exactMatch   | Political geography for school districts, counties, states, etc.  |
| Entity (jurisdiction)      | `identifiers.additionalSchemes`  | `cep.entity.schema.json`  | OCD    | `ocd-jurisdiction/...`   | exactMatch   | Governing bodies over divisions.       |
| Entity (organization)      | `identifiers.additionalSchemes`  | `cep.entity.schema.json`  | OCD    | `ocd-organization/...`   | exactMatch   | Committees, councils, agencies, boards.    |
| Entity (person)       | `identifiers.additionalSchemes`  | `cep.entity.schema.json`  | OCD    | `ocd-person/...`    | exactMatch   | People in OCD-compatible civic graphs. |
| Entity (financial)    | `identifiers.lei` | `cep.entity.schema.json`  | LEI    | ISO 17442 Legal Entity Identifier    | exactMatch   | Global financial identity for entities in financial transactions.       |
| Entity (federal)      | `identifiers.samUei`  | `cep.entity.schema.json`  | UEI    | SAM.gov Unique Entity Identifier     | exactMatch   | U.S. federal financial identity for entities.     |
| Entity (SNFEI)  | `identifiers.snfei`   | `cep.entity.schema.json`  | (none, new)  | Hash-based Sub-National Federated Entity Identifier | newConcept   | Bridges UEI/LEI to local civic entities; open-source identifier.  |
| Relationship (contract)    | `relationshipTypeUri = #prime-contract`      | `relationship-type.json`  | OCDS   | `Contract`    | exactMatch   | Prime contract between government and supplier.   |
| Relationship (contract)    | `relationshipTypeUri = #prime-contract`      | `relationship-type.json`  | USAspending  | `contract` (glossary)    | exactMatch   | Alignment with USASpending contract awards.       |
| Relationship (subcontract) | `relationshipTypeUri = #subcontract`    | `relationship-type.json`  | OCDS   | `Contract` linked via `relatedProcesses` | broadMatch   | Subcontracts under a prime contract.   |
| Relationship (grant)  | `relationshipTypeUri = #grant-award`    | `relationship-type.json`  | USAspending  | `grant` (glossary)  | exactMatch   | Federal grants / assistance awards.    |
| Relationship (subgrant)    | `relationshipTypeUri = #subgrant`       | `relationship-type.json`  | OCDS   | `Award` + `Implementation.transactions` for subawards      | broadMatch   | Pass-through grants from state to local entities. |
| Relationship (coop agrmt)  | `relationshipTypeUri = #cooperative-agreement`     | `relationship-type.json`  | USAspending  | `cooperative agreement`  | exactMatch   | Cooperative federal assistance relationships.     |
| Relationship (interagency) | `relationshipTypeUri = #interagency-agreement`     | `relationship-type.json`  | OCDS   | `Contract` or `Agreement` between government parties       | relatedMatch | Transfer agreements between agencies.  |
| Relationship (MOU)    | `relationshipTypeUri = #memorandum-of-understanding`    | `relationship-type.json`  | Popolo       | `Membership` / `Organization` with informal cooperation    | relatedMatch | Non-binding but structured relationships.  |
| Relationship (fiscal spon) | `relationshipTypeUri = #fiscal-sponsorship`  | `relationship-type.json`  | HSDS   | `Funding` / `Service` provider/host relationship    | relatedMatch | Connects projects to 501(c)(3) entities.   |
| Relationship (board)  | `relationshipTypeUri = #board-membership`    | `relationship-type.json`  | Popolo       | `Membership` (person ↔ organization, role = board member)  | exactMatch   | Governance relationships.  |
| Relationship (employment)  | `relationshipTypeUri = #employment`     | `relationship-type.json`  | Popolo       | `Membership` or `Post` with classification `employment`    | exactMatch   | Staff roles in agencies, schools, vendors. |
| Relationship (consulting)  | `relationshipTypeUri = #consulting-engagement`     | `relationship-type.json`  | Popolo       | `Membership` with classification `consultant`   | relatedMatch | Non-employee professional services.    |
| Relationship (subsidiary)  | `relationshipTypeUri = #subsidiary`     | `relationship-type.json`  | Schema.org   | `subOrganization`   | relatedMatch | Corporate ownership within civic vendor trees.    |
| Relationship (joint ven.)  | `relationshipTypeUri = #joint-venture`  | `relationship-type.json`  | FIBO   | Joint venture relationship    | relatedMatch | Optional mapping for advanced financial/corporate semantics.      |
| Relationship (reg. reg.)   | `relationshipTypeUri = #regulatory-registration`   | `relationship-type.json`  | Schema.org   | `GovernmentOrganization` + `registration`-related properties    | relatedMatch | Entities registered with regulatory bodies.       |
| Relationship (loan)   | `relationshipTypeUri = #loan-agreement`      | `relationship-type.json`  | FIBO   | Loan contract       | exactMatch   | For loans, bond-like instruments, and revolving credit.      |
| Relationship (bill-ties)   | `sourceReferences` on Relationship      | `cep.relationship.schema.json` | OCD    | `Bill` object       | relatedMatch | Relationship authorized or shaped by a bill.      |
| Relationship (vote-ties)   | `sourceReferences` on Relationship      | `cep.relationship.schema.json` | OCD    | `Vote` object       | relatedMatch | Relationship activated/approved by a vote. |
| Relationship (event-ties)  | `sourceReferences` on Relationship      | `cep.relationship.schema.json` | OCD    | `Event` object      | relatedMatch | Relationship linked to hearings, meetings, signings.  |
| Exchange (grant)      | `exchangeTypeUri = #grant-disbursement`      | `exchange-type.json`      | OCDS   | `Implementation.transactions` of type grant disbursement   | exactMatch   | Payment of grant funds under a grant-award relationship.     |
| Exchange (contract fee)    | `exchangeTypeUri = #contract-fee-payment`    | `exchange-type.json`      | OCDS   | `Implementation.transactions` of type payment   | exactMatch   | Invoice payments under contracts.      |
| Exchange (subaward)   | `exchangeTypeUri = #subgrant-disbursement`   | `exchange-type.json`      | USAspending  | Subaward transactions    | exactMatch   | Pass-through disbursements to subrecipients.      |
| Exchange (donation)   | `exchangeTypeUri = #campaign-contribution`   | `exchange-type.json`      | Popolo / DIME     | Contribution records mapped to `Person` / `Organization` donors/recipients | relatedMatch | Campaign finance contributions tied into CEP graph.   |
| Exchange (in-kind)    | `value.valueTypeUri = #in-kind`  | `value-type.json`   | Schema.org   | `Offer` / `Grant` with non-monetary value       | relatedMatch | Non-monetary goods/services tracked in CEP.       |
| Exchange (service-hours)   | `value.valueTypeUri = #service-hours`   | `value-type.json`   | HSDS   | `Service` + time-based contributions | relatedMatch | Volunteer or staff time as value.      |
| Exchange (categorization)  | `categorization.naicsCode`   | `cep.exchange.schema.json`     | NAICS  | NAICS industry codes     | exactMatch   | Standard economic activity classification for exchanges.     |
| Exchange (categorization)  | `categorization.cfdaNumber`  | `cep.exchange.schema.json`     | CFDA / Assistance | CFDA / Assistance Listing number     | exactMatch   | Links exchanges to federal assistance programs.   |
| Exchange (categorization)  | `categorization.gtasAccountCode` | `cep.exchange.schema.json`     | GTAS   | Treasury Account Symbol  | exactMatch   | Aligns exchanges to Treasury reporting accounts.  |
| Exchange (source)     | `sourceReferences` on Exchange   | `cep.exchange.schema.json`     | OCDS   | `Release`, `Award`, `Contract`, `Implementation.transactions`   | relatedMatch | Exchange derived from procurement data.    |
| Exchange (bill/vote)  | `sourceReferences` on Exchange   | `cep.exchange.schema.json`     | OCD    | `Bill`, `Vote`, `Event`  | relatedMatch | Funding events tied to legislative activity.      |
| Value (monetary)      | `value.valueTypeUri = #monetary` | `value-type.json`   | XBRL   | Monetary item types (e.g., `monetaryItemType`)  | exactMatch   | Monetary amounts aligned with financial reporting taxonomies.     |
| Value (monetary)      | `value.amount`, `value.currencyCode`    | `cep.exchange.schema.json`     | XBRL   | `xbrli:monetaryItemType`, ISO 4217   | exactMatch   | Strictly typed monetary values. |
| Value (provisioning)  | `valueTypeUri = #service-hours` / `#in-kind`       | `value-type.json`   | HSDS   | `Service` value dimensions    | relatedMatch | For human services and community programs. |
| Provenance (entity)   | `attestation` block on Entity    | `cep.entity.schema.json`  | W3C PROV     | `prov:Entity`, `prov:wasGeneratedBy`, `prov:wasAttributedTo`    | exactMatch   | Entity record as a PROV entity with associated agent/activity.    |
| Provenance (relationship)  | `attestation` block on Relationship     | `cep.relationship.schema.json` | W3C PROV     | Same as above       | exactMatch   | Relationship record provenance in PROV terms.     |
| Provenance (exchange)      | `attestation` block on Exchange  | `cep.exchange.schema.json`     | W3C PROV     | Same as above       | exactMatch   | Exchange record provenance.     |
| Provenance (revision)      | `previousRecordHash`, `revisionNumber`  | all CEP core schemas      | W3C PROV     | `prov:wasRevisionOf`, `prov:qualifiedRevision`  | exactMatch   | Immutable revision chains expressed via hashes and monotone revisions.  |
| Provenance (hash)     | Canonical string + SHA-256 hash (SSOT)  | implementation / spec     | W3C PROV     | `prov:generatedAtTime`, `prov:wasDerivedFrom`   | relatedMatch | Hash as integrity proof supporting PROV-compatible derivation chains.   |
| IDs (UEI)  | `identifiers.samUei`  | `cep.entity.schema.json`  | UEI    | SAM.gov UEI   | exactMatch   | Federal identity.     |
| IDs (LEI)  | `identifiers.lei` | `cep.entity.schema.json`  | LEI    | LEI      | exactMatch   | Global financial identity.      |
| IDs (SNFEI)     | `identifiers.snfei`   | `cep.entity.schema.json`  | (none, new)  | SNFEI    | newConcept   | Open, recomputable sub-federal ID bridging UEI/LEI and local civic entities. |
| IDs (OCD)  | `identifiers.additionalSchemes`  | `cep.entity.schema.json`  | OCD    | `ocd-division`, `ocd-jurisdiction`, `ocd-organization`, `ocd-person`  | exactMatch   | Primary bridge to the Open Civic Data topology.   |
| IDs (OCDS)      | `identifiers.additionalSchemes` on Entity/Relationship  | `cep.entity/relationship`      | OCDS   | `Parties`, `Award.id`, `Contract.id` | relatedMatch | Tie CEP entities/relationships back to OCDS releases. |
| IDs (Popolo)    | `identifiers.additionalSchemes`  | `cep.entity.schema.json`  | Popolo       | `Person.id`, `Organization.id`       | relatedMatch | Where Popolo IDs exist separately from OCD IDs.   |


## CEP Concept Mapping Table (by Schema)

| CEP Concept / Field   | External Standard | External Concept / ID / Class   | Mapping Type | Suggested `externalUri` / `schemeUri` example     | Notes   |
| ------ | -------- | ------------- | ------------ | ----- | ------ |
| **Entity (general civic entity)** | Popolo       | `Person`, `Organization`    | broadMatch   | `http://www.popoloproject.com/specs/person.html`, `http://www.popoloproject.com/specs/organization.html` | CEP `entity` covers both persons and orgs; Popolo separates.     |
| `entityTypeUri` `#natural-person` | Popolo       | `Person`    | exactMatch   | `http://www.popoloproject.com/specs/person.html`  | For individuals (candidates, officials, donors).      |
| `entityTypeUri` `#organization`   | Popolo       | `Organization`    | exactMatch   | `http://www.popoloproject.com/specs/organization.html`      | Base org type, parent of more specific government / nonprofit types.  |
| `entityTypeUri` `#government-jurisdiction`   | Popolo/OCD   | Popolo `Organization` + OCD `jurisdiction`    | relatedMatch | `https://opencivicdata.org/id/jurisdiction`       | A governing body; mix of org and jurisdiction. |
| `identifiers.additionalSchemes` (division)   | OCD    | `ocd-division/*`  | exactMatch   | `https://opencivicdata.org/id/division`     | Store OCD division IDs as `schemeUri = "https://opencivicdata.org/id/division"`. |
| `identifiers.additionalSchemes` (jurisdiction)    | OCD    | `ocd-jurisdiction/*`   | exactMatch   | `https://opencivicdata.org/id/jurisdiction`       | Store OCD jurisdiction IDs.    |
| `identifiers.additionalSchemes` (person)     | OCD / Popolo      | `ocd-person/*` (Popolo person)     | exactMatch   | `https://opencivicdata.org/id/person`  | Link CEP entity to OCD/Popolo person IDs.  |
| `identifiers.additionalSchemes` (organization)    | OCD / Popolo      | `ocd-organization/*`   | exactMatch   | `https://opencivicdata.org/id/organization`       | Link CEP entity to OCD/Popolo org IDs.     |
| `identifiers.lei`      | leif / LEI  | `LEI` (ISO 17442 Legal Entity Identifier)     | exactMatch   | `https://www.gleif.org/en/about-lei/introducing-the-legal-entity-identifier-lei`  | Already in schema; key for global finance interoperability.      |
| `identifiers.samUei`   | US Federal   | UEI (SAM.gov Unique Entity Identifier) | exactMatch   | `https://www.sam.gov/`      | Tier 2 identity in our stack.  |
| `identifiers.additionalSchemes` (OCDS party)      | OCDS   | `parties.identifier.id`     | relatedMatch | `https://standard.open-contracting.org/1.1/en/schema/reference/#parties`      | Use for suppliers / buyers in procurement records.    |
| `entityTypeUri` `#supplier` / `#contractor`  | OCDS   | `Organization` with role `supplier` / `tenderer`  | exactMatch   | `https://standard.open-contracting.org/1.1/en/schema/reference/#parties`      | Straight mapping for vendors.  |
| `entityTypeUri` `#school-district`    | Schema.org   | `SchoolDistrict` (subtype of `EducationalOrganization`)  | relatedMatch | `https://schema.org/SchoolDistrict`    | For education equity analytics.     |
| `entityTypeUri` `#government-agency`  | Schema.org   | `GovernmentOrganization`    | exactMatch   | `https://schema.org/GovernmentOrganization`       | For agencies at any level.     |
| **Relationship (legal / functional relationships)**     | Popolo       | `Membership`, `Post`   | relatedMatch | `http://www.popoloproject.com/specs/membership.html`   | CEP Relationship is more general; includes contracts, grants, etc.    |
| `relationshipTypeUri` `#board-membership`    | Popolo       | `Membership`      | exactMatch   | `http://www.popoloproject.com/specs/membership.html`   | Board membership of a person in an org.    |
| `relationshipTypeUri` `#employment`   | Popolo       | `Membership` (with `post` / `role`)    | broadMatch   | `http://www.popoloproject.com/specs/membership.html`   | Employment is a constrained membership.    |
| `relationshipTypeUri` `#consulting-engagement`    | Popolo       | `Membership` or `ContactDetail`    | relatedMatch | `http://www.popoloproject.com/specs/membership.html`   | Less formal, non-employee service relationship.       |
| `relationshipTypeUri` `#prime-contract`      | OCDS   | `Contract`  | exactMatch   | `https://standard.open-contracting.org/latest/en/schema/reference/#contract`  | CEP Relationship for a prime contract matches OCDS Contract.     |
| `relationshipTypeUri` `#subcontract`  | OCDS   | `Contract` with `relatedProcesses` / `relatedLots`       | narrowMatch  | `https://standard.open-contracting.org/latest/en/schema/reference/#contract`  | Subcontracts are contracts linked to a parent award.  |
| `relationshipTypeUri` `#grant-award`  | OCDS   | `Award`     | relatedMatch | `https://standard.open-contracting.org/latest/en/schema/reference/#award`     | OCDS is procurement-focused; grants are adjacent.     |
| `relationshipTypeUri` `#cooperative-agreement`    | OCDS   | `Award` or `Contract` (context-specific)      | relatedMatch | `https://standard.open-contracting.org/latest/en/schema/reference/`    | Map conceptually to awarded agreements.    |
| `relationshipTypeUri` `#loan-agreement`      | FIBO   | `LoanAgreement`   | exactMatch   | `https://spec.edmcouncil.org/fibo/ontology/FBC/DebtAndEquities/Debt/LoanAgreement`       | Optional FIBO mapping; useful for financial instruments.  |
| `relationshipTypeUri` `#subsidiary`   | Schema.org   | `subOrganization`      | relatedMatch | `https://schema.org/subOrganization`   | See vocab.mappings.       |
| `relationshipTypeUri` `#joint-venture`       | Schema.org   | `Organization` with `memberOf`     | relatedMatch | `https://schema.org/Organization`      | Joint ventures as special multi-party orgs.    |
| `relationshipTypeUri` `#regulatory-registration`  | Schema.org   | `GovernmentOrganization` / `Service`   | relatedMatch | `https://schema.org/GovernmentOrganization`       | Registration with a regulator.      |
| `relationshipTypeUri` `#fiscal-sponsorship`  | HSDS / Nonprofit  | HSDS `Funding` or `Service` | relatedMatch | `https://github.com/openreferral/specification`   | For nonprofit fiscal sponsor relationships.    |
| **Exchange (value transfer events)**  | OCDS   | `Implementation.transactions[]`    | exactMatch   | `https://standard.open-contracting.org/latest/en/schema/reference/#implementation`       | CEP Exchange is very close to OCDS transaction-level data.       |
| `exchangeTypeUri` `#grant-disbursement`      | Schema.org   | `MonetaryGrant`   | exactMatch   | `https://schema.org/MonetaryGrant`     | Core pattern for education & campaign finance. |
| `exchangeTypeUri` `#contract-fee-payment`    | Schema.org   | `Payment` / `Invoice`  | relatedMatch | `https://schema.org/Invoice`    | Payment for contracted services.    |
| `exchangeTypeUri` `#donation` (if defined)   | Schema.org   | `DonateAction`    | relatedMatch | `https://schema.org/DonateAction`      | Campaign or charitable donations.   |
| `exchangeTypeUri` `#loan-disbursement`       | FIBO   | `LoanPrincipalPayment`      | relatedMatch | FIBO debt concepts   | More detailed financial modeling if needed.    |
| **Value & Categorization** | XBRL   | GAAP / GRT financial elements      | exactMatch   | e.g. `http://xbrl.us/us-gaap/2024-01-31#RevenueRecognition` | Map CEP `categorization` fields to XBRL when possible.    |
| `value.currencyCode`   | ISO 4217     | Currency codes    | exactMatch   | `https://www.iso.org/iso-4217-currency-codes.html`     | Already enforced by regex.     |
| `categorization.naicsCode` | NAICS  | NAICS activity code    | exactMatch   | `https://www.census.gov/naics/` | Already included in schema.    |
| `categorization.cfdaNumber`       | Assistance List   | CFDA / Assistance Listing number   | exactMatch   | `https://sam.gov/content/assistance-listing`      | For federal assistance programs.    |
| `categorization.gtasAccountCode`  | US GTAS      | Treasury Account Symbol (TAS)      | exactMatch   | `https://fiscal.treasury.gov/gtas/`    | Connects to federal reporting.      |
| **Provenance & Attestation**      | W3C PROV     | `Entity`, `Activity`, `Agent`, `wasGeneratedBy`, `wasAttributedTo`, etc. | relatedMatch | `https://www.w3.org/TR/prov-o/` | CEP is PROV-aligned but more specialized.  |
| `attestation.attestorId`   | W3C PROV     | `Agent` / `prov:wasAttributedTo`   | exactMatch   | `https://www.w3.org/TR/prov-o/#wasAttributedTo`   | The attesting node is the Agent.    |
| `attestation.attestationTimestamp`    | W3C PROV     | `generatedAtTime`      | exactMatch   | `https://www.w3.org/TR/prov-o/#generatedAtTime`   | When the record (Entity) was generated.    |
| `previousRecordHash`   | W3C PROV     | `wasRevisionOf`   | exactMatch   | `https://www.w3.org/TR/prov-o/#wasRevisionOf`     | Revision chain between CEP records. |
| `provenanceChain.fundingChainTag` | W3C PROV     | `wasDerivedFrom` chain      | relatedMatch | `https://www.w3.org/TR/prov-o/#wasDerivedFrom`    | Human-readable representation of a PROV derivation path.  |
| `provenanceChain.parentExchangeId`    | W3C PROV     | `wasDerivedFrom`  | exactMatch   | `https://www.w3.org/TR/prov-o/#wasDerivedFrom`    | Parent exchange is the immediate predecessor in the flow. |
| **Events, Bills, Votes**   | OCD / Popolo      | `Bill`, `VoteEvent`, `Event`       | relatedMatch | `https://opencivicdata.org/specs/`     | Linked via `sourceReferences`.      |
| `sourceReferences` (bill)  | OCD    | `ocd-bill/*`      | exactMatch   | `https://opencivicdata.org/id/bill`    | For authorizing legislation.   |
| `sourceReferences` (vote)  | OCD / Popolo      | `ocd-vote/*` or Popolo `VoteEvent` | exactMatch   | `https://opencivicdata.org/id/vote`    | For authorization votes.  |
| `sourceReferences` (event) | OCD / Popolo      | `Event`     | exactMatch   | `https://opencivicdata.org/id/event`   | Meetings, hearings tied to relationships or exchanges.    |
| **Human Services / Programs (optional)**     | HSDS   | `Service`, `Organization`, `Funding`   | relatedMatch | `https://github.com/openreferral/specification`   | For social services and community programs.    |
| `entityTypeUri` `#service-provider`   | HSDS   | `Organization`    | exactMatch   | `https://github.com/openreferral/specification`   | For agencies and nonprofits providing services.       |
| `relationshipTypeUri` `#service-delivery-agreement` (future) | HSDS   | `Service` / `Funding`  | relatedMatch | HSDS docs | If added.      |

## Vocabulary Notes

prime-contract

-   USAspending contract is a close conceptual match.
-   OCDS contract is the canonical public procurement concept, so exactMatch.

subcontract

-   OCDS does not have a first-class subcontract object, but it is conceptually a specialized contract under a main award, so narrowMatch.

grant-award

-   USAspending grant is a direct match to federal grants.
-   OCDS award is broader (covers procurements and grants), so relatedMatch.
-   Schema.org MonetaryGrant is almost exactly, so exactMatch.

loan-agreement

-   FIBO Contracts and Loans ontologies give similar semantic neighborhood but more general, so relatedMatch.

subsidiary

-   Schema.org subOrganization is close but not strictly legal-definition match, so relatedMatch.
-   FIBO Subsidiary is semantically close but in financial-industry framing; so relatedMatch.

board-membership and employment

-   Both are specializations of Popolo Membership (person–organization relationship with roles and time-bounds), so narrowMatch.
