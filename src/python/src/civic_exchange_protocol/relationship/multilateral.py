"""Multilateral member definitions for n-ary relationships.

Multilateral relationships involve more than two parties, such as:
- Consortia
- Joint ventures
- Board memberships

Members are sorted by entity_id to guarantee deterministic ordering
for hash stability across all implementations.
"""

from collections.abc import Iterator
from dataclasses import dataclass

from civic_exchange_protocol.core import Canonicalize, insert_required


@dataclass
class Member(Canonicalize):
    """A member in a multilateral relationship."""

    entity_id: str
    role_uri: str
    participation_share: float | None = None

    def with_share(self, share: float) -> "Member":
        """Return a new Member with the participation share set."""
        return Member(
            entity_id=self.entity_id,
            role_uri=self.role_uri,
            participation_share=share,
        )

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical field representation of the member.

        Returns:
        -------
        dict[str, str]
            Dictionary containing entityId, roleUri, and optionally participationShare.
        """
        fields: dict[str, str] = {}
        insert_required(fields, "entityId", self.entity_id)
        if self.participation_share is not None:
            insert_required(fields, "participationShare", f"{self.participation_share:.4f}")
        insert_required(fields, "roleUri", self.role_uri)
        return fields


class MultilateralMembers(Canonicalize):
    """A collection of members in a multilateral relationship.

    Members are automatically sorted by entity_id to ensure
    hash stability regardless of insertion order.
    """

    def __init__(self) -> None:
        """Initialize an empty collection of multilateral relationship members."""
        self._members: list[Member] = []

    def add(self, member: Member) -> None:
        """Add a member to the set."""
        # Check for duplicate entity_id
        for existing in self._members:
            if existing.entity_id == member.entity_id:
                return  # Already exists
        self._members.append(member)

    def __len__(self) -> int:
        """Return the number of members in the collection."""
        return len(self._members)

    def __iter__(self) -> Iterator[Member]:
        """Iterate over members in sorted order by entity_id."""
        return iter(sorted(self._members, key=lambda m: m.entity_id))

    def is_empty(self) -> bool:
        """Check if the collection has no members.

        Returns:
        -------
        bool
            True if the collection is empty, False otherwise.
        """
        return len(self._members) == 0

    def validate_shares(self) -> None:
        """Validate that all participation shares sum to 1.0 (if present).

        Raises:
            ValueError: If validation fails.
        """
        shares = [m.participation_share for m in self._members if m.participation_share is not None]

        if not shares:
            return

        if len(shares) != len(self._members):
            raise ValueError("All members must have participation shares if any do")

        total = sum(shares)
        if abs(total - 1.0) > 0.0001:
            raise ValueError(f"Participation shares must sum to 1.0, got {total:.4f}")

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields."""
        fields: dict[str, str] = {}

        # Serialize as array, members sorted by entity_id
        if self._members:
            sorted_members = sorted(self._members, key=lambda m: m.entity_id)
            members_json = ",".join(m.to_canonical_string() for m in sorted_members)
            fields["members"] = f"[{members_json}]"

        return fields
