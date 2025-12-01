"""Exchange builder: raw data -> canonical CEP Exchange Record."""

from dataclasses import dataclass
from typing import Any

from civic_exchange_protocol.core import (
    Attestation,
    CanonicalTimestamp,
)
from civic_exchange_protocol.exchange import (
    ExchangeCategorization,
    ExchangeParty,
    ExchangeRecord,
    ExchangeStatus,
    ExchangeStatusCode,
    ExchangeValue,
    SourceReference,
)


@dataclass
class ExchangeBuildResult:
    """Result of building an exchange from raw data."""

    exchange: ExchangeRecord
    warnings: list[str]


# Map raw field names to canonical field names
FIELD_MAP = {
    "exchangeId": "exchange_id",
    "exchangeType": "exchange_type",
    "grantorEntityId": "source_entity_id",
    "granteeEntityId": "recipient_entity_id",
    "sourceEntityId": "source_entity_id",
    "recipientEntityId": "recipient_entity_id",
    "grantAmount": "amount",
    "amount": "amount",
    "currency": "currency",
    "awardDate": "occurred_date",
    "occurredDate": "occurred_date",
    "description": "description",
    "attestation": "attestation",
    "sourceSystem": "source_system",
    "sourceRecordId": "source_record_id",
    "sourceUrl": "source_url",
    # Categorization fields
    "cfdaNumber": "cfda_number",
    "naicsCode": "naics_code",
    "gtasAccountCode": "gtas_account_code",
    "localCategoryCode": "local_category_code",
    "localCategoryLabel": "local_category_label",
    "programCode": "local_category_code",
}

# Required fields for exchange building
EXCHANGE_REQUIRED = frozenset(
    {
        "exchange_id",
        "exchange_type",
        "source_entity_id",
        "recipient_entity_id",
        "amount",
        "currency",
        "occurred_date",
        "attestation",
    }
)

# Exchange type to URI mapping
EXCHANGE_TYPE_URI_MAP = {
    "GRANT": "https://civic-exchange.org/types/grant",
    "CONTRACT": "https://civic-exchange.org/types/contract",
    "PAYMENT": "https://civic-exchange.org/types/payment",
    "DONATION": "https://civic-exchange.org/types/donation",
    "FEE": "https://civic-exchange.org/types/fee",
    "TAX": "https://civic-exchange.org/types/tax",
    "TRANSFER": "https://civic-exchange.org/types/transfer",
}


def map_fields(raw: dict[str, Any]) -> tuple[dict[str, Any], list[str]]:
    """Map raw field names to canonical names.

    Args:
        raw: Dictionary with raw exchange fields.

    Returns:
        Tuple of (mapped fields dict, list of warnings).
    """
    mapped: dict[str, Any] = {}
    warnings: list[str] = []

    for raw_key, value in raw.items():
        if raw_key in FIELD_MAP:
            canonical_key = FIELD_MAP[raw_key]
            mapped[canonical_key] = value
        else:
            warnings.append(f"Unknown field ignored: {raw_key!r}")

    return mapped, warnings


def validate_required(mapped: dict[str, Any]) -> None:
    """Validate that required fields are present and non-null.

    Args:
        mapped: Dictionary with canonical field names.

    Raises:
        ValueError: If required fields are missing or null.
    """
    missing = EXCHANGE_REQUIRED - set(mapped.keys())
    if missing:
        raise ValueError(f"Missing required fields: {missing}")

    for field in EXCHANGE_REQUIRED:
        if mapped.get(field) is None:
            raise ValueError(f"Required field {field!r} cannot be null")


def parse_exchange_type_uri(exchange_type: str) -> str:
    """Convert exchange type to URI.

    Args:
        exchange_type: Raw exchange type (e.g., "GRANT").

    Returns:
        Exchange type URI.
    """
    upper = exchange_type.upper()
    if upper in EXCHANGE_TYPE_URI_MAP:
        return EXCHANGE_TYPE_URI_MAP[upper]
    return f"https://civic-exchange.org/types/{exchange_type.lower()}"


