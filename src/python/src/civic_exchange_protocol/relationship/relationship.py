"""CEP Relationship Record definition.

A Relationship Record represents a verifiable legal or functional relationship
between two or more attested entities.

Relationships can be:
- Bilateral: Two-party relationships with clear directionality (contracts, grants)
- Multilateral: N-ary relationships (consortia, boards, joint ventures)
"""

from dataclasses import dataclass, field
from enum import Enum

from civic_exchange_protocol.core import (
    SCHEMA_VERSION,
    Attestation,
    CanonicalHash,
    Canonicalize,
    CanonicalTimestamp,
    format_amount,
    insert_if_present,
    insert_required,
)

from .bilateral import BilateralParties
from .multilateral import MultilateralMembers


class RelationshipStatusCode(Enum):
    """Relationship operational status."""

    PENDING = "PENDING"
    ACTIVE = "ACTIVE"
    SUSPENDED = "SUSPENDED"
    COMPLETED = "COMPLETED"
    TERMINATED = "TERMINATED"
    AMENDED = "AMENDED"

    def as_str(self) -> str:
        """Return the string value of the relationship status code."""
        return self.value


@dataclass
class RelationshipStatus(Canonicalize):
    """Relationship status information."""

    status_code: RelationshipStatusCode
    status_effective_timestamp: CanonicalTimestamp

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical representation of relationship status fields.

        Returns:
        -------
        dict[str, str]
            Dictionary containing the canonical field representations.
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
class FinancialTerms(Canonicalize):
    """Financial terms of a relationship."""

    total_value: float | None = None
    obligated_value: float | None = None
    currency_code: str = "USD"

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical representation of financial terms fields.

        Returns:
        -------
        dict[str, str]
            Dictionary containing the canonical field representations.
        """
        fields: dict[str, str] = {}
        insert_required(fields, "currencyCode", self.currency_code)
        if self.obligated_value is not None:
            insert_required(fields, "obligatedValue", format_amount(self.obligated_value))
        if self.total_value is not None:
            insert_required(fields, "totalValue", format_amount(self.total_value))
        return fields


@dataclass
class SourceReference(Canonicalize):
    """Reference to an authoritative source record."""

    source_system_uri: str
    source_record_id: str
    source_url: str | None = None

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical representation of source reference fields.

        Returns:
        -------
        dict[str, str]
            Dictionary containing the canonical field representations.
        """
        fields: dict[str, str] = {}
        insert_required(fields, "sourceRecordId", self.source_record_id)
        insert_required(fields, "sourceSystemUri", self.source_system_uri)
        insert_if_present(fields, "sourceUrl", self.source_url)
        return fields


# Type alias for parties
# Type alias for parties
Parties = BilateralParties | MultilateralMembers


