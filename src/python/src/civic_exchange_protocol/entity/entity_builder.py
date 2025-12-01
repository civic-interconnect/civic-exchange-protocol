"""Entity builder: raw data -> canonical CEP Entity."""

from dataclasses import dataclass
from typing import Any

from civic_exchange_protocol.core import CanonicalTimestamp
from civic_exchange_protocol.snfei import (
    SnfeiResult,
    apply_localization,
    generate_snfei,
    normalize_address,
)


@dataclass
class EntityBuildResult:
    """Result of building an entity from raw data."""

    entity: dict[str, Any]
    snfei_result: SnfeiResult
    warnings: list[str]


# Map raw field names to canonical field names
FIELD_MAP = {
    "entityId": "source_id",
    "legalName": "legal_name",
    "entityType": "entity_type",
    "jurisdiction": "jurisdiction",
    "countryCode": "country_code",
    "address": "address",
    "registrationDate": "registration_date",
}

# Required fields for SNFEI generation
SNFEI_REQUIRED = {"legal_name", "country_code"}

# Entity type normalization
ENTITY_TYPE_MAP = {
    "MUNICIPALITY": "municipality",
    "COUNTY": "county",
    "STATE": "state",
    "FEDERAL": "federal",
    "SCHOOL_DISTRICT": "school_district",
    "SPECIAL_DISTRICT": "special_district",
}


def map_fields(raw: dict[str, Any]) -> tuple[dict[str, Any], list[str]]:
    """Map raw field names to canonical names.

    Args:
        raw: Dictionary with raw entity fields.

    Returns:
        Tuple of (mapped fields dict, list of warnings).
    """
    mapped = {}
    warnings = []

    for raw_key, canonical_key in FIELD_MAP.items():
        if raw_key in raw:
            mapped[canonical_key] = raw[raw_key]

    for key in raw:
        if key not in FIELD_MAP:
            warnings.append(f"Unknown field ignored: {key!r}")

    return mapped, warnings


def validate_required(mapped: dict[str, Any]) -> None:
    """Validate that required fields are present and non-null.

    Args:
        mapped: Dictionary with canonical field names.

    Raises:
        ValueError: If required fields are missing or null.
    """
    missing = SNFEI_REQUIRED - set(mapped.keys())
    if missing:
        raise ValueError(f"Missing required fields: {missing}")

    for field in SNFEI_REQUIRED:
        if mapped.get(field) is None:
            raise ValueError(f"Required field {field!r} cannot be null")


def localize_name(legal_name: str, jurisdiction: str | None, country_code: str) -> str:
    """Apply jurisdiction-specific localization to legal name.

    Args:
        legal_name: Raw legal name.
        jurisdiction: Jurisdiction code (e.g., "US-IL").
        country_code: Country code fallback (e.g., "US").

    Returns:
        Localized legal name.
    """
    return apply_localization(legal_name, jurisdiction or country_code)


def normalize_entity_type(raw_type: str | None) -> str:
    """Normalize entity type to CEP vocabulary.

    Args:
        raw_type: Raw entity type string.

    Returns:
        Normalized entity type.
    """
    if not raw_type:
        return "OTHER"
    return ENTITY_TYPE_MAP.get(raw_type.upper(), raw_type.lower())


def build_entity_dict(
    mapped: dict[str, Any],
    legal_name: str,
    snfei_value: str,
) -> dict[str, Any]:
    """Build the canonical entity dictionary.

    Args:
        mapped: Canonical field dict.
        legal_name: Localized legal name.
        snfei_value: Computed SNFEI hash string.

    Returns:
        CEP entity dictionary.
    """
    now = CanonicalTimestamp.now()

    entity: dict[str, Any] = {
        "@context": "https://civic-exchange.org/contexts/entity/v1",
        "@type": "CepEntity",
        "snfei": snfei_value,
        "legalName": legal_name,
        "entityType": normalize_entity_type(mapped.get("entity_type")),
        "jurisdiction": mapped.get("jurisdiction"),
        "countryCode": mapped["country_code"],
        "createdAt": str(now),
        "updatedAt": str(now),
    }

    if mapped.get("address"):
        entity["address"] = normalize_address(mapped["address"])

    if mapped.get("registration_date"):
        entity["registrationDate"] = mapped["registration_date"]

    if mapped.get("source_id"):
        entity["sourceIdentifiers"] = [{"system": "source", "value": mapped["source_id"]}]

    return entity


def build_entity(raw: dict[str, Any]) -> EntityBuildResult:
    """Build a canonical CEP entity from raw input data.

    Applies the normalizing functor pipeline:
        Raw -> Localized -> Normalized -> Canonical Entity + SNFEI

    Args:
        raw: Dictionary with raw entity fields.

    Returns:
        EntityBuildResult with entity, SNFEI result, and warnings.

    Raises:
        ValueError: If required fields are missing.
    """
    mapped, warnings = map_fields(raw)
    validate_required(mapped)

    legal_name = localize_name(
        mapped["legal_name"],
        mapped.get("jurisdiction"),
        mapped["country_code"],
    )

    snfei_result = generate_snfei(
        legal_name=legal_name,
        country_code=mapped["country_code"],
        address=mapped.get("address"),
        registration_date=mapped.get("registration_date"),
    )

    entity = build_entity_dict(mapped, legal_name, snfei_result.snfei.value)

    return EntityBuildResult(
        entity=entity,
        snfei_result=snfei_result,
        warnings=warnings,
    )