def parse_timestamp(date_str: str) -> CanonicalTimestamp:
    """Parse a date or datetime string to CanonicalTimestamp.

    Args:
        date_str: Date string (e.g., "2024-05-15" or "2024-05-15T14:02:10Z").

    Returns:
        CanonicalTimestamp.
    """
    # If it's just a date, add midnight UTC
    if "T" not in date_str:
        date_str = f"{date_str}T00:00:00.000000Z"
    return CanonicalTimestamp.parse(date_str)


def build_attestation(raw_attestation: dict[str, Any]) -> Attestation:
    """Build an Attestation from raw attestation data."""
    attested_by = raw_attestation.get("attestedBy", "unknown")
    timestamp_str = raw_attestation.get("attestationTimestamp", "")
    timestamp = CanonicalTimestamp.parse(timestamp_str)

    return Attestation.new(
        attestor_id=attested_by,
        attestation_timestamp=timestamp,
        proof_type="ManualAttestation",
        proof_value="",  # No cryptographic proof for manual attestations
        verification_method_uri=f"urn:cep:attestor:{attested_by.replace(' ', '-').lower()}",
    )


def build_exchange_party(entity_id: str) -> ExchangeParty:
    """Build an ExchangeParty from an entity ID.

    Args:
        entity_id: Entity identifier string.

    Returns:
        ExchangeParty object.
    """
    return ExchangeParty(entity_id=entity_id)


def build_exchange_value(amount: float, currency: str) -> ExchangeValue:
    """Build an ExchangeValue from amount and currency."""
    return ExchangeValue.monetary(amount, currency)


def build_categorization(mapped: dict[str, Any]) -> ExchangeCategorization | None:
    """Build categorization from mapped fields."""
    cfda = mapped.get("cfda_number")
    naics = mapped.get("naics_code")
    gtas = mapped.get("gtas_account_code")
    local_code = mapped.get("local_category_code")
    local_label = mapped.get("local_category_label") or mapped.get("description")

    if not any([cfda, naics, gtas, local_code]):
        return None

    return ExchangeCategorization(
        cfda_number=cfda,
        naics_code=naics,
        gtas_account_code=gtas,
        local_category_code=local_code,
        local_category_label=local_label,
    )


def build_source_reference(mapped: dict[str, Any]) -> SourceReference | None:
    """Build a source reference if data is available."""
    source_system = mapped.get("source_system")
    source_record_id = mapped.get("source_record_id")
    source_url = mapped.get("source_url")

    if not source_system or not source_record_id:
        return None

    return SourceReference(
        source_system_uri=source_system,
        source_record_id=source_record_id,
        source_url=source_url,
    )


def build_exchange_record(
    mapped: dict[str, Any],
    attestation: Attestation,
    occurred_timestamp: CanonicalTimestamp,
) -> ExchangeRecord:
    """Build the canonical ExchangeRecord."""
    exchange_type_uri = parse_exchange_type_uri(mapped["exchange_type"])
    source_party = build_exchange_party(mapped["source_entity_id"])
    recipient_party = build_exchange_party(mapped["recipient_entity_id"])
    value = build_exchange_value(mapped["amount"], mapped["currency"])

    status = ExchangeStatus(
        status_code=ExchangeStatusCode.COMPLETED,
        status_effective_timestamp=occurred_timestamp,
    )

    relationship_id = f"rel:{mapped['source_entity_id']}:{mapped['recipient_entity_id']}"

    record = ExchangeRecord.new(
        verifiable_id=mapped["exchange_id"],
        relationship_id=relationship_id,
        exchange_type_uri=exchange_type_uri,
        source_entity=source_party,
        recipient_entity=recipient_party,
        value=value,
        occurred_timestamp=occurred_timestamp,
        status=status,
        attestation=attestation,
    )

    categorization = build_categorization(mapped)
    if categorization:
        record = record.with_categorization(categorization)

    source_ref = build_source_reference(mapped)
    if source_ref:
        record = record.with_source_reference(source_ref)

    return record


def build_exchange(raw: dict[str, Any]) -> ExchangeBuildResult:
    """Build a canonical CEP Exchange Record from raw input data."""
    mapped, warnings = map_fields(raw)
    validate_required(mapped)

    attestation = build_attestation(mapped["attestation"])
    occurred_timestamp = parse_timestamp(mapped["occurred_date"])
    exchange = build_exchange_record(mapped, attestation, occurred_timestamp)

    return ExchangeBuildResult(
        exchange=exchange,
        warnings=warnings,
    )
