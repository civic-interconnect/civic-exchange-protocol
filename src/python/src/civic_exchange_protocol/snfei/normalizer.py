"""CEP Core Linker: The Normalizing Functor.

This module implements the universal normalization pipeline that transforms
entity attributes into hash-ready canonical form for SNFEI generation.

The architecture follows the Category Theory foundation:
- Localization Functor: Jurisdiction-specific transforms (YAML-driven)
- Normalizing Functor: Universal normalization steps (this module)
- SNFEI Hash: Final SHA-256 computation

Directory Structure:
    /snfei/
        normalizer.py      # Universal normalization (this file)
        generator.py       # SNFEI hash generation
        localization.py    # Localization functor implementation
    /localization/
        us/               # US state-specific rules
            ca.yaml
            ny.yaml
        ca/               # Canada province-specific rules
            on.yaml
            qc.yaml
        base.yaml         # Fallback rules

Mathematical Foundation:
    The Normalizing Functor N transforms the category of Raw Entity Data
    into the category of Canonical Entity Data:

    N: RawEntity → CanonicalEntity

    Where N preserves identity (same entity always maps to same canonical form)
    and composition (N(L(x)) = N ∘ L(x) where L is the localization functor).
"""

from dataclasses import dataclass
from datetime import datetime
import re
import unicodedata

# =============================================================================
# UNIVERSAL EXPANSION MAPS
# =============================================================================

# Legal entity suffixes: ALWAYS expand to full form
LEGAL_SUFFIX_EXPANSIONS: dict[str, str] = {
    # Corporations
    "inc": "incorporated",
    "inc.": "incorporated",
    "incorp": "incorporated",
    "corp": "corporation",
    "corp.": "corporation",
    # Limited Liability
    "llc": "limited liability company",
    "l.l.c.": "limited liability company",
    "l.l.c": "limited liability company",
    "llp": "limited liability partnership",
    "l.l.p.": "limited liability partnership",
    "lp": "limited partnership",
    "l.p.": "limited partnership",
    # Limited
    "ltd": "limited",
    "ltd.": "limited",
    "ltda": "limitada",  # Spanish/Portuguese
    "ltée": "limitee",  # French (will be ASCII-ified)
    # Professional
    "pc": "professional corporation",
    "p.c.": "professional corporation",
    "pllc": "professional limited liability company",
    "p.l.l.c.": "professional limited liability company",
    "pa": "professional association",
    "p.a.": "professional association",
    # Company
    "co": "company",
    "co.": "company",
    "cos": "companies",
    # Partnership
    "gp": "general partnership",
    "g.p.": "general partnership",
    # Other
    "plc": "public limited company",
    "p.l.c.": "public limited company",
    "sa": "sociedad anonima",  # Spanish
    "s.a.": "sociedad anonima",
    "ag": "aktiengesellschaft",  # German
    "gmbh": "gesellschaft mit beschrankter haftung",  # German
    "bv": "besloten vennootschap",  # Dutch
    "b.v.": "besloten vennootschap",
    "nv": "naamloze vennootschap",  # Dutch
    "n.v.": "naamloze vennootschap",
    "pty": "proprietary",  # Australian
    "pty.": "proprietary",
}

