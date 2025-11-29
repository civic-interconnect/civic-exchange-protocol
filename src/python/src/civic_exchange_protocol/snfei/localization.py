"""Localization Functor: Jurisdiction-Specific Transformations.

This module loads and applies jurisdiction-specific normalization rules
BEFORE the universal Normalizing Functor is applied.

The Localization Functor L transforms raw local data into a canonical
intermediate form that the universal normalizer can process:

    L: RawLocal → IntermediateCanonical
    N: IntermediateCanonical → FinalCanonical

    SNFEI = Hash(N(L(raw_data)))

Directory Structure:
    /localization/
        base.yaml           # Default/fallback rules
        US/
            base.yaml       # US-wide rules
            CA.yaml         # California-specific
            NY.yaml         # New York-specific
        CA/
            base.yaml       # Canada-wide rules
            ON.yaml         # Ontario-specific
            QC.yaml         # Quebec-specific
"""

from dataclasses import dataclass, field
import os
from pathlib import Path

# Note: PyYAML would be imported here in production
# import yaml


@dataclass
class LocalizationRule:
    """A single localization transformation rule."""

    pattern: str  # Text to match (case-insensitive)
    replacement: str  # Replacement text
    is_regex: bool = False  # Whether pattern is a regex
    context: str | None = None  # Optional context (e.g., "agency", "school")


@dataclass
class LocalizationConfig:
    """Configuration for a specific jurisdiction."""

    jurisdiction: str  # e.g., "US/CA", "CA/ON"
    parent: str | None  # Parent jurisdiction for inheritance

    # Transformation maps
    abbreviations: dict[str, str] = field(default_factory=dict)
    agency_names: dict[str, str] = field(default_factory=dict)
    entity_types: dict[str, str] = field(default_factory=dict)

    # Additional rules
    rules: list[LocalizationRule] = field(default_factory=list)

    # Stop words specific to this jurisdiction
    stop_words: set[str] = field(default_factory=set)

    def apply_to_name(self, name: str) -> str:
        """Apply jurisdiction-specific transformations to a name.

        Order of application:
        1. Agency name expansions
        2. Abbreviation expansions
        3. Entity type standardization
        4. Custom rules
        """
        result = name.lower()

        # 1. Agency names (exact match, case-insensitive)
        for abbrev, full in self.agency_names.items():
            # Word boundary matching
            import re

            pattern = r"\b" + re.escape(abbrev.lower()) + r"\b"
            result = re.sub(pattern, full.lower(), result)

        # 2. Abbreviations
        tokens = result.split()
        expanded = []
        for token in tokens:
            if token in self.abbreviations:
                expanded.append(self.abbreviations[token].lower())
            else:
                expanded.append(token)
        result = " ".join(expanded)

        # 3. Entity types
        for local_type, canonical_type in self.entity_types.items():
            import re

            pattern = r"\b" + re.escape(local_type.lower()) + r"\b"
            result = re.sub(pattern, canonical_type.lower(), result)

        # 4. Custom rules
        for rule in self.rules:
            if rule.is_regex:
                import re

                result = re.sub(rule.pattern, rule.replacement, result, flags=re.IGNORECASE)
            else:
                result = result.replace(rule.pattern.lower(), rule.replacement.lower())

        return result


# =============================================================================
# BUILT-IN LOCALIZATION CONFIGS
# =============================================================================

# These would normally be loaded from YAML files, but we include
# common ones as built-in defaults.

US_BASE_CONFIG = LocalizationConfig(
    jurisdiction="US",
    parent=None,
    abbreviations={
        # Federal agencies
        "doj": "department of justice",
        "dod": "department of defense",
        "hhs": "department of health and human services",
        "doe": "department of energy",
        "ed": "department of education",
        "dot": "department of transportation",
        "dhs": "department of homeland security",
        "usda": "united states department of agriculture",
        "epa": "environmental protection agency",
        "fda": "food and drug administration",
        "fcc": "federal communications commission",
        "ftc": "federal trade commission",
        "sec": "securities and exchange commission",
        "irs": "internal revenue service",
        "ssa": "social security administration",
        "va": "veterans administration",
        "nasa": "national aeronautics and space administration",
        "fbi": "federal bureau of investigation",
        "cia": "central intelligence agency",
        "nsa": "national security agency",
    },
    entity_types={
        "k-12": "kindergarten through twelfth grade",
        "k12": "kindergarten through twelfth grade",
        "501c3": "five oh one c three",
        "501(c)(3)": "five oh one c three",
        "501c4": "five oh one c four",
        "501(c)(4)": "five oh one c four",
    },
)

US_CA_CONFIG = LocalizationConfig(
    jurisdiction="US/CA",
    parent="US",
    abbreviations={
        "caltrans": "california department of transportation",
        "calpers": "california public employees retirement system",
        "calstrs": "california state teachers retirement system",
        "uc": "university of california",
        "csu": "california state university",
        "lausd": "los angeles unified school district",
        "sfusd": "san francisco unified school district",
        "ousd": "oakland unified school district",
    },
    agency_names={
        "dmv": "department of motor vehicles",
        "edd": "employment development department",
        "ftb": "franchise tax board",
        "boe": "board of equalization",
        "cdcr": "california department of corrections and rehabilitation",
        "chp": "california highway patrol",
    },
)

US_NY_CONFIG = LocalizationConfig(
    jurisdiction="US/NY",
    parent="US",
    abbreviations={
        "mta": "metropolitan transportation authority",
        "nycha": "new york city housing authority",
        "nypd": "new york police department",
        "fdny": "fire department new york",
        "suny": "state university of new york",
        "cuny": "city university of new york",
        "dot": "department of transportation",  # NY DOT
        "dec": "department of environmental conservation",
    },
    agency_names={
        "port authority": "port authority of new york and new jersey",
        "thruway": "new york state thruway authority",
        "lirr": "long island rail road",
    },
)