@dataclass
class RelationshipRecord(Canonicalize):
    """A complete CEP Relationship Record."""

    # Required fields
    verifiable_id: str
    relationship_type_uri: str
    parties: Parties
    effective_timestamp: CanonicalTimestamp
    status: RelationshipStatus
    jurisdiction_iso: str
    attestation: Attestation

    # Optional fields
    schema_version: str = field(default=SCHEMA_VERSION)
    parent_relationship_id: str | None = None
    expiration_timestamp: CanonicalTimestamp | None = None
    financial_terms: FinancialTerms | None = None
    terms_attributes: dict[str, str] | None = None
    source_references: list[SourceReference] | None = None
    previous_record_hash: CanonicalHash | None = None
    revision_number: int = 1

    @classmethod
    def new_bilateral(
        cls,
        verifiable_id: str,
        relationship_type_uri: str,
        parties: BilateralParties,
        effective_timestamp: CanonicalTimestamp,
        status: RelationshipStatus,
        jurisdiction_iso: str,
        attestation: Attestation,
    ) -> "RelationshipRecord":
        """Create a new bilateral RelationshipRecord."""
        return cls(
            verifiable_id=verifiable_id,
            relationship_type_uri=relationship_type_uri,
            parties=parties,
            effective_timestamp=effective_timestamp,
            status=status,
            jurisdiction_iso=jurisdiction_iso,
            attestation=attestation,
        )

    @classmethod
    def new_multilateral(
        cls,
        verifiable_id: str,
        relationship_type_uri: str,
        members: MultilateralMembers,
        effective_timestamp: CanonicalTimestamp,
        status: RelationshipStatus,
        jurisdiction_iso: str,
        attestation: Attestation,
    ) -> "RelationshipRecord":
        """Create a new multilateral RelationshipRecord."""
        return cls(
            verifiable_id=verifiable_id,
            relationship_type_uri=relationship_type_uri,
            parties=members,
            effective_timestamp=effective_timestamp,
            status=status,
            jurisdiction_iso=jurisdiction_iso,
            attestation=attestation,
        )

    def with_parent(self, parent_id: str) -> "RelationshipRecord":
        """Return a new RelationshipRecord with parent relationship set."""
        return RelationshipRecord(
            verifiable_id=self.verifiable_id,
            relationship_type_uri=self.relationship_type_uri,
            parties=self.parties,
            effective_timestamp=self.effective_timestamp,
            status=self.status,
            jurisdiction_iso=self.jurisdiction_iso,
            attestation=self.attestation,
            schema_version=self.schema_version,
            parent_relationship_id=parent_id,
            expiration_timestamp=self.expiration_timestamp,
            financial_terms=self.financial_terms,
            terms_attributes=self.terms_attributes,
            source_references=self.source_references,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_expiration(self, timestamp: CanonicalTimestamp) -> "RelationshipRecord":
        """Return a new RelationshipRecord with expiration timestamp set."""
        return RelationshipRecord(
            verifiable_id=self.verifiable_id,
            relationship_type_uri=self.relationship_type_uri,
            parties=self.parties,
            effective_timestamp=self.effective_timestamp,
            status=self.status,
            jurisdiction_iso=self.jurisdiction_iso,
            attestation=self.attestation,
            schema_version=self.schema_version,
            parent_relationship_id=self.parent_relationship_id,
            expiration_timestamp=timestamp,
            financial_terms=self.financial_terms,
            terms_attributes=self.terms_attributes,
            source_references=self.source_references,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_financial_terms(self, terms: FinancialTerms) -> "RelationshipRecord":
        """Return a new RelationshipRecord with financial terms set."""
        return RelationshipRecord(
            verifiable_id=self.verifiable_id,
            relationship_type_uri=self.relationship_type_uri,
            parties=self.parties,
            effective_timestamp=self.effective_timestamp,
            status=self.status,
            jurisdiction_iso=self.jurisdiction_iso,
            attestation=self.attestation,
            schema_version=self.schema_version,
            parent_relationship_id=self.parent_relationship_id,
            expiration_timestamp=self.expiration_timestamp,
            financial_terms=terms,
            terms_attributes=self.terms_attributes,
            source_references=self.source_references,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_source_reference(self, reference: SourceReference) -> "RelationshipRecord":
        """Return a new RelationshipRecord with a source reference added."""
        refs = list(self.source_references) if self.source_references else []
        refs.append(reference)
        return RelationshipRecord(
            verifiable_id=self.verifiable_id,
            relationship_type_uri=self.relationship_type_uri,
            parties=self.parties,
            effective_timestamp=self.effective_timestamp,
            status=self.status,
            jurisdiction_iso=self.jurisdiction_iso,
            attestation=self.attestation,
            schema_version=self.schema_version,
            parent_relationship_id=self.parent_relationship_id,
            expiration_timestamp=self.expiration_timestamp,
            financial_terms=self.financial_terms,
            terms_attributes=self.terms_attributes,
            source_references=refs,
            previous_record_hash=self.previous_record_hash,
            revision_number=self.revision_number,
        )

    def with_previous_hash(self, hash_val: CanonicalHash) -> "RelationshipRecord":
        """Return a new RelationshipRecord with previous hash set."""
        return RelationshipRecord(
            verifiable_id=self.verifiable_id,
            relationship_type_uri=self.relationship_type_uri,
            parties=self.parties,
            effective_timestamp=self.effective_timestamp,
            status=self.status,
            jurisdiction_iso=self.jurisdiction_iso,
            attestation=self.attestation,
            schema_version=self.schema_version,
            parent_relationship_id=self.parent_relationship_id,
            expiration_timestamp=self.expiration_timestamp,
            financial_terms=self.financial_terms,
            terms_attributes=self.terms_attributes,
            source_references=self.source_references,
            previous_record_hash=hash_val,
            revision_number=self.revision_number,
        )

    def with_revision(self, revision: int) -> "RelationshipRecord":
        """Return a new RelationshipRecord with revision number set."""
        return RelationshipRecord(
            verifiable_id=self.verifiable_id,
            relationship_type_uri=self.relationship_type_uri,
            parties=self.parties,
            effective_timestamp=self.effective_timestamp,
            status=self.status,
            jurisdiction_iso=self.jurisdiction_iso,
            attestation=self.attestation,
            schema_version=self.schema_version,
            parent_relationship_id=self.parent_relationship_id,
            expiration_timestamp=self.expiration_timestamp,
            financial_terms=self.financial_terms,
            terms_attributes=self.terms_attributes,
            source_references=self.source_references,
            previous_record_hash=self.previous_record_hash,
            revision_number=revision,
        )

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields in alphabetical order."""
        fields: dict[str, str] = {}

        # All fields in alphabetical order
        insert_required(fields, "attestation", self.attestation.to_canonical_string())
        insert_required(
            fields, "effectiveTimestamp", self.effective_timestamp.to_canonical_string()
        )
        if self.expiration_timestamp is not None:
            insert_required(
                fields, "expirationTimestamp", self.expiration_timestamp.to_canonical_string()
            )
        if self.financial_terms is not None:
            insert_required(fields, "financialTerms", self.financial_terms.to_canonical_string())
        insert_required(fields, "jurisdictionIso", self.jurisdiction_iso)
        insert_if_present(fields, "parentRelationshipId", self.parent_relationship_id)

        # Parties (bilateral or multilateral)
        if isinstance(self.parties, BilateralParties):
            insert_required(fields, "bilateralParties", self.parties.to_canonical_string())
        else:
            insert_required(fields, "multilateralMembers", self.parties.to_canonical_string())

        if self.previous_record_hash is not None:
            insert_required(fields, "previousRecordHash", self.previous_record_hash.as_hex())
        insert_required(fields, "relationshipTypeUri", self.relationship_type_uri)
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

        insert_required(fields, "status", self.status.to_canonical_string())

        # Terms attributes (already sorted as dict)
        if self.terms_attributes:
            import json

            fields["termsAttributes"] = json.dumps(
                dict(sorted(self.terms_attributes.items())), separators=(",", ":")
            )

        insert_required(fields, "verifiableId", self.verifiable_id)

        return fields
