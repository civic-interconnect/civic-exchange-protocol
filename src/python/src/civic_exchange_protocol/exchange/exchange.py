"""CEP Exchange Record definition.

An Exchange Record represents a verifiable value exchange (financial, in-kind,
or informational) between entities within an established relationship.
This is the atomic unit of civic transparency.
"""

from dataclasses import dataclass, field
from enum import Enum

from civic_exchange_protocol.core import (
    SCHEMA_VERSION,
    Attestation,
    CanonicalHash,
    Canonicalize,
    CanonicalTimestamp,
    insert_if_present,
    insert_required,
)

from .provenance import ExchangeCategorization, ProvenanceChain
from .value import ExchangeParty, ExchangeValue


class ExchangeStatusCode(Enum):
    """Exchange operational status."""

    PENDING = "PENDING"
    COMPLETED = "COMPLETED"
    REVERSED = "REVERSED"
    CANCELED = "CANCELED"
    DISPUTED = "DISPUTED"

    def as_str(self) -> str:
        """Return the string value of the exchange status code."""
        return self.value


@dataclass
class ExchangeStatus(Canonicalize):
    """Exchange status information."""

    status_code: ExchangeStatusCode
    status_effective_timestamp: CanonicalTimestamp

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical field representation of the exchange status.

        Returns:
        -------
        dict[str, str]
            Dictionary containing the canonical fields.
        """
        fields: dict[str, str] = {}
        insert_required(fields, "statusCode", self.status_code.as_str())
        insert_required(
            fields,
            "statusEffectiveTimestamp",
            self.status_effective_timestamp.to_canonical_string(),
        )
        return fields


@dataclass
class SourceReference(Canonicalize):
    """Reference to an authoritative source record."""

    source_system_uri: str
    source_record_id: str
    source_url: str | None = None

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical field representation of the source reference.

        Returns:
        -------
        dict[str, str]
            Dictionary containing the canonical fields.
        """
        fields: dict[str, str] = {}
        insert_required(fields, "sourceRecordId", self.source_record_id)
        insert_required(fields, "sourceSystemUri", self.source_system_uri)
        insert_if_present(fields, "sourceUrl", self.source_url)
        return fields


