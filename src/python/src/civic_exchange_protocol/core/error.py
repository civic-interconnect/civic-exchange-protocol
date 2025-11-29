"""Error types for CEP operations."""


class CepError(Exception):
    """Base exception for CEP operations."""

    pass


class InvalidTimestampError(CepError):
    """Invalid timestamp format."""

    def __init__(self, message: str) -> None:
        """Initialize with error message about invalid timestamp."""
        super().__init__(f"invalid timestamp: {message}")


class InvalidHashError(CepError):
    """Invalid hash format."""

    def __init__(self, value: str) -> None:
        """Initialize with the invalid hash value."""
        super().__init__(f"invalid hash: expected 64 hex characters, got {value}")


class InvalidIdentifierError(CepError):
    """Invalid identifier format."""

    def __init__(self, message: str) -> None:
        """Initialize with error message about invalid identifier."""
        super().__init__(f"invalid identifier: {message}")


class MissingFieldError(CepError):
    """Missing required field."""

    def __init__(self, field: str) -> None:
        """Initialize with the name of the missing field."""
        super().__init__(f"missing required field: {field}")


class UnsupportedVersionError(CepError):
    """Schema version mismatch."""

    def __init__(self, version: str) -> None:
        """Initialize with the unsupported version string."""
        super().__init__(f"unsupported schema version: {version}")


class HashMismatchError(CepError):
    """Hash verification failed."""

    def __init__(self, expected: str, actual: str) -> None:
        """Initialize with expected and actual hash values."""
        super().__init__(f"hash verification failed: expected {expected}, got {actual}")
        self.expected = expected
        self.actual = actual


class RevisionChainError(CepError):
    """Revision chain error."""

    def __init__(self, message: str) -> None:
        """Initialize with error message about revision chain."""
        super().__init__(f"revision chain error: {message}")
