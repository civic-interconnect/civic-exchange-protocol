"""CEP Relationship - Relationship records for the Civic Exchange Protocol.

This package defines the RelationshipRecord type, which represents a verifiable legal or functional
relationship between two or more attested entities.
"""

from .bilateral import BilateralParties, Party
from .multilateral import Member, MultilateralMembers
from .relationship import (
    FinancialTerms,
    Parties,
    RelationshipRecord,
    RelationshipStatus,
    RelationshipStatusCode,
    SourceReference,
)

__all__ = [
    # Bilateral
    "Party",
    "BilateralParties",
    # Multilateral
    "Member",
    "MultilateralMembers",
    # Relationship
    "RelationshipRecord",
    "RelationshipStatus",
    "RelationshipStatusCode",
    "FinancialTerms",
    "SourceReference",
    "Parties",
]
