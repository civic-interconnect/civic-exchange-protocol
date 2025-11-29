"""CEP Core Linker: Entity Resolution and SNFEI Generation.

This package implements the Normalizing Functor architecture for generating
deterministic entity identifiers (SNFEIs) from heterogeneous source data.

Architecture:
    ┌──────────────┐     ┌────────────────┐     ┌─────────────┐
    │  Raw Entity  │     │  Intermediate  │     │  Canonical  │
    │    Data      │───> │    Canonical   │───> │   Entity    │
    │              │  L  │                │  N  │             │
    └──────────────┘     └────────────────┘     └─────────────┘
                                                        │
                                                        │ SHA-256
                                                        V
                                                ┌──────────────┐
                                                │    SNFEI     │
                                                │  (64-char)   │
                                                └──────────────┘

    L = Localization Functor (jurisdiction-specific transforms)
    N = Normalizing Functor (universal normalization)

Usage:
    from civic_exchange_protocol.core_linker import (
        generate_snfei,
        normalize_legal_name,
        apply_localization,
    )

    # Simple SNFEI generation
    snfei, inputs = generate_snfei(
        legal_name="Springfield USD #12",
        country_code="US",
        address="123 Main St",
    )

    # With jurisdiction-specific localization
    from civic_exchange_protocol.core_linker import apply_localization
    localized = apply_localization("MTA", "us/ny")
    # -> "metropolitan transportation authority"
"""

from .generator import (
    Snfei,
    SnfeiResult,
    compute_snfei,
    generate_snfei,
    generate_snfei_simple,
    generate_snfei_with_confidence,
)
from .localization import (
    LocalizationConfig,
    LocalizationRegistry,
    LocalizationRule,
    apply_localization,
    get_localization_config,
)
from .normalizer import (
    CanonicalInput,
    build_canonical_input,
    normalize_address,
    normalize_legal_name,
    normalize_registration_date,
)

__all__ = [
    # SNFEI Generation
    "Snfei",
    "SnfeiResult",
    "generate_snfei",
    "generate_snfei_simple",
    "generate_snfei_with_confidence",
    "compute_snfei",
    # Normalization
    "normalize_legal_name",
    "normalize_address",
    "normalize_registration_date",
    "CanonicalInput",
    "build_canonical_input",
    # Localization
    "apply_localization",
    "get_localization_config",
    "LocalizationConfig",
    "LocalizationRegistry",
    "LocalizationRule",
]
