"""Central schema registry for CEP validation."""

from functools import lru_cache
import json
from pathlib import Path

from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012

from .version import SCHEMA_VERSION


# Extract major.minor for schema lookup (drop patch)
def _schema_version() -> str:
    """Get major.minor version for schema lookup."""
    parts = SCHEMA_VERSION.split(".")
    return f"{parts[0]}.{parts[1]}"


# Map (name, version) to relative paths from repo root
SCHEMA_CATALOG = {
    ("entity", "1.0"): "schemas/cep.entity.schema.json",
    ("exchange", "1.0"): "schemas/cep.exchange.schema.json",
    ("relationship", "1.0"): "schemas/cep.relationship.schema.json",
    ("snfei", "1.0"): "test_vectors/schemas/v1.0/generation-vector-set.schema.json",
}


def _find_repo_root() -> Path:
    """Walk up from current file to find repository root."""
    current = Path(__file__).resolve().parent
    for parent in [current, *current.parents]:
        if (parent / "pyproject.toml").exists():
            return parent
    raise RuntimeError("Could not find repository root")


@lru_cache(maxsize=1)
def _build_global_registry() -> tuple[Registry, dict]:
    """Build registry containing all CEP schemas. Cached on first call."""
    repo_root = _find_repo_root()
    schemas = {}
    resources = []

    for (name, version), rel_path in SCHEMA_CATALOG.items():
        schema_path = repo_root / rel_path
        if not schema_path.is_file():
            continue

        with schema_path.open("r", encoding="utf-8") as f:
            schema = json.load(f)

        schema_id = schema.get("$id", f"urn:cep:{name}:{version}")
        schemas[(name, version)] = schema
        resources.append(
            (schema_id, Resource.from_contents(schema, default_specification=DRAFT202012))
        )

    registry = Registry().with_resources(resources)
    return registry, schemas


def get_schema(name: str, version: str | None = None) -> dict:
    """Get a schema by logical name and optional version.

    Args:
        name: Schema name (e.g., 'entity', 'exchange').
        version: Schema version. Defaults to current SCHEMA_VERSION.

    Returns:
        Schema as a dict.

    Raises:
        ValueError: If schema name or version is unknown.
    """
    if version is None:
        version = _schema_version()

    _, schemas = _build_global_registry()
    key = (name, version)
    if key not in schemas:
        available = [f"{n} v{v}" for (n, v) in schemas if n == name]
        if available:
            raise ValueError(
                f"Unknown version {version!r} for schema {name!r}. Available: {available}"
            )
        raise ValueError(f"Unknown schema: {name!r}")
    return schemas[key]


def get_registry() -> Registry:
    """Get the shared registry for validation."""
    registry, _ = _build_global_registry()
    return registry


def list_schemas() -> list[tuple[str, str]]:
    """List all available (name, version) pairs."""
    return list(SCHEMA_CATALOG.keys())
