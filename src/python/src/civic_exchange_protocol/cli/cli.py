"""Command-line interface for the Civic Exchange Protocol.

This module provides CLI commands for:
- snfei: Generate an SNFEI for an entity name and country
- version: Display the package version
- validate-json: Validate JSON files against CEP schemas
"""

from pathlib import Path

import typer

from civic_exchange_protocol.snfei.generator import generate_snfei_with_confidence
from civic_exchange_protocol.validation.json_validator import (
    ValidationSummary,
    validate_json_path,
)

app = typer.Typer(help="Civic Exchange Protocol CLI")


@app.command()
def snfei(
    legal_name: str = typer.Argument(..., help="Raw legal name"),
    country_code: str = typer.Option("US", "--country-code", "-c", help="ISO country code"),
) -> None:
    """Generate an SNFEI for an entity name and country."""
    result = generate_snfei_with_confidence(
        legal_name=legal_name,
        country_code=country_code,
    )
    typer.echo(f"SNFEI: {result.snfei.value}")
    typer.echo(f"Tier: {result.tier}, confidence: {result.confidence_score}")


@app.command()
def version() -> None:
    """Show package version."""
    from civic_exchange_protocol import __version__

    typer.echo(__version__)


@app.command()
def validate_json(
    path: Path | None = None,
    schema: str = typer.Option(
        ...,
        "--schema",
        "-s",
        help="Schema name (for example: entity, exchange, relationship, snfei).",
    ),
    recursive: bool = typer.Option(
        False,
        "--recursive",
        "-r",
        help="Recurse into subdirectories when validating a directory.",
    ),
) -> None:
    """Validate JSON file(s) against a CEP JSON Schema.

    Behavior:
    - If PATH is a file, validates that single JSON file.
    - If PATH is a directory, validates all *.json files within it.
      Use --recursive to walk subdirectories.
    """
    if path is None:
        typer.echo("Error: Path argument is required.")
        raise typer.Exit(code=1)

    summary: ValidationSummary = validate_json_path(
        path=path,
        schema_name=schema,
        recursive=recursive,
    )

    if not summary.results:
        typer.echo("No JSON files found to validate.")
        raise typer.Exit(code=1)

    errors_found = False

    for result in summary.results:
        if result.ok:
            typer.echo(f"[OK] {result.path}")
        else:
            errors_found = True
            typer.echo(f"[ERROR] {result.path}")
            for err in result.errors:
                typer.echo(f"  - {err}")

    if errors_found:
        typer.echo("Validation completed with errors.")
        raise typer.Exit(code=1)

    typer.echo("All files validated successfully.")
    raise typer.Exit(code=0)
