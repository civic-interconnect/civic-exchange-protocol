"""Command-line interface for the Civic Exchange Protocol.

This module provides CLI commands for:
- version: Display the package version
- validate: Validate exchange protocol data
"""

from typer import Typer

app = Typer(help="Civic Exchange Protocol CLI")


@app.command()
def version():
    """Show package version."""
    from civic_exchange_protocol import __version__

    print(__version__)


@app.command()
def validate():
    """Validate exchange protocol data."""
    print("Validator coming soon.")
