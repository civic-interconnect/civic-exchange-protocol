"""Cryptographic hashing utilities for CEP records.

All CEP hashes are SHA-256, represented as lowercase hexadecimal strings.
"""

import hashlib
from typing import Optional


class CanonicalHash:
    """A SHA-256 hash value represented as a 64-character lowercase hex string."""

    __slots__ = ("_hex",)

    def __init__(self, hex_value: str) -> None:
        """Create a CanonicalHash from a hex string.

        Args:
            hex_value: A 64-character hexadecimal string.

        Raises:
            ValueError: If the string is not valid.
        """
        if len(hex_value) != 64:
            raise ValueError(f"Hash must be 64 hex characters, got {len(hex_value)}")
        if not all(c in "0123456789abcdefABCDEF" for c in hex_value):
            raise ValueError("Hash must contain only hexadecimal characters")
        self._hex = hex_value.lower()

    @classmethod
    def from_canonical_string(cls, canonical: str) -> "CanonicalHash":
        """Compute the SHA-256 hash of the given canonical string.

        Args:
            canonical: The canonical string to hash.

        Returns:
            A CanonicalHash instance.
        """
        hasher = hashlib.sha256()
        hasher.update(canonical.encode("utf-8"))
        return cls(hasher.hexdigest())

    @classmethod
    def from_hex(cls, hex_value: str) -> Optional["CanonicalHash"]:
        """Create a CanonicalHash from a pre-computed hex string.

        Args:
            hex_value: A hexadecimal string.

        Returns:
            A CanonicalHash instance, or None if invalid.
        """
        try:
            return cls(hex_value)
        except ValueError:
            return None

    def as_hex(self) -> str:
        """Return the hash as a lowercase hex string."""
        return self._hex

    def as_bytes(self) -> bytes:
        """Return the hash as bytes (32 bytes)."""
        return bytes.fromhex(self._hex)

    def __str__(self) -> str:
        """Return a string representation of the hash."""
        return self._hex

    def __repr__(self) -> str:
        """Return a detailed string representation of the hash."""
        return f"CanonicalHash({self._hex!r})"

    def __eq__(self, other: object) -> bool:
        """Check equality with another CanonicalHash instance."""
        if isinstance(other, CanonicalHash):
            return self._hex == other._hex
        return NotImplemented

    def __hash__(self) -> int:
        """Return a hash value for the CanonicalHash instance."""
        return hash(self._hex)