# Common abbreviations: ALWAYS expand
COMMON_ABBREVIATIONS: dict[str, str] = {
    # Organizational
    "assn": "association",
    "assoc": "association",
    "dept": "department",
    "div": "division",
    "grp": "group",
    "org": "organization",
    "inst": "institute",
    "ctr": "center",
    "ctre": "centre",  # British spelling
    "comm": "commission",
    "auth": "authority",
    "admin": "administration",
    "svcs": "services",
    "svc": "service",
    "mgmt": "management",
    "mgt": "management",
    # Geographic
    "natl": "national",
    "intl": "international",
    "regl": "regional",
    "govt": "government",
    "fed": "federal",
    "muni": "municipal",
    "metro": "metropolitan",
    # Educational
    "univ": "university",
    "coll": "college",
    "acad": "academy",
    "sch": "school",
    "elem": "elementary",
    "dist": "district",
    "usd": "unified school district",
    "isd": "independent school district",
    # Geographic features
    "st": "saint",
    "ste": "sainte",
    "mt": "mount",
    "ft": "fort",
    "pt": "point",
    "cty": "county",
    "twp": "township",
    "vlg": "village",
    "boro": "borough",
    # Directions
    "n": "north",
    "s": "south",
    "e": "east",
    "w": "west",
    "ne": "northeast",
    "nw": "northwest",
    "se": "southeast",
    "sw": "southwest",
    # Business
    "mfg": "manufacturing",
    "mfr": "manufacturer",
    "bros": "brothers",
    "sys": "systems",
    "tech": "technology",
    "ind": "industries",
    "inds": "industries",
    "ent": "enterprises",
    "hldgs": "holdings",
    "props": "properties",
    "invs": "investments",
    "inv": "investment",
    "fin": "financial",
    "ins": "insurance",
    "med": "medical",
    "hlth": "health",
    "pharm": "pharmaceutical",
    "bio": "biological",
    "chem": "chemical",
    "elec": "electric",
    "util": "utilities",
    # Junior/Senior (for schools, orgs)
    "jr": "junior",
    "sr": "senior",
}

# Stop words to remove (after normalization)
STOP_WORDS: set[str] = {
    "the",
    "of",
    "a",
    "an",
    "and",
    "for",
    "in",
    "on",
    "at",
    "to",
    "by",
}

# Entity type indicators (helps with classification, not removed)
ENTITY_TYPE_INDICATORS: set[str] = {
    "corporation",
    "incorporated",
    "company",
    "limited",
    "partnership",
    "association",
    "foundation",
    "trust",
    "fund",
    "institute",
    "society",
    "authority",
    "district",
    "agency",
    "department",
    "commission",
    "board",
    "council",
    "committee",
}


# =============================================================================
# NORMALIZATION PIPELINE
# =============================================================================


def _to_ascii(text: str) -> str:
    """Convert Unicode to ASCII equivalent.

    Handles accented characters, special quotes, etc.
    """
    # Normalize to NFD (decomposed form)
    normalized = unicodedata.normalize("NFD", text)
    # Remove combining characters (accents)
    ascii_text = "".join(c for c in normalized if unicodedata.category(c) != "Mn")
    # Handle special characters that don't decompose
    replacements = {
        "æ": "ae",
        "œ": "oe",
        "ø": "o",
        "ß": "ss",
        "ð": "d",
        "þ": "th",
        "’": "",  # curly apostrophe
        "'": "",
        """: "",
        """: "",
        "–": "-",  # en-dash
        "—": "-",  # em-dash
        "…": "...",
    }
    for old, new in replacements.items():
        ascii_text = ascii_text.replace(old, new)
    # Final ASCII encoding
    return ascii_text.encode("ascii", "ignore").decode("ascii")


def _remove_punctuation(text: str) -> str:
    """Remove all punctuation from text.

    Per spec: Remove commas, periods, apostrophes, hyphens, etc.
    Only alphanumeric and spaces remain.
    """
    result = []
    for c in text:
        if c.isalnum() or c.isspace():
            result.append(c)
        else:
            result.append(" ")  # Replace with space to maintain word boundaries
    return "".join(result)


def _collapse_whitespace(text: str) -> str:
    """Collapse multiple spaces to single space, trim ends."""
    return " ".join(text.split())


def _expand_token(token: str) -> str:
    """Expand a single token if it matches known abbreviations.

    Order of precedence:
    1. Legal suffixes (highest priority)
    2. Common abbreviations
    """
    lower = token.lower()

    # Check legal suffixes first
    if lower in LEGAL_SUFFIX_EXPANSIONS:
        return LEGAL_SUFFIX_EXPANSIONS[lower]

    # Check common abbreviations
    if lower in COMMON_ABBREVIATIONS:
        return COMMON_ABBREVIATIONS[lower]

    return lower


def _expand_abbreviations(text: str) -> str:
    """Expand all abbreviations in the text."""
    tokens = text.split()
    expanded = [_expand_token(t) for t in tokens]
    return " ".join(expanded)


