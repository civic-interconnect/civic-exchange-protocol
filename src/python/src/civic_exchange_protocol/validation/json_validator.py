"""JSON validation helpers for the Civic Exchange Protocol.

This module is imported by the CLI, and contains domain-specific logic:
- where schemas live
- how schema names map to schema files
- how individual JSON documents are validated
"""

from dataclasses import dataclass
import json
from pathlib import Path

from jsonschema import Draft202012Validator
from jsonschema import ValidationError as JsonSchemaError

SCHEMA_MAP = {
    "entity": "schemas/cep.entity.schema.json",
    "exchange": "schemas/cep.exchange.schema.json",
    "relationship": "schemas/cep.relationship.schema.json",
    "snfei": "test_vectors/schemas/v1.0/generation-vector-set.schema.json",
}


@dataclass
class FileValidationResult:
    """Validation result for a single JSON file."""

    path: Path
    ok: bool
    errors: list[str]


@dataclass
class ValidationSummary:
    """Summary of validation across one or more files."""

    results: list[FileValidationResult]

    @property
    def ok(self) -> bool:
        """Return True if all files validated successfully."""
        return all(r.ok for r in self.results)


def _iter_json_files(path: Path, recursive: bool) -> list[Path]:
    """Return list of JSON files under a file or directory."""
    if path.is_file():
        return [path]

    if recursive:
        return [p for p in path.rglob("*.json") if p.is_file()]

    return [p for p in path.glob("*.json") if p.is_file()]


def _find_repo_root(start: Path | None = None) -> Path:
    """Find the repository root by walking up until pyproject.toml is found."""
    path = (start or Path(__file__)).resolve()
    for parent in [path] + list(path.parents):
        if (parent / "pyproject.toml").is_file():
            return parent
    raise RuntimeError("Could not find repository root (pyproject.toml not found).")


def _load_schema_from_repo_root(schema_name: str) -> dict:
    """Load a JSON Schema by logical name from the repository.

    This assumes CLI is usually run from the repository root.

    Args:
        schema_name: Logical schema name (for example: 'entity').

    Returns:
        Loaded JSON schema as a dict.
        Raises: ValueError if schema_name is unknown.
                FileNotFoundError if schema file is missing.
    """
    if schema_name not in SCHEMA_MAP:
        raise ValueError(f"Unknown schema name: {schema_name!r}. Update SCHEMA_MAP.")

    schema_rel_path = SCHEMA_MAP[schema_name]
    repo_root = _find_repo_root()
    schema_path = repo_root / schema_rel_path

    if not schema_path.is_file():
        raise FileNotFoundError(f"Schema file not found for {schema_name!r}: {schema_path}")

    with schema_path.open("r", encoding="utf-8") as f:
        return json.load(f)


def _format_schema_error(err: JsonSchemaError) -> str:
    """Format a jsonschema ValidationError with rich context.

    Includes:
    - human-readable message
    - instance path (where in the JSON document)
    - schema path (which part of the schema failed)
    """
    instance_parts = [str(p) for p in err.path]
    instance_path = ".".join(instance_parts) if instance_parts else "(root)"

    schema_parts = [str(p) for p in err.schema_path]
    schema_path = " -> ".join(schema_parts) if schema_parts else "(root)"

    return f"{err.message} (instance path: {instance_path}; schema path: {schema_path})"


def validate_json_path(
    path: Path,
    schema_name: str,
    recursive: bool = False,
) -> ValidationSummary:
    """Validate a file or directory of JSON files against a CEP schema.

    Args:
        path: Path to a JSON file or directory.
        schema_name: Logical schema name (for example: 'entity').
        recursive: If True and path is a directory, traverse subdirectories.

    Returns:
        ValidationSummary with per-file results.
    """
    schema = _load_schema_from_repo_root(schema_name)
    validator = Draft202012Validator(schema)

    json_files = _iter_json_files(path, recursive=recursive)
    results: list[FileValidationResult] = []

    for json_path in json_files:
        errors: list[str] = []

        # 1) Parse JSON with line/column-aware errors
        try:
            with json_path.open("r", encoding="utf-8") as f:
                data = json.load(f)
        except json.JSONDecodeError as exc:
            errors.append(f"Invalid JSON at line {exc.lineno}, column {exc.colno}: {exc.msg}")
            results.append(FileValidationResult(path=json_path, ok=False, errors=errors))
            continue

        # 2) Schema validation errors
        for err in validator.iter_errors(data):
            errors.append(_format_schema_error(err))

        results.append(FileValidationResult(path=json_path, ok=not errors, errors=errors))

    return ValidationSummary(results=results)
