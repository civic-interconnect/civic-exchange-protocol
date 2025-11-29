"""Civic Exchange Protocol (CEP) - Python Implementation.

A protocol for transparent, verifiable civic data exchange.

Submodules:
    core: Core types (timestamps, hashes, attestations)
    core_linker: SNFEI generation and entity normalization
    entity: Entity records and identifiers
    relationship: Relationship records
    exchange: Exchange records
"""

from importlib.metadata import PackageNotFoundError
from importlib.metadata import version as _version

# Make submodules importable
from . import core, entity, exchange, relationship, snfei

try:
    # Distribution name from pyproject.toml
    __version__ = _version("civic-exchange-protocol")
except PackageNotFoundError:  # pragma: no cover - mainly for editable installs
    __version__ = "0.0.0"

__all__ = [
    "core",
    "entity",
    "exchange",
    "relationship",
    "snfei",
]