def _remove_stop_words(text: str, preserve_initial: bool = True) -> str:
    """Remove stop words from text.

    Args:
        text: Input text (should be lowercase).
        preserve_initial: If True, don't remove stop word if it's the first word.
    """
    tokens = text.split()
    if not tokens:
        return ""

    result = []
    for i, token in enumerate(tokens):
        if token in STOP_WORDS:
            # Preserve if first word and flag is set
            if i == 0 and preserve_initial:
                result.append(token)
            # Otherwise skip
        else:
            result.append(token)

    return " ".join(result)


def normalize_legal_name(
    name: str,
    remove_stop_words: bool = True,
    preserve_initial_stop: bool = False,
) -> str:
    """Apply the universal normalization pipeline to a legal name.

    Pipeline (in order):
    1. Convert to lowercase
    2. ASCII transliteration
    3. Remove punctuation
    4. Collapse whitespace
    5. Expand abbreviations
    6. Remove stop words (optional)
    7. Final trim

    Args:
        name: Raw legal name from source system.
        remove_stop_words: Whether to filter out stop words.
        preserve_initial_stop: If True, preserve stop word at start of name.

    Returns:
        Normalized name suitable for SNFEI hashing.

    Example:
        >>> normalize_legal_name("The Springfield Unified Sch. Dist., Inc.")
        "springfield unified school district incorporated"
    """
    if not name:
        return ""

    # 1. Lowercase
    text = name.lower()

    # 2. ASCII transliteration
    text = _to_ascii(text)

    # 3. Remove punctuation
    text = _remove_punctuation(text)

    # 4. Collapse whitespace
    text = _collapse_whitespace(text)

    # 5. Expand abbreviations
    text = _expand_abbreviations(text)

    # 6. Remove stop words
    if remove_stop_words:
        text = _remove_stop_words(text, preserve_initial=preserve_initial_stop)

    # 7. Final collapse and trim
    return _collapse_whitespace(text)


# =============================================================================
# ADDRESS NORMALIZATION
# =============================================================================

# US Postal abbreviations (USPS standard)
US_ADDRESS_EXPANSIONS: dict[str, str] = {
    # Street types
    "st": "street",
    "st.": "street",
    "ave": "avenue",
    "ave.": "avenue",
    "blvd": "boulevard",
    "blvd.": "boulevard",
    "dr": "drive",
    "dr.": "drive",
    "rd": "road",
    "rd.": "road",
    "ln": "lane",
    "ln.": "lane",
    "ct": "court",
    "ct.": "court",
    "cir": "circle",
    "cir.": "circle",
    "pl": "place",
    "pl.": "place",
    "sq": "square",
    "sq.": "square",
    "pkwy": "parkway",
    "hwy": "highway",
    "trl": "trail",
    "way": "way",
    "ter": "terrace",
    "ter.": "terrace",
    # Directionals
    "n": "north",
    "n.": "north",
    "s": "south",
    "s.": "south",
    "e": "east",
    "e.": "east",
    "w": "west",
    "w.": "west",
    "ne": "northeast",
    "nw": "northwest",
    "se": "southeast",
    "sw": "southwest",
}

# Secondary unit designators to REMOVE (apartment, suite, etc.)
SECONDARY_UNIT_PATTERNS = [
    r"\bapt\.?\s*#?\s*\w+",
    r"\bsuite\s*#?\s*\w+",
    r"\bste\.?\s*#?\s*\w+",
    r"\bunit\s*#?\s*\w+",
    r"\b#\s*\d+\w*",
    r"\bfloor\s*\d+",
    r"\bfl\.?\s*\d+",
    r"\broom\s*\d+",
    r"\brm\.?\s*\d+",
    r"\bbldg\.?\s*\w+",
    r"\bbuilding\s*\w+",
]


