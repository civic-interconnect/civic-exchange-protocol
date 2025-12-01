"""CEP Core - Core primitives for the Civic Exchange Protocol.

This package provides the foundational types used by all CEP record types:

- CanonicalTimestamp: Microsecond-precision UTC timestamps
- CanonicalHash: SHA-256 hash values
- Canonicalize: Base class for deterministic serialization
- Attestation: Cryptographic proof of record integrity
- Schema Registry: Central schema loading and validation support
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
from .schema_registry import get_registry, get_schema, list_schemas
from .timestamp import CanonicalTimestamp
from .version import SCHEMA_VERSION, __version__

__all__ = [
    # Version
    "SCHEMA_VERSION",
    "__version__",
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
    # Schema Registry
    "get_schema",
    "get_registry",
    "list_schemas",
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
