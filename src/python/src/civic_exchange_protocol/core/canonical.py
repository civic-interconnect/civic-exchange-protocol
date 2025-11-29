"""Canonical serialization for CEP records.

This module provides the base class and utilities for generating
deterministic canonical strings from CEP records. The canonical string
is the input to SHA-256 hashing for record integrity verification.

Canonicalization Rules:
1. Field Order: Fields MUST be serialized in alphabetical order.
2. Null/Empty Omission: Fields with None or empty string values
   MUST be omitted entirely from the canonical string.
3. Timestamp Format: All timestamps MUST use YYYY-MM-DDTHH:MM:SS.ffffffZ
   with exactly 6 decimal places for microseconds.
4. Numeric Format: Monetary amounts MUST use exactly 2 decimal places.
   Integers MUST NOT have decimal points.
5. String Escaping: Strings are NOT JSON-escaped in the canonical form.
   The canonical string is a simple key:value concatenation.
6. Encoding: The canonical string MUST be UTF-8 encoded.
"""

from abc import ABC, abstractmethod
from decimal import ROUND_HALF_UP, Decimal

from .hash import CanonicalHash


class Canonicalize(ABC):
    """Base class for types that can be serialized to a canonical string for hashing."""

    @abstractmethod
    def canonical_fields(self) -> dict[str, str]:
        """Return the ordered map of field names to their canonical string values.

        Fields with None/null/empty values should NOT be included in the dict.
        The dict will be sorted alphabetically by key.

        Returns:
            A dictionary of field names to string values.
        """
        pass

    def to_canonical_string(self) -> str:
        """Generate the canonical string representation for hashing.

        Format: "field1":"value1","field2":"value2",...

        Fields are ordered alphabetically by key.

        Returns:
            The canonical string.
        """
        fields = self.canonical_fields()
        # Sort by key alphabetically
        sorted_items = sorted(fields.items(), key=lambda x: x[0])
        parts = [f'"{k}":"{v}"' for k, v in sorted_items]
        return ",".join(parts)

    def calculate_hash(self) -> CanonicalHash:
        """Compute the SHA-256 hash of the canonical string.

        Returns:
            A CanonicalHash instance.
        """
        return CanonicalHash.from_canonical_string(self.to_canonical_string())


def format_amount(amount: float) -> str:
    """Format a monetary amount with exactly 2 decimal places.

    This ensures consistent formatting across all implementations:
    - 100 becomes "100.00"
    - 100.5 becomes "100.50"
    - 100.756 becomes "100.76" (rounded)

    Args:
        amount: The monetary amount.

    Returns:
        The formatted string with exactly 2 decimal places.
    """
    # Use Decimal for precise rounding
    d = Decimal(str(amount))
    rounded = d.quantize(Decimal("0.01"), rounding=ROUND_HALF_UP)
    return f"{rounded:.2f}"


def insert_if_present(fields: dict[str, str], key: str, value: str | None) -> None:
    """Add a field to the dict only if the value is not None and not empty.

    Args:
        fields: The dictionary to add to.
        key: The field name.
        value: The field value (may be None or empty).
    """
    if value is not None and value != "":
        fields[key] = value


def insert_required(fields: dict[str, str], key: str, value: str) -> None:
    """Add a required field to the dict.

    Args:
        fields: The dictionary to add to.
        key: The field name.
        value: The field value.
    """
    fields[key] = value
