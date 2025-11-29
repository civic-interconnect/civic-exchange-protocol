CEP Canonical Serialization and Data Model Specification v1.0

This document defines the Canonical Serialization Rules required to achieve Hash Parity across all node implementations. Compliance with these rules is the minimum requirement for certification.

## 1. The Canonical String (C-String)

The Canonical String is the byte-for-byte precise, unambiguous sequence of characters derived from the TransactionRecord JSON payload. This C-String is the input for the final SHA-256 validation hash.

## Rule 1.1: Strict Field Ordering (The Single Source of Truth)

All fields that are present in the source data MUST be serialized in the following, non-negotiable order. This list is the CANONICAL_FIELD_ORDER.

1. transactionId
2. sourceUei
3. recipientUei
4. nodeRole
5. transactionAmount
6. transactionDate
7. transactionStatus
8. localFundCategory
9. dataRevisionNumber
10. dataTimestamp
11. dataCustodianUei
12. relatedUei
13. cfdaNumber
14. stateLegislativeDistrict
15. regionalCode

### Rule 1.2: Field Omission (Null/Empty Exclusion)

Optional fields (relatedUei, cfdaNumber, stateLegislativeDistrict, regionalCode) MUST BE OMITTED from the C-String if their value is null, undefined, or an empty string ("").

### Rule 1.3: Field Delimitation

Each field in the C-String MUST be separated by a single, non-escaped pipe character (|).

## 2. Data Type Precision and Formatting Rules

The following rules override any language-specific default formatting to ensure absolute consistency.

### Rule 2.1: Date and Time (Microsecond Precision)

All date/time fields (transactionDate, dataTimestamp) MUST be formatted using ISO 8601 UTC with mandatory six fractional seconds (microseconds) and terminated by the literal 'Z' (Zulu time).

| Data Type | Example (Correct) | Example (Incorrect - Will Fail Hash Parity) |
|---|---|---|
| DateTime | 2025-11-27T17:52:30.123456Z | 2025-11-27T17:52:30Z (Missing microseconds) |
| DateTime | 2025-11-27T17:52:30.000000Z | 2025-11-27 17:52:30 (Missing 'T' and 'Z') |

### Rule 2.2: Monetary Value (Fixed Decimal Precision)

The transactionAmount field MUST be serialized to exactly two decimal places, regardless of whether the source data contains cents, using a period (.) as the separator. Scientific notation is strictly forbidden.

| Data Type | Source Value | C-String Output |
|---|---|---|
| Monetary | 125000.55 | 125000.55 |
| Monetary | 125000 | 125000.00 (Mandatory two decimals) |
| Monetary | 125000.556 | INVALID SOURCE (Must be rounded or truncated at source) |

### Rule 2.3: Numeric Types (Integer)

The dataRevisionNumber field MUST be serialized as a standard, non-decimal integer with no leading zeros.

## 3. Final Hash Generation

The complete C-String is passed directly to the SHA-256 algorithm.

- Algorithm: SHA-256 (Secure Hash Algorithm 256-bit)
- Input: The final pipe-delimited C-String.
- Output: The 64-character hexadecimal representation of the hash.

This 64-character hash is the Validation Hash used for Node Certification.

Example Canonical String Construction

Given a payload where cfdaNumber is present but relatedUei and regionalCode are null:
```
transactionId|sourceUei|recipientUei|...|dataCustodianUei|cfdaNumber|stateLegislativeDistrict
TX-1234|1A2B3C4D5E6F|7G8H9I0J1K2L|...|1A2B3C4D5E6F|84.002|LD-22
```
The resulting C-String would look like this (abbreviated):

```
TX-1234|1A2B3C4D5E6F|7G8H9I0J1K2L|SOURCE|125000.00|2025-11-27T17:52:30.123456Z|...|84.002|LD-22
```