@dataclass
class ExchangeRecord(Canonicalize):
    """A complete CEP Exchange Record."""

    # Required fields
    verifiable_id: str
    relationship_id: str
    exchange_type_uri: str
    source_entity: ExchangeParty
    recipient_entity: ExchangeParty
    value: ExchangeValue
    occurred_timestamp: CanonicalTimestamp
    status: ExchangeStatus
    attestation: Attestation

    # Optional fields
    schema_version: str = field(default=SCHEMA_VERSION)
    provenance_chain: ProvenanceChain | None = None
    categorization: ExchangeCategorization | None = None
    source_references: list[SourceReference] | None = None
    previous_record_hash: CanonicalHash | None = None
    revision_number: int = 1

    @classmethod
    def new(
        cls,
        verifiable_id: str,
        relationship_id: str,
        exchange_type_uri: str,
        source_entity: ExchangeParty,
        recipient_entity: ExchangeParty,
        value: ExchangeValue,
        occurred_timestamp: CanonicalTimestamp,
        status: ExchangeStatus,
        attestation: Attestation,
    ) -> "ExchangeRecord":
        """Create a new ExchangeRecord with required fields."""
        return cls(
            verifiable_id=verifiable_id,
            relationship_id=relationship_id,
            exchange_type_uri=exchange_type_uri,
            source_entity=source_entity,
            recipient_entity=recipient_entity,
            value=value,
            occurred_timestamp=occurred_timestamp,
            status=status,
            attestation=attestation,
        )

    def with_provenance(self, chain: ProvenanceChain) -> "ExchangeRecord":
        """Return a new ExchangeRecord with provenance chain set."""
        return ExchangeRecord(
            verifiable_id=self.verifiable_id,
            relationship_id=self.relationship_id,
            exchange_type_uri=self.exchange_type_uri,
            source_entity=self.source_entity,
            recipient_entity=self.recipient_entity,
            value=self.value,
            occurred_timestamp=self.occurred_timestamp,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            provenance_chain=chain,
            categorization=self.categorization,
            source_references=self.source_references,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_categorization(self, cat: ExchangeCategorization) -> "ExchangeRecord":
        """Return a new ExchangeRecord with categorization set."""
        return ExchangeRecord(
            verifiable_id=self.verifiable_id,
            relationship_id=self.relationship_id,
            exchange_type_uri=self.exchange_type_uri,
            source_entity=self.source_entity,
            recipient_entity=self.recipient_entity,
            value=self.value,
            occurred_timestamp=self.occurred_timestamp,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            provenance_chain=self.provenance_chain,
            categorization=cat,
            source_references=self.source_references,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_source_reference(self, reference: SourceReference) -> "ExchangeRecord":
        """Return a new ExchangeRecord with a source reference added."""
        refs = list(self.source_references) if self.source_references else []
        refs.append(reference)
        return ExchangeRecord(
            verifiable_id=self.verifiable_id,
            relationship_id=self.relationship_id,
            exchange_type_uri=self.exchange_type_uri,
            source_entity=self.source_entity,
            recipient_entity=self.recipient_entity,
            value=self.value,
            occurred_timestamp=self.occurred_timestamp,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            provenance_chain=self.provenance_chain,
            categorization=self.categorization,
            source_references=refs,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_previous_hash(self, hash_val: CanonicalHash) -> "ExchangeRecord":
        """Return a new ExchangeRecord with previous hash set."""
        return ExchangeRecord(
            verifiable_id=self.verifiable_id,
            relationship_id=self.relationship_id,
            exchange_type_uri=self.exchange_type_uri,
            source_entity=self.source_entity,
            recipient_entity=self.recipient_entity,
            value=self.value,
            occurred_timestamp=self.occurred_timestamp,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            provenance_chain=self.provenance_chain,
            categorization=self.categorization,
            source_references=self.source_references,
            previous_record_hash=hash_val,
            revision_number=self.revision_number,
        )

    def with_revision(self, revision: int) -> "ExchangeRecord":
        """Return a new ExchangeRecord with revision number set."""
        return ExchangeRecord(
            verifiable_id=self.verifiable_id,
            relationship_id=self.relationship_id,
            exchange_type_uri=self.exchange_type_uri,
            source_entity=self.source_entity,
            recipient_entity=self.recipient_entity,
            value=self.value,
            occurred_timestamp=self.occurred_timestamp,
            status=self.status,
            attestation=self.attestation,
            schema_version=self.schema_version,
            provenance_chain=self.provenance_chain,
            categorization=self.categorization,
            source_references=self.source_references,
            previous_record_hash=self.previous_record_hash,
            revision_number=revision,
        )

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields in alphabetical order."""
        fields: dict[str, str] = {}

        # All fields in alphabetical order
        insert_required(fields, "attestation", self.attestation.to_canonical_string())

        if self.categorization is not None and self.categorization.has_any():
            insert_required(fields, "categorization", self.categorization.to_canonical_string())

        insert_required(fields, "exchangeTypeUri", self.exchange_type_uri)
        insert_required(fields, "occurredTimestamp", self.occurred_timestamp.to_canonical_string())

        if self.previous_record_hash is not None:
            insert_required(fields, "previousRecordHash", self.previous_record_hash.as_hex())

        if self.provenance_chain is not None and self.provenance_chain.has_any():
            insert_required(fields, "provenanceChain", self.provenance_chain.to_canonical_string())

        insert_required(fields, "recipientEntity", self.recipient_entity.to_canonical_string())
        insert_required(fields, "relationshipId", self.relationship_id)
        insert_required(fields, "revisionNumber", str(self.revision_number))
        insert_required(fields, "schemaVersion", self.schema_version)

        # Source references sorted by sourceSystemUri then sourceRecordId
        if self.source_references:
            sorted_refs = sorted(
                self.source_references,
                key=lambda r: (r.source_system_uri, r.source_record_id),
            )
            refs_json = ",".join(r.to_canonical_string() for r in sorted_refs)
            fields["sourceReferences"] = f"[{refs_json}]"

        insert_required(fields, "sourceEntity", self.source_entity.to_canonical_string())
        insert_required(fields, "status", self.status.to_canonical_string())
        insert_required(fields, "value", self.value.to_canonical_string())
        insert_required(fields, "verifiableId", self.verifiable_id)

        return fields
