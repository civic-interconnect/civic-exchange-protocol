"""Builder for creating civic exchange entity records.

This module provides a builder function for constructing
CEP-compliant Entity JSON objects from minimal raw input.
"""

from collections.abc import Mapping
from typing import Any

from civic_exchange_protocol.core import SCHEMA_VERSION
from civic_exchange_protocol.snfei import generate_snfei_with_confidence

ENTITY_TYPE_URI_MAP: dict[str, str] = {
    "MUNICIPALITY": (
        "https://raw.githubusercontent.com/"
        "civic-interconnect/civic-exchange-protocol/main/"
        "vocabulary/entity-type.json#local-government"
    ),
    "SCHOOL_DISTRICT": (
        "https://raw.githubusercontent.com/"
        "civic-interconnect/civic-exchange-protocol/main/"
        "vocabulary/entity-type.json#educational-institution"
    ),
    "NONPROFIT_501C3": (
        "https://raw.githubusercontent.com/"
        "civic-interconnect/civic-exchange-protocol/main/"
        "vocabulary/entity-type.json#nonprofit-501c3"
    ),
}


def _lookup_entity_type_uri(entity_type: str | None) -> str | None:
    if not entity_type:
        return None
    return ENTITY_TYPE_URI_MAP.get(entity_type)


def build_entity_from_raw(
    raw: Mapping[str, Any],
    *,
    attestor_id: str = "cep-entity:demo:attestor-1",
    attestation_timestamp: str = "2025-11-28T15:00:00.000000Z",
    status_effective_date: str = "2024-01-01",
) -> dict[str, Any]:
    """Build a CEP-compliant Entity JSON object from a raw entity record.

    raw is your minimal input, e.g.:

    {
        "entityId": "US-IL-MUNI-0012",
        "legalName": "City of Springfield",
        "entityType": "MUNICIPALITY",
        "jurisdiction": "US-IL",
        "countryCode": "US",
        "address": "200 Main Street",
        "registrationDate": null
    }
    """
    legal_name = raw["legalName"]
    country_code = raw["countryCode"]
    address = raw.get("address")
    registration_date = raw.get("registrationDate")

    # If you have LEI / SAM UEI in raw, pass them through here.
    lei = raw.get("lei")
    sam_uei = raw.get("samUei")

    snfei_result = generate_snfei_with_confidence(
        legal_name=legal_name,
        country_code=country_code,
        address=address,
        registration_date=registration_date,
        lei=lei,
        sam_uei=sam_uei,
    )

    snfei_value = snfei_result.snfei.value
    canonical = snfei_result.canonical

    legal_name_normalized = canonical.legal_name_normalized
    entity_type = raw.get("entityType")
    entity_type_uri = _lookup_entity_type_uri(entity_type)

    entity: dict[str, Any] = {
        "schemaVersion": SCHEMA_VERSION,
        "verifiableId": f"cep-entity:snfei:{snfei_value}",
        "identifiers": {
            "snfei": snfei_value,
            "additionalSchemes": [],
        },
        "legalName": legal_name,
        "legalNameNormalized": legal_name_normalized,
        "entityTypeUri": entity_type_uri,
        "jurisdictionIso": raw["jurisdiction"],
        "status": {
            "statusCode": "ACTIVE",
            "statusEffectiveDate": status_effective_date,
            "statusTerminationDate": None,
            "successorEntityId": None,
        },
        "naicsCode": None,
        "resolutionConfidence": {
            "score": snfei_result.confidence_score,
            "methodUri": (
                "https://raw.githubusercontent.com/"
                "civic-interconnect/civic-exchange-protocol/main/"
                "vocabulary/resolution-method.json#snfei-v1"
            ),
            "sourceRecordCount": 1,
        },
        "attestation": {
            "attestorId": attestor_id,
            "attestationTimestamp": attestation_timestamp,
            "proofType": "Ed25519Signature2020",
            "proofValue": "BASE64_SIGNATURE_EXAMPLE",
            "verificationMethodUri": "https://example.org/keys/attestor-1#primary",
            "proofPurpose": "assertionMethod",
            "anchorUri": None,
        },
        "previousRecordHash": None,
        "revisionNumber": 1,
    }

    # identifier extras
    additional_schemes = entity["identifiers"]["additionalSchemes"]
    if "entityId" in raw:
        additional_schemes.append(
            {
                "schemeUri": "https://specs.civic-interconnect.org/schemes/local-entity-id",
                "value": str(raw["entityId"]),
            }
        )
    if lei:
        additional_schemes.append(
            {
                "schemeUri": "https://www.gleif.org/en/lei-solutions",
                "value": lei,
            }
        )
    if sam_uei:
        additional_schemes.append(
            {
                "schemeUri": "https://sam.gov/content/entity-registration",
                "value": sam_uei,
            }
        )

    # Keep raw fields for pedagogy / debuggability
    entity.update(raw)

    return entity
