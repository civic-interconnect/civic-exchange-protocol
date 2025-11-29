"""CEP Core - Core primitives for the Civic Exchange Protocol.

This package provides the foundational types used by all CEP record types:

- CanonicalTimestamp: Microsecond-precision UTC timestamps
- CanonicalHash: SHA-256 hash values
- Canonicalize: Base class for deterministic serialization
- Attestation: Cryptographic proof of record integrity
"""

from .attestation import Attestation, ProofPurpose
from .canonical import (
    Canonicalize,
    format_amount,
    insert_if_present,
    insert_required,
)
from .error import (
    CepError,
    HashMismatchError,
    InvalidHashError,
    InvalidIdentifierError,
    InvalidTimestampError,
    MissingFieldError,
    RevisionChainError,
    UnsupportedVersionError,
)
from .hash import CanonicalHash
from .timestamp import CanonicalTimestamp

__version__ = "0.1.0"

SCHEMA_VERSION = "1.0.0"

__all__ = [
    # Version
    "SCHEMA_VERSION",
    # Timestamp
    "CanonicalTimestamp",
    # Hash
    "CanonicalHash",
    # Canonical
    "Canonicalize",
    "format_amount",
    "insert_if_present",
    "insert_required",
    # Attestation
    "Attestation",
    "ProofPurpose",
    # Errors
    "CepError",
    "InvalidTimestampError",
    "InvalidHashError",
    "InvalidIdentifierError",
    "MissingFieldError",
    "UnsupportedVersionError",
    "HashMismatchError",
    "RevisionChainError",
]
