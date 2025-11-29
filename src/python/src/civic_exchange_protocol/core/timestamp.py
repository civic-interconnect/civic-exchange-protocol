"""Canonical timestamp handling for CEP records.

All CEP timestamps MUST be:
- UTC timezone (indicated by 'Z' suffix)
- ISO 8601 format
- Microsecond precision (exactly 6 decimal places)

Example: 2025-11-28T14:30:00.000000Z
"""

from datetime import UTC, datetime

# The canonical format string - EXACTLY 6 decimal places, Z suffix
CANONICAL_FORMAT = "%Y-%m-%dT%H:%M:%S.%fZ"


class CanonicalTimestamp:
    """A canonical CEP timestamp with mandatory microsecond precision."""

    __slots__ = ("_dt",)

    def __init__(self, dt: datetime) -> None:
        """Create a new CanonicalTimestamp from a datetime.

        Args:
            dt: A datetime object. If naive, assumed to be UTC.
                If aware, will be converted to UTC.
        """
        if dt.tzinfo is None:
            # Naive datetime - assume UTC
            self._dt = dt.replace(tzinfo=UTC)
        else:
            # Convert to UTC
            self._dt = dt.astimezone(UTC)

    @classmethod
    def now(cls) -> "CanonicalTimestamp":
        """Return the current UTC time as a CanonicalTimestamp."""
        return cls(datetime.now(UTC))

    @classmethod
    def parse(cls, s: str) -> "CanonicalTimestamp":
        """Parse an ISO 8601 timestamp string.

        Accepts formats:
        - 2025-11-28T14:30:00.123456Z
        - 2025-11-28T14:30:00.123456+00:00
        - 2025-11-28T14:30:00Z (will add .000000)

        Args:
            s: The timestamp string to parse.

        Returns:
            A CanonicalTimestamp instance.

        Raises:
            ValueError: If the string cannot be parsed.
        """
        # Handle Z suffix
        if s.endswith("Z"):
            s = s[:-1] + "+00:00"

        # Try parsing with microseconds
        try:
            dt = datetime.fromisoformat(s)
            return cls(dt)
        except ValueError:
            pass

        # Try without microseconds and add them
        try:
            # Remove timezone for parsing, then add back
            if "+" in s:
                base, tz = s.rsplit("+", 1)
                dt = datetime.fromisoformat(base)
                dt = dt.replace(tzinfo=UTC)
                return cls(dt)
            if s.count("-") > 2:  # Has negative offset
                base, tz = s.rsplit("-", 1)
                dt = datetime.fromisoformat(base)
                dt = dt.replace(tzinfo=UTC)
                return cls(dt)
        except ValueError:
            pass

        raise ValueError(f"Cannot parse timestamp: {s}")

    def as_datetime(self) -> datetime:
        """Return the underlying datetime object (UTC)."""
        return self._dt

    def to_canonical_string(self) -> str:
        """Return the canonical string representation.

        Format: YYYY-MM-DDTHH:MM:SS.ffffffZ

        This format is REQUIRED for hash stability across all CEP implementations.
        """
        return self._dt.strftime(CANONICAL_FORMAT)

    def __str__(self) -> str:
        """Return the canonical string representation of the timestamp."""
        return self.to_canonical_string()

    def __repr__(self) -> str:
        """Return the developer-friendly representation of the timestamp."""
        return f"CanonicalTimestamp({self.to_canonical_string()!r})"

    def __eq__(self, other: object) -> bool:
        """Check equality with another CanonicalTimestamp."""
        if isinstance(other, CanonicalTimestamp):
            return self._dt == other._dt
        return NotImplemented

    def __lt__(self, other: "CanonicalTimestamp") -> bool:
        """Check if this timestamp is less than another."""
        if isinstance(other, CanonicalTimestamp):
            return self._dt < other._dt
        return NotImplemented

    def __le__(self, other: "CanonicalTimestamp") -> bool:
        """Check if this timestamp is less than or equal to another."""
        if isinstance(other, CanonicalTimestamp):
            return self._dt <= other._dt
        return NotImplemented

    def __gt__(self, other: "CanonicalTimestamp") -> bool:
        """Check if this timestamp is greater than another."""
        if isinstance(other, CanonicalTimestamp):
            return self._dt > other._dt
        return NotImplemented

    def __ge__(self, other: "CanonicalTimestamp") -> bool:
        """Check if this timestamp is greater than or equal to another."""
        if isinstance(other, CanonicalTimestamp):
            return self._dt >= other._dt
        return NotImplemented

    def __hash__(self) -> int:
        """Return the hash of the timestamp."""
        return hash(self._dt)
