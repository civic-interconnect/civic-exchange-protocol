# tools/sync_schemas_for_docs.py
"""Utility script to synchronize JSON schema files to the documentation directory.

This module copies all JSON schema files from the schemas/ directory to the
docs/schemas/ directory for inclusion in the project documentation.
"""

from pathlib import Path
import shutil


def main() -> None:
    """Copy JSON schema files from schemas/ to docs/schemas/ directory."""
    root = Path(__file__).resolve().parent.parent  # repo root
    src_dir = root / "schemas"
    dst_dir = root / "docs" / "schemas"

    if not src_dir.exists():
        raise SystemExit(f"Source directory not found: {src_dir}")

    dst_dir.mkdir(parents=True, exist_ok=True)

    for schema_file in src_dir.glob("*.json"):
        target = dst_dir / schema_file.name
        shutil.copy2(schema_file, target)

    print(f"Copied schemas from {src_dir} to {dst_dir}")


if __name__ == "__main__":
    main()
