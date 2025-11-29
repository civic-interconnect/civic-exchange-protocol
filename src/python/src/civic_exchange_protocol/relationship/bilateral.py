"""Bilateral party definitions for two-party relationships.

Bilateral relationships have clear directionality:
- Party A: The initiating, granting, or contracting party
- Party B: The receiving, performing, or beneficiary party
"""

from dataclasses import dataclass

from civic_exchange_protocol.core import Canonicalize, insert_required


@dataclass
class Party(Canonicalize):
    """A party in a bilateral relationship."""

    entity_id: str
    role_uri: str

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical field representation of the party.

        Returns:
        -------
        dict[str, str]
            A dictionary containing the canonical fields with entityId and roleUri.
        """
        fields: dict[str, str] = {}
        insert_required(fields, "entityId", self.entity_id)
        insert_required(fields, "roleUri", self.role_uri)
        return fields


@dataclass
class BilateralParties(Canonicalize):
    """Bilateral parties in a two-party relationship."""

    party_a: Party  # Initiating, granting, or contracting party
    party_b: Party  # Receiving, performing, or beneficiary party

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical field representation of bilateral parties.

        Returns:
        -------
        dict[str, str]
            A dictionary containing the canonical fields with partyA and partyB.
        """
        fields: dict[str, str] = {}
        # Nested objects serialized as their canonical strings
        insert_required(fields, "partyA", self.party_a.to_canonical_string())
        insert_required(fields, "partyB", self.party_b.to_canonical_string())
        return fields
