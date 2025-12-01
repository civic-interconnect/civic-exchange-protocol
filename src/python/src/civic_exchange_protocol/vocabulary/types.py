"""Vocabulary types for Civic Exchange Protocol (CEP).

Typed model for CEP controlled vocabularies.

This module mirrors the structure of `cep.vocabulary.schema.json` and provides:
- Vocabulary
- VocabularyTerm
- VocabularyMapping

It also includes helpers to load from JSON and to perform basic sanity checks
(code uniqueness, termUri prefix, etc.).
"""

from dataclasses import dataclass, field
import json
from pathlib import Path
import re
from typing import Any, Literal

__all__ = [
    "Vocabulary",
    "VocabularyTerm",
    "VocabularyMapping",
]

TermStatus = Literal["active", "deprecated", "experimental"]
MappingType = Literal["exactMatch", "broadMatch", "narrowMatch", "relatedMatch"]

SEMVER_REGEX = re.compile(r"^[0-9]+\.[0-9]+\.[0-9]+$")


@dataclass
class VocabularyTerm:
    """A single controlled term in a CEP vocabulary."""

    term_uri: str
    code: str
    label: str
    definition: str
    status: TermStatus
    added_in_version: str
    parent_term_uri: str | None = None
    see_also: list[str] = field(default_factory=list)
    deprecation_note: str | None = None

    @staticmethod
    def from_dict(data: dict[str, Any]) -> "VocabularyTerm":
        """Create a VocabularyTerm from a JSON dictionary."""
        return VocabularyTerm(
            term_uri=data["termUri"],
            code=data["code"],
            label=data["label"],
            definition=data["definition"],
            status=data.get("status", "active"),
            added_in_version=data["addedInVersion"],
            parent_term_uri=data.get("parentTermUri"),
            see_also=list(data.get("seeAlso", [])),
            deprecation_note=data.get("deprecationNote"),
        )

    def to_dict(self) -> dict[str, Any]:
        """Convert this term back to a JSON-serializable dictionary."""
        data: dict[str, Any] = {
            "termUri": self.term_uri,
            "code": self.code,
            "label": self.label,
            "definition": self.definition,
            "status": self.status,
            "addedInVersion": self.added_in_version,
        }
        # Optional fields only if present
        if self.parent_term_uri is not None:
            data["parentTermUri"] = self.parent_term_uri
        if self.see_also:
            data["seeAlso"] = list(self.see_also)
        if self.deprecation_note is not None:
            data["deprecationNote"] = self.deprecation_note
        return data


@dataclass
class VocabularyMapping:
    """Mapping from a CEP term to an external standard term."""

    term_uri: str
    external_uri: str
    mapping_type: MappingType
    external_standard: str

    @staticmethod
    def from_dict(data: dict[str, Any]) -> "VocabularyMapping":
        """Create a VocabularyMapping from a JSON dictionary."""
        return VocabularyMapping(
            term_uri=data["termUri"],
            external_uri=data["externalUri"],
            mapping_type=data["mappingType"],
            external_standard=data["externalStandard"],
        )

    def to_dict(self) -> dict[str, Any]:
        """Convert this mapping back to a JSON-serializable dictionary."""
        return {
            "termUri": self.term_uri,
            "externalUri": self.external_uri,
            "mappingType": self.mapping_type,
            "externalStandard": self.external_standard,
        }


@dataclass
class Vocabulary:
    """CEP controlled vocabulary container.

    This corresponds to the root object defined in `cep.vocabulary.schema.json`.
    """

    vocabulary_uri: str
    version: str
    title: str
    effective_date: str
    description: str | None = None
    governance_uri: str | None = None
    deprecates_version: str | None = None
    terms: list[VocabularyTerm] = field(default_factory=list)
    mappings: list[VocabularyMapping] = field(default_factory=list)

    # ---------------------------------------------------------------------
    # Construction helpers
    # ---------------------------------------------------------------------

    @staticmethod
    def from_dict(data: dict[str, Any]) -> "Vocabulary":
        """Create a Vocabulary from a JSON dictionary."""
        terms_data = data.get("terms", [])
        mappings_data = data.get("mappings", [])

        return Vocabulary(
            vocabulary_uri=data["vocabularyUri"],
            version=data["version"],
            title=data["title"],
            effective_date=data["effectiveDate"],
            description=data.get("description"),
            governance_uri=data.get("governanceUri"),
            deprecates_version=data.get("deprecatesVersion"),
            terms=[VocabularyTerm.from_dict(t) for t in terms_data],
            mappings=[VocabularyMapping.from_dict(m) for m in mappings_data],
        )

    @staticmethod
    def load(path: Path) -> "Vocabulary":
        """Load a Vocabulary from a JSON file."""
        text = path.read_text(encoding="utf-8")
        data = json.loads(text)
        return Vocabulary.from_dict(data)

    # ---------------------------------------------------------------------
    # Serialization
    # ---------------------------------------------------------------------

    def to_dict(self) -> dict[str, Any]:
        """Convert this vocabulary to a JSON-serializable dictionary."""
        data: dict[str, Any] = {
            "vocabularyUri": self.vocabulary_uri,
            "version": self.version,
            "title": self.title,
            "effectiveDate": self.effective_date,
            "terms": [t.to_dict() for t in self.terms],
        }
        if self.description is not None:
            data["description"] = self.description
        if self.governance_uri is not None:
            data["governanceUri"] = self.governance_uri
        if self.deprecates_version is not None:
            data["deprecatesVersion"] = self.deprecates_version
        if self.mappings:
            data["mappings"] = [m.to_dict() for m in self.mappings]
        return data

    # ---------------------------------------------------------------------
    # Basic validation and lookup helpers
    # ---------------------------------------------------------------------

    def validate_basic(self) -> None:
        """Run basic sanity checks consistent with the schema intent.

        This is *not* a full JSON Schema validator, but it catches common issues:
        - version is a SemVer string
        - term codes are unique
        - termUris use the vocabularyUri as prefix
        """
        if not SEMVER_REGEX.match(self.version):
            raise ValueError(f"Invalid vocabulary version '{self.version}' (expected SemVer).")

        # Unique codes and termUris
        codes: dict[str, VocabularyTerm] = {}
        term_uris: dict[str, VocabularyTerm] = {}

        for term in self.terms:
            # Code uniqueness
            if term.code in codes:
                raise ValueError(
                    f"Duplicate term code '{term.code}' in vocabulary '{self.vocabulary_uri}'."
                )
            codes[term.code] = term

            # termUri uniqueness
            if term.term_uri in term_uris:
                raise ValueError(
                    f"Duplicate termUri '{term.term_uri}' in vocabulary '{self.vocabulary_uri}'."
                )
            term_uris[term.term_uri] = term

            # Optional: ensure termUri starts with vocabulary_uri
            if not term.term_uri.startswith(self.vocabulary_uri):
                # Not strictly required by schema, but strongly recommended.
                raise ValueError(
                    f"termUri '{term.term_uri}' does not start with vocabularyUri "
                    f"'{self.vocabulary_uri}'."
                )

    def get_term_by_code(self, code: str) -> VocabularyTerm | None:
        """Return the term with the given code, if present."""
        for term in self.terms:
            if term.code == code:
                return term
        return None

    def get_term_by_uri(self, term_uri: str) -> VocabularyTerm | None:
        """Return the term with the given URI, if present."""
        for term in self.terms:
            if term.term_uri == term_uri:
                return term
        return None

    def list_active_codes(self) -> list[str]:
        """Return a list of codes for active terms only."""
        return [t.code for t in self.terms if t.status == "active"]
