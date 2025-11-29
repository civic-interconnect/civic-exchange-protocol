"""Entity identifier types for CEP.

CEP supports multiple identifier schemes organized into tiers:

- Tier 1 (Global): LEI (Legal Entity Identifier)
- Tier 2 (Federal): SAM.gov UEI
- Tier 3 (Sub-National): SNFEI (generated hash-based identifier)
- Extended: Canadian BN, UK Companies House, etc.
"""

from dataclasses import dataclass
import json
from typing import Optional

from civic_exchange_protocol.core import Canonicalize, insert_if_present
from civic_exchange_protocol.snfei import Snfei


@dataclass(frozen=True)
class SamUei:
    """SAM.gov Unique Entity Identifier (12 alphanumeric characters)."""

    value: str

    def __post_init__(self) -> None:
        """Validate the SAM UEI format after initialization."""
        if not self._is_valid(self.value):
            raise ValueError(f"Invalid SAM UEI: {self.value}")

    @staticmethod
    def _is_valid(value: str) -> bool:
        return (
            len(value) == 12 and all(c.isupper() or c.isdigit() for c in value) and value.isalnum()
        )

    @classmethod
    def new(cls, value: str) -> Optional["SamUei"]:
        """Create a new SAM UEI, returning None if invalid."""
        try:
            return cls(value)
        except ValueError:
            return None

    def as_str(self) -> str:
        """Return the SAM UEI as a string."""
        return self.value


@dataclass(frozen=True)
class Lei:
    """Legal Entity Identifier per ISO 17442 (20 alphanumeric characters)."""

    value: str

    def __post_init__(self) -> None:
        """Validate the LEI format after initialization."""
        if not self._is_valid(self.value):
            raise ValueError(f"Invalid LEI: {self.value}")

    @staticmethod
    def _is_valid(value: str) -> bool:
        return len(value) == 20 and value.isalnum()

    @classmethod
    def new(cls, value: str) -> Optional["Lei"]:
        """Create a new LEI, returning None if invalid."""
        try:
            return cls(value.upper())
        except ValueError:
            return None

    def as_str(self) -> str:
        """Return the LEI as a string."""
        return self.value


@dataclass(frozen=True)
class CanadianBn:
    """Canadian Business Number with program account."""

    value: str

    def __post_init__(self) -> None:
        """Validate the Canadian BN format after initialization."""
        if not self._is_valid(self.value):
            raise ValueError(f"Invalid Canadian BN: {self.value}")

    @staticmethod
    def _is_valid(value: str) -> bool:
        # Pattern: 9 digits + 2 letters + 4 digits (e.g., 123456789RC0001)
        if len(value) != 15:
            return False
        digits1 = value[:9]
        letters = value[9:11]
        digits2 = value[11:15]
        return digits1.isdigit() and letters.isalpha() and letters.isupper() and digits2.isdigit()

    @classmethod
    def new(cls, value: str) -> Optional["CanadianBn"]:
        """Create a new Canadian BN, returning None if invalid."""
        try:
            return cls(value)
        except ValueError:
            return None

    def as_str(self) -> str:
        """Return the Canadian BN as a string."""
        return self.value


@dataclass
class AdditionalScheme:
    """An additional identifier scheme not explicitly defined in the schema."""

    scheme_uri: str
    value: str


@dataclass
class EntityIdentifiers(Canonicalize):
    """Collection of all known identifiers for an entity."""

    sam_uei: SamUei | None = None
    lei: Lei | None = None
    snfei: Snfei | None = None
    canadian_bn: CanadianBn | None = None
    additional_schemes: list[AdditionalScheme] | None = None

    def with_sam_uei(self, uei: SamUei) -> "EntityIdentifiers":
        """Return a new EntityIdentifiers with the SAM UEI set."""
        return EntityIdentifiers(
            sam_uei=uei,
            lei=self.lei,
            snfei=self.snfei,
            canadian_bn=self.canadian_bn,
            additional_schemes=self.additional_schemes,
        )

    def with_lei(self, lei: Lei) -> "EntityIdentifiers":
        """Return a new EntityIdentifiers with the LEI set."""
        return EntityIdentifiers(
            sam_uei=self.sam_uei,
            lei=lei,
            snfei=self.snfei,
            canadian_bn=self.canadian_bn,
            additional_schemes=self.additional_schemes,
        )

    def with_snfei(self, snfei: Snfei) -> "EntityIdentifiers":
        """Return a new EntityIdentifiers with the SNFEI set."""
        return EntityIdentifiers(
            sam_uei=self.sam_uei,
            lei=self.lei,
            snfei=snfei,
            canadian_bn=self.canadian_bn,
            additional_schemes=self.additional_schemes,
        )

    def has_any(self) -> bool:
        """Return True if at least one identifier is present."""
        return (
            self.sam_uei is not None
            or self.lei is not None
            or self.snfei is not None
            or self.canadian_bn is not None
            or (self.additional_schemes is not None and len(self.additional_schemes) > 0)
        )

    def primary_identifier(self) -> str | None:
        """Return the 'best' identifier for use as the verifiable ID.

        Priority: LEI > SAM UEI > SNFEI > Canadian BN > first additional
        """
        if self.lei is not None:
            return f"cep-entity:lei:{self.lei.as_str()}"
        if self.sam_uei is not None:
            return f"cep-entity:sam-uei:{self.sam_uei.as_str()}"
        if self.snfei is not None:
            return f"cep-entity:snfei:{self.snfei.as_str()}"
        if self.canadian_bn is not None:
            return f"cep-entity:canadian-bn:{self.canadian_bn.as_str()}"
        if self.additional_schemes and len(self.additional_schemes) > 0:
            return f"cep-entity:other:{self.additional_schemes[0].value}"
        return None

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields in alphabetical order."""
        fields: dict[str, str] = {}

        # Additional schemes serialized as JSON array string
        if self.additional_schemes and len(self.additional_schemes) > 0:
            sorted_schemes = sorted(self.additional_schemes, key=lambda x: x.scheme_uri)
            schemes_data = [{"schemeUri": s.scheme_uri, "value": s.value} for s in sorted_schemes]
            fields["additionalSchemes"] = json.dumps(schemes_data, separators=(",", ":"))

        insert_if_present(
            fields, "canadianBn", self.canadian_bn.as_str() if self.canadian_bn else None
        )
        insert_if_present(fields, "lei", self.lei.as_str() if self.lei else None)
        insert_if_present(fields, "samUei", self.sam_uei.as_str() if self.sam_uei else None)
        insert_if_present(fields, "snfei", self.snfei.as_str() if self.snfei else None)

        return fields