CA_BASE_CONFIG = LocalizationConfig(
    jurisdiction="CA",
    parent=None,
    abbreviations={
        "rcmp": "royal canadian mounted police",
        "cra": "canada revenue agency",
        "cbsa": "canada border services agency",
    },
    entity_types={
        "ltée": "limitee",
        "ltee": "limitee",
        "inc": "incorporated",
        "enr": "enregistree",  # Quebec registered
    },
)

CA_ON_CONFIG = LocalizationConfig(
    jurisdiction="CA/ON",
    parent="CA",
    abbreviations={
        "ttc": "toronto transit commission",
        "omb": "ontario municipal board",
        "wsib": "workplace safety and insurance board",
        "lcbo": "liquor control board of ontario",
    },
)

CA_QC_CONFIG = LocalizationConfig(
    jurisdiction="CA/QC",
    parent="CA",
    abbreviations={
        "stm": "societe de transport de montreal",
        "saaq": "societe de assurance automobile du quebec",
        "hydro-quebec": "hydro quebec",
    },
    entity_types={
        # French to English standardization
        "limitée": "limitee",
        "incorporée": "incorporated",
        "société": "societe",
        "compagnie": "company",
    },
    rules=[
        # Remove accents from common French words
        LocalizationRule(pattern="é", replacement="e"),
        LocalizationRule(pattern="è", replacement="e"),
        LocalizationRule(pattern="ê", replacement="e"),
        LocalizationRule(pattern="à", replacement="a"),
        LocalizationRule(pattern="â", replacement="a"),
        LocalizationRule(pattern="ô", replacement="o"),
        LocalizationRule(pattern="û", replacement="u"),
        LocalizationRule(pattern="ç", replacement="c"),
    ],
)


# =============================================================================
# LOCALIZATION REGISTRY
# =============================================================================

BUILT_IN_CONFIGS: dict[str, LocalizationConfig] = {
    "US": US_BASE_CONFIG,
    "US/CA": US_CA_CONFIG,
    "US/NY": US_NY_CONFIG,
    "CA": CA_BASE_CONFIG,
    "CA/ON": CA_ON_CONFIG,
    "CA/QC": CA_QC_CONFIG,
}


class LocalizationRegistry:
    """Registry for loading and caching localization configurations."""

    def __init__(self, config_dir: Path | None = None):
        """Initialize the registry.

        Args:
            config_dir: Optional path to localization YAML files.
                       If None, only built-in configs are available.
        """
        self.config_dir = config_dir
        self._cache: dict[str, LocalizationConfig] = dict(BUILT_IN_CONFIGS)

    def get_config(self, jurisdiction: str) -> LocalizationConfig:
        """Get localization config for a jurisdiction.

        Falls back through parent jurisdictions if specific config not found.

        Args:
            jurisdiction: Jurisdiction code (e.g., "US/CA", "CA/ON").

        Returns:
            LocalizationConfig for the jurisdiction.
        """
        # Check cache
        if jurisdiction in self._cache:
            return self._cache[jurisdiction]

        # Try to load from YAML
        if self.config_dir:
            config = self._load_yaml(jurisdiction)
            if config:
                self._cache[jurisdiction] = config
                return config

        # Fall back to parent
        if "/" in jurisdiction:
            parent = jurisdiction.rsplit("/", 1)[0]
            return self.get_config(parent)

        # Return empty config as last resort
        return LocalizationConfig(jurisdiction=jurisdiction, parent=None)

    def _load_yaml(self, jurisdiction: str) -> LocalizationConfig | None:
        """Load config from YAML file.

        Expected path: config_dir/{jurisdiction}.yaml
        e.g., config_dir/US/CA.yaml
        """
        if not self.config_dir:
            return None

        # Try exact path
        yaml_path = self.config_dir / f"{jurisdiction.replace('/', os.sep)}.yaml"
        if not yaml_path.exists():
            return None

        # Would parse YAML here
        # For now, return None (not implemented without PyYAML)
        return None

    def merge_configs(
        self, child: LocalizationConfig, parent: LocalizationConfig
    ) -> LocalizationConfig:
        """Merge child config with parent (child overrides parent)."""
        merged_abbrevs = dict(parent.abbreviations)
        merged_abbrevs.update(child.abbreviations)

        merged_agencies = dict(parent.agency_names)
        merged_agencies.update(child.agency_names)

        merged_types = dict(parent.entity_types)
        merged_types.update(child.entity_types)

        return LocalizationConfig(
            jurisdiction=child.jurisdiction,
            parent=parent.jurisdiction,
            abbreviations=merged_abbrevs,
            agency_names=merged_agencies,
            entity_types=merged_types,
            rules=parent.rules + child.rules,
            stop_words=parent.stop_words | child.stop_words,
        )


# Global registry instance
_registry = LocalizationRegistry()


def get_localization_config(jurisdiction: str) -> LocalizationConfig:
    """Get localization config for a jurisdiction (convenience function)."""
    return _registry.get_config(jurisdiction)


def apply_localization(name: str, jurisdiction: str) -> str:
    """Apply localization transforms to a name.

    This is the Localization Functor L.

    Args:
        name: Raw entity name.
        jurisdiction: Jurisdiction code (e.g., "US/CA").

    Returns:
        Name with jurisdiction-specific transforms applied.
    """
    config = get_localization_config(jurisdiction)
    return config.apply_to_name(name)
