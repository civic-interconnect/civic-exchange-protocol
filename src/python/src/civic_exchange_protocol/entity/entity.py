"""CEP Entity Record definition.

The Entity Record is the foundational primitive in CEP. It represents a
verified civic entity (government agency, contractor, nonprofit, individual).
All relationships and exchanges reference attested entities.
"""

from dataclasses import dataclass, field
from enum import Enum

from civic_exchange_protocol.core import (
    SCHEMA_VERSION,
    Attestation,
    CanonicalHash,
    Canonicalize,
    insert_if_present,
    insert_required,
)

from .identifiers import EntityIdentifiers


class EntityStatusCode(Enum):
    """Entity operational status."""

    ACTIVE = "ACTIVE"
    INACTIVE = "INACTIVE"
    SUSPENDED = "SUSPENDED"
    DISSOLVED = "DISSOLVED"
    MERGED = "MERGED"

    def as_str(self) -> str:
        """Return the string representation of the status code.

        Returns:
        -------
        str
            The status code value as a string.
        """
        return self.value


@dataclass
class EntityStatus(Canonicalize):
    """Entity status information."""

    status_code: EntityStatusCode
    status_effective_date: str  # YYYY-MM-DD format
    status_termination_date: str | None = None
    successor_entity_id: str | None = None

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields for entity status.

        Returns:
        dict[str, str]
            A dictionary containing the canonical representation of status fields.
        """
        fields: dict[str, str] = {}
        insert_required(fields, "statusCode", self.status_code.as_str())
        insert_required(fields, "statusEffectiveDate", self.status_effective_date)
        insert_if_present(fields, "statusTerminationDate", self.status_termination_date)
        insert_if_present(fields, "successorEntityId", self.successor_entity_id)
        return fields


@dataclass
class ResolutionConfidence(Canonicalize):
    """Entity resolution confidence metadata."""

    score: float  # 0.0 to 1.0
    method_uri: str | None = None
    source_record_count: int | None = None

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields for resolution confidence.

        Returns:
        -------
        dict[str, str]
            A dictionary containing the canonical representation of resolution confidence fields.
        """
        fields: dict[str, str] = {}
        insert_if_present(fields, "methodUri", self.method_uri)
        # Score formatted to 2 decimal places
        insert_required(fields, "score", f"{self.score:.2f}")
        if self.source_record_count is not None:
            insert_required(fields, "sourceRecordCount", str(self.source_record_count))
        return fields


