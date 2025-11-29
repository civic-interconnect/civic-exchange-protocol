"""Command-line interface for the Civic Exchange Protocol.

This module provides CLI commands for:
- version: Display the package version
- validate: Validate exchange protocol data
"""

import typer

from civic_exchange_protocol.snfei.generator import generate_snfei_with_confidence

app = typer.Typer(help="Civic Exchange Protocol CLI")


@app.command()
def snfei(
    legal_name: str = typer.Argument(..., help="Raw legal name"),
    country_code: str = typer.Option("US", help="ISO country code"),
) -> None:
    """Generate an SNFEI for an entity name and country."""
    result = generate_snfei_with_confidence(
        legal_name=legal_name,
        country_code=country_code,
    )
    typer.echo(f"SNFEI: {result.snfei.value}")
    typer.echo(f"Tier: {result.tier}, confidence: {result.confidence_score}")


@app.command()
def version():
    """Show package version."""
    from civic_exchange_protocol import __version__

    typer.echo(__version__)


@app.command()
def validate():
    """Validate exchange protocol data (placeholder)."""
    typer.echo("Validator coming soon.")
