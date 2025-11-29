"""CEP Entity - Entity records for the Civic Exchange Protocol.

This package defines the EntityRecord type, which represents a verified
civic entity. Entities are the foundational primitive in CEP—all relationships
and exchanges reference attested entities.
"""

from .entity import (
    EntityRecord,
    EntityStatus,
    EntityStatusCode,
    ResolutionConfidence,
)
from .identifiers import (
    AdditionalScheme,
    CanadianBn,
    EntityIdentifiers,
    Lei,
    SamUei,
    Snfei,
)

__all__ = [
    # Entity
    "EntityRecord",
    "EntityStatus",
    "EntityStatusCode",
    "ResolutionConfidence",
    # Identifiers
    "EntityIdentifiers",
    "SamUei",
    "Lei",
    "Snfei",
    "CanadianBn",
    "AdditionalScheme",
]
