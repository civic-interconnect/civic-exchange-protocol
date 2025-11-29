# Reference Standards

## Associated Standards

| Standard/Acronym   | Purpose    | Why We Interface    |
|--------------------|------------|---------------|
| XBRL (eXtensible Business Reporting Language)  | The international standard for electronic transmission of business and financial data (e.g., SEC and FDIC filings use it). | We map its transactional fields directly to the relevant XBRL taxonomy elements for regulatory compliance reporting. |
| LEI (Legal Entity Identifier) | Global standard for identifying parties to financial transactions worldwide. | While the UEI is authoritative in the U.S. government space, our protocol needs to contain a field for the LEI if the entity is globally registered, ensuring compliance for any international transactions. |
| W3C PROV (Provenance)   | The World Wide Web Consortium standard for recording the historical lifecycle and data quality of a piece of information.  | Our core value is Provenance. We adopt the principles of W3C PROV to formally define how data history, revisions, and sources are timestamped and logged.|
| GTAS (Government-wide Treasury Account Symbol) | The framework used by the Treasury for standardized federal financial reporting. | Transactional data (e.g., amounts, categories) must be translatable into GTAS fields for seamless reporting up to the Treasury level. |


## Bridge

- Official U.S UEI: The U.S. government already uses a Unique Entity Identifier (UEI), a 12-character alphanumeric ID assigned by SAM.gov, for all entities receiving federal financial assistance or doing business with the federal government (Source 3.1, 3.2). Often does not appear on state/local campaign finance reports or local school district consultant contracts unless federal funds are directly involved.
- Open-Source Data Cleaning Tools: Tools like Splink, OpenRefine, and Python/Pandas to manually clean and standardize messy names ("Acme Consulting, LLC" vs. "Acme Consulting") (Source 2.1, 2.3).