def normalize_address(
    address: str,
    remove_secondary: bool = True,
) -> str:
    """Normalize a street address for SNFEI hashing.

    Pipeline:
    1. Lowercase
    2. ASCII transliteration
    3. Remove secondary unit designators (apt, suite, etc.)
    4. Remove punctuation
    5. Expand postal abbreviations
    6. Collapse whitespace

    Args:
        address: Raw street address.
        remove_secondary: Whether to remove apartment/suite numbers.

    Returns:
        Normalized address string.

    Example:
        >>> normalize_address("123 N. Main St., Suite 400")
        "123 north main street"
    """
    if not address:
        return ""

    # 1. Lowercase
    text = address.lower()

    # 2. ASCII transliteration
    text = _to_ascii(text)

    # 3. Remove secondary unit designators
    if remove_secondary:
        for pattern in SECONDARY_UNIT_PATTERNS:
            text = re.sub(pattern, "", text, flags=re.IGNORECASE)

    # 4. Remove punctuation
    text = _remove_punctuation(text)

    # 5. Collapse whitespace first
    text = _collapse_whitespace(text)

    # 6. Expand postal abbreviations
    tokens = text.split()
    expanded = []
    for token in tokens:
        if token in US_ADDRESS_EXPANSIONS:
            expanded.append(US_ADDRESS_EXPANSIONS[token])
        else:
            expanded.append(token)
    text = " ".join(expanded)

    # 7. Final trim
    return text.strip()


# =============================================================================
# REGISTRATION DATE NORMALIZATION
# =============================================================================


def normalize_registration_date(date_str: str) -> str | None:
    """Normalize a registration date to ISO 8601 format.

    Returns None if date cannot be parsed.

    Args:
        date_str: Date string in various formats.

    Returns:
        ISO 8601 date string (YYYY-MM-DD) or None.
    """
    if not date_str:
        return None

    # Remove extra whitespace
    date_str = date_str.strip()

    # Try common date patterns

    patterns = [
        # ISO format
        (r"^(\d{4})-(\d{2})-(\d{2})$", "%Y-%m-%d"),
        # US format
        (r"^(\d{1,2})/(\d{1,2})/(\d{4})$", "%m/%d/%Y"),
        (r"^(\d{1,2})-(\d{1,2})-(\d{4})$", "%m-%d-%Y"),
        # European format
        (r"^(\d{1,2})/(\d{1,2})/(\d{4})$", "%d/%m/%Y"),
        # Year only
        (r"^(\d{4})$", "%Y"),
    ]

    for pattern, fmt in patterns:
        if re.match(pattern, date_str):
            try:
                if fmt == "%Y":
                    # Year only - use January 1
                    return f"{date_str}-01-01"
                dt = datetime.strptime(date_str, fmt)
                return dt.strftime("%Y-%m-%d")
            except ValueError:
                continue

    return None


# =============================================================================
# CANONICAL INPUT BUILDER
# =============================================================================


@dataclass
class CanonicalInput:
    """Normalized input for SNFEI hashing."""

    legal_name_normalized: str
    address_normalized: str | None
    country_code: str
    registration_date: str | None

    def to_hash_string(self) -> str:
        """Generate the concatenated string for hashing.

        Format:
            legal_name_normalized|address_normalized|country_code|registration_date

        Empty/None fields are included as empty strings to maintain
        consistent field positions.
        """
        parts = [
            self.legal_name_normalized,
            self.address_normalized or "",
            self.country_code,
            self.registration_date or "",
        ]
        return "|".join(parts)

    def to_hash_string_v2(self) -> str:
        """Alternative format that omits empty fields.

        This produces shorter strings but requires all implementations
        to handle optional fields identically.
        """
        parts = [self.legal_name_normalized]
        if self.address_normalized:
            parts.append(self.address_normalized)
        parts.append(self.country_code)
        if self.registration_date:
            parts.append(self.registration_date)
        return "|".join(parts)


def build_canonical_input(
    legal_name: str,
    country_code: str,
    address: str | None = None,
    registration_date: str | None = None,
) -> CanonicalInput:
    """Build a canonical input structure from raw entity data.

    Args:
        legal_name: Raw legal name.
        country_code: ISO 3166-1 alpha-2 country code.
        address: Optional street address.
        registration_date: Optional registration/formation date.

    Returns:
        CanonicalInput with all fields normalized.
    """
    return CanonicalInput(
        legal_name_normalized=normalize_legal_name(legal_name),
        address_normalized=normalize_address(address) if address else None,
        country_code=country_code.upper(),
        registration_date=normalize_registration_date(registration_date)
        if registration_date
        else None,
    )
