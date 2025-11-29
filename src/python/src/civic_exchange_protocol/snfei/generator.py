"""SNFEI Hash Generation.

This module computes the final SNFEI (Sub-National Federated Entity Identifier)
from normalized entity attributes.

The SNFEI formula:
    SNFEI = SHA256(Concatenate[
        legal_name_normalized,
        address_normalized,
        country_code,
        registration_date
    ])

All inputs must pass through the Normalizing Functor before hashing.
"""

from dataclasses import dataclass
import hashlib

from .normalizer import (
    CanonicalInput,
    build_canonical_input,
)


@dataclass(frozen=True)
class Snfei:
    """A validated SNFEI (64-character lowercase hex string)."""

    value: str

    def __post_init__(self) -> None:
        """Validate SNFEI format after initialization."""
        if len(self.value) != 64:
            raise ValueError(f"SNFEI must be 64 characters, got {len(self.value)}")
        if not all(c in "0123456789abcdef" for c in self.value):
            raise ValueError("SNFEI must be lowercase hex")

    def __str__(self) -> str:
        """Return string representation of SNFEI."""
        return self.value

    def __repr__(self) -> str:
        """Return abbreviated representation of SNFEI."""
        return f"Snfei('{self.value[:8]}...{self.value[-8:]}')"

    def short(self, length: int = 12) -> str:
        """Return a shortened version for display."""
        return self.value[:length]


def compute_snfei(canonical: CanonicalInput) -> Snfei:
    """Compute SNFEI from canonical input.

    Args:
        canonical: Normalized input structure.

    Returns:
        Computed SNFEI.
    """
    hash_input = canonical.to_hash_string()
    hash_bytes = hashlib.sha256(hash_input.encode("utf-8")).hexdigest().lower()
    return Snfei(hash_bytes)


def generate_snfei(
    legal_name: str,
    country_code: str,
    address: str | None = None,
    registration_date: str | None = None,
) -> tuple[Snfei, CanonicalInput]:
    """Generate an SNFEI from raw entity attributes.

    This is the main entry point for SNFEI generation. It applies the
    Normalizing Functor to all inputs before hashing.

    Args:
        legal_name: Raw legal name from source system.
        country_code: ISO 3166-1 alpha-2 country code (e.g., "US", "CA").
        address: Optional primary street address.
        registration_date: Optional formation/registration date.

    Returns:
        Tuple of (SNFEI, CanonicalInput) for verification.

    Example:
        >>> snfei, inputs = generate_snfei(
        ...     legal_name="Springfield Unified Sch. Dist., Inc.",
        ...     country_code="US",
        ...     address="123 Main St., Suite 100",
        ...     registration_date="01/15/1985",
        ... )
        >>> print(snfei)
        a1b2c3d4...
        >>> print(inputs.legal_name_normalized)
        springfield unified school district incorporated
    """
    canonical = build_canonical_input(
        legal_name=legal_name,
        country_code=country_code,
        address=address,
        registration_date=registration_date,
    )
    snfei = compute_snfei(canonical)
    return snfei, canonical


def generate_snfei_simple(
    legal_name: str,
    country_code: str,
    address: str | None = None,
) -> str:
    """Generate SNFEI as a simple hex string.

    Convenience function that returns just the hash value.

    Args:
        legal_name: Raw legal name.
        country_code: ISO 3166-1 alpha-2 country code.
        address: Optional primary street address.

    Returns:
        64-character lowercase hex SNFEI string.
    """
    snfei, _ = generate_snfei(legal_name, country_code, address)
    return snfei.value


# =============================================================================
# TIER-BASED SNFEI (WITH CONFIDENCE SCORING)
# =============================================================================


@dataclass
class SnfeiResult:
    """Result of SNFEI generation with confidence metadata."""

    snfei: Snfei
    canonical: CanonicalInput
    confidence_score: float  # 0.0 to 1.0
    tier: int  # 1, 2, or 3
    fields_used: list  # Which fields contributed

    def to_dict(self) -> dict:
        """Convert result to dictionary for serialization."""
        return {
            "snfei": self.snfei.value,
            "confidence_score": self.confidence_score,
            "tier": self.tier,
            "fields_used": self.fields_used,
            "canonical": {
                "legal_name_normalized": self.canonical.legal_name_normalized,
                "address_normalized": self.canonical.address_normalized,
                "country_code": self.canonical.country_code,
                "registration_date": self.canonical.registration_date,
            },
        }


def generate_snfei_with_confidence(
    legal_name: str,
    country_code: str,
    address: str | None = None,
    registration_date: str | None = None,
    lei: str | None = None,
    sam_uei: str | None = None,
) -> SnfeiResult:
    """Generate SNFEI with confidence scoring and tier classification.

    Tier Classification:
    - Tier 1: Entity has LEI (global identifier) - confidence 1.0
    - Tier 2: Entity has SAM UEI (federal identifier) - confidence 0.95
    - Tier 3: Entity uses SNFEI (computed hash) - confidence varies

    Tier 3 Confidence Scoring:
    - Base: 0.5 (name + country only)
    - +0.2 if address is provided
    - +0.2 if registration_date is provided
    - +0.1 if name is reasonably long (>3 words)

    Args:
        legal_name: Raw legal name.
        country_code: ISO 3166-1 alpha-2 country code.
        address: Optional street address.
        registration_date: Optional registration date.
        lei: Optional LEI (Legal Entity Identifier).
        sam_uei: Optional SAM.gov Unique Entity Identifier.

    Returns:
        SnfeiResult with SNFEI, confidence score, and metadata.
    """
    fields_used = ["legal_name", "country_code"]

    # Tier 1: LEI available
    if lei and len(lei) == 20:
        canonical = build_canonical_input(legal_name, country_code, address, registration_date)
        # For Tier 1, we still compute SNFEI but confidence is 1.0
        snfei = compute_snfei(canonical)
        return SnfeiResult(
            snfei=snfei,
            canonical=canonical,
            confidence_score=1.0,
            tier=1,
            fields_used=["lei"] + fields_used,
        )

    # Tier 2: SAM UEI available
    if sam_uei and len(sam_uei) == 12:
        canonical = build_canonical_input(legal_name, country_code, address, registration_date)
        snfei = compute_snfei(canonical)
        return SnfeiResult(
            snfei=snfei,
            canonical=canonical,
            confidence_score=0.95,
            tier=2,
            fields_used=["sam_uei"] + fields_used,
        )

    # Tier 3: Compute SNFEI from attributes
    canonical = build_canonical_input(legal_name, country_code, address, registration_date)
    snfei = compute_snfei(canonical)

    # Calculate confidence score
    confidence = 0.5  # Base score

    if address:
        fields_used.append("address")
        confidence += 0.2

    if registration_date:
        fields_used.append("registration_date")
        confidence += 0.2

    # Bonus for longer, more specific names
    word_count = len(canonical.legal_name_normalized.split())
    if word_count > 3:
        confidence += 0.1

    # Cap at 0.9 for Tier 3
    confidence = min(confidence, 0.9)

    return SnfeiResult(
        snfei=snfei,
        canonical=canonical,
        confidence_score=round(confidence, 2),
        tier=3,
        fields_used=fields_used,
    )
