# Python Implementation


## Monetary Data Type

We use decimal.Decimal for financial values to prevent floating-point inaccuracies, and rely on f-string formatting (f"{value:.2f}") to strictly enforce the two-decimal-place rule.

## Date/Time Control

The datetime objects are explicitly made timezone-aware (tzinfo=timezone.utc) and the format string ISO8601_MICROS_FORMAT is custom-defined to guarantee the necessary microsecond precision and Z suffix.

## Serialization Logic

The core serialization logic is separated into serialize_field and generate_canonical_string, ensuring that the CANONICAL_FIELD_ORDER list is the singular authority on the structure, overriding Python's natural dictionary ordering.

## Compare to Rust (Official)

| Module | Rust | Python |
|-----|-----|-----|
| timestamp | CanonicalTimestamp with %.6fZ format | CanonicalTimestamp with %f + Z |
| hash | CanonicalHash using sha2 | CanonicalHash using hashlib.sha256 |
| canonical | Canonicalize trait + BTreeMap | Canonicalize ABC + sorted() dict |
| attestation | Attestation struct | @dataclass Attestation |
| format_amount | format!("{:.2}", amount) | Decimal + f"{:.2f}" |