@dataclass
class EntityRecord(Canonicalize):
    """A complete CEP Entity Record."""

    # Required fields
    verifiable_id: str
    identifiers: EntityIdentifiers
    legal_name: str
    jurisdiction_iso: str
    status: EntityStatus
    attestation: Attestation

    # Optional fields
    schema_version: str = field(default=SCHEMA_VERSION)
    legal_name_normalized: str | None = None
    entity_type_uri: str | None = None
    naics_code: str | None = None
    resolution_confidence: ResolutionConfidence | None = None
    previous_record_hash: CanonicalHash | None = None
    revision_number: int = 1

    @classmethod
    def new(
        cls,
        verifiable_id: str,
        identifiers: EntityIdentifiers,
        legal_name: str,
        jurisdiction_iso: str,
        status: EntityStatus,
        attestation: Attestation,
    ) -> "EntityRecord":
        """Create a new EntityRecord with required fields."""
        return cls(
            verifiable_id=verifiable_id,
            identifiers=identifiers,
            legal_name=legal_name,
            jurisdiction_iso=jurisdiction_iso,
            status=status,
            attestation=attestation,
        )

    def with_normalized_name(self, name: str) -> "EntityRecord":
        """Return a new EntityRecord with the normalized name set."""
        return EntityRecord(
            verifiable_id=self.verifiable_id,
            identifiers=self.identifiers,
            legal_name=self.legal_name,
            jurisdiction_iso=self.jurisdiction_iso,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            legal_name_normalized=name,
            entity_type_uri=self.entity_type_uri,
            naics_code=self.naics_code,
            resolution_confidence=self.resolution_confidence,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_entity_type(self, uri: str) -> "EntityRecord":
        """Return a new EntityRecord with the entity type URI set."""
        return EntityRecord(
            verifiable_id=self.verifiable_id,
            identifiers=self.identifiers,
            legal_name=self.legal_name,
            jurisdiction_iso=self.jurisdiction_iso,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            legal_name_normalized=self.legal_name_normalized,
            entity_type_uri=uri,
            naics_code=self.naics_code,
            resolution_confidence=self.resolution_confidence,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_naics(self, code: str) -> "EntityRecord":
        """Return a new EntityRecord with the NAICS code set."""
        return EntityRecord(
            verifiable_id=self.verifiable_id,
            identifiers=self.identifiers,
            legal_name=self.legal_name,
            jurisdiction_iso=self.jurisdiction_iso,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            legal_name_normalized=self.legal_name_normalized,
            entity_type_uri=self.entity_type_uri,
            naics_code=code,
            resolution_confidence=self.resolution_confidence,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_resolution_confidence(self, confidence: ResolutionConfidence) -> "EntityRecord":
        """Return a new EntityRecord with resolution confidence set."""
        return EntityRecord(
            verifiable_id=self.verifiable_id,
            identifiers=self.identifiers,
            legal_name=self.legal_name,
            jurisdiction_iso=self.jurisdiction_iso,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            legal_name_normalized=self.legal_name_normalized,
            entity_type_uri=self.entity_type_uri,
            naics_code=self.naics_code,
            resolution_confidence=confidence,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_previous_hash(self, hash_val: CanonicalHash) -> "EntityRecord":
        """Return a new EntityRecord with the previous hash set."""
        return EntityRecord(
            verifiable_id=self.verifiable_id,
            identifiers=self.identifiers,
            legal_name=self.legal_name,
            jurisdiction_iso=self.jurisdiction_iso,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            legal_name_normalized=self.legal_name_normalized,
            entity_type_uri=self.entity_type_uri,
            naics_code=self.naics_code,
            resolution_confidence=self.resolution_confidence,
            previous_record_hash=hash_val,
            revision_number=self.revision_number,
        )

    def with_revision(self, revision: int) -> "EntityRecord":
        """Return a new EntityRecord with the revision number set."""
        return EntityRecord(
            verifiable_id=self.verifiable_id,
            identifiers=self.identifiers,
            legal_name=self.legal_name,
            jurisdiction_iso=self.jurisdiction_iso,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            legal_name_normalized=self.legal_name_normalized,
            entity_type_uri=self.entity_type_uri,
            naics_code=self.naics_code,
            resolution_confidence=self.resolution_confidence,
            previous_record_hash=self.previous_record_hash,
            revision_number=revision,
        )

    def validate(self) -> None:
        """Validate that the record has all required fields properly set.

        Raises:
            ValueError: If validation fails.
        """
        if self.schema_version != SCHEMA_VERSION:
            raise ValueError(f"Unsupported schema version: {self.schema_version}")
        if not self.verifiable_id:
            raise ValueError("verifiableId is required")
        if not self.identifiers.has_any():
            raise ValueError("At least one identifier is required")
        if not self.legal_name:
            raise ValueError("legalName is required")
        if not self.jurisdiction_iso:
            raise ValueError("jurisdictionIso is required")
        if self.revision_number < 1:
            raise ValueError("revisionNumber must be >= 1")

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields in alphabetical order."""
        fields: dict[str, str] = {}

        # All fields in alphabetical order
        insert_required(fields, "attestation", self.attestation.to_canonical_string())
        insert_if_present(fields, "entityTypeUri", self.entity_type_uri)

        # Identifiers is a nested object
        identifiers_canonical = self.identifiers.to_canonical_string()
        if identifiers_canonical:
            insert_required(fields, "identifiers", identifiers_canonical)

        insert_required(fields, "jurisdictionIso", self.jurisdiction_iso)
        insert_required(fields, "legalName", self.legal_name)
        insert_if_present(fields, "legalNameNormalized", self.legal_name_normalized)
        insert_if_present(fields, "naicsCode", self.naics_code)

        if self.previous_record_hash is not None:
            insert_required(fields, "previousRecordHash", self.previous_record_hash.as_hex())

        # Resolution confidence is a nested object
        if self.resolution_confidence is not None:
            insert_required(
                fields, "resolutionConfidence", self.resolution_confidence.to_canonical_string()
            )

        insert_required(fields, "revisionNumber", str(self.revision_number))
        insert_required(fields, "schemaVersion", self.schema_version)

        # Status is a nested object
        insert_required(fields, "status", self.status.to_canonical_string())

        insert_required(fields, "verifiableId", self.verifiable_id)

        return fields
