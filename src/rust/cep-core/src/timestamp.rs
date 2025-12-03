/// Canonical timestamp handling for CEP records.
///
/// All CEP timestamps MUST be:
/// - UTC timezone (indicated by 'Z' suffix)
/// - ISO 8601 format
/// - Microsecond precision (exactly 6 decimal places)
///
/// Example: `2025-11-28T14:30:00.000000Z`

use chrono::{DateTime, Utc};
use std::fmt;
use std::str::FromStr;

/// A canonical CEP timestamp with mandatory microsecond precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CanonicalTimestamp(DateTime<Utc>);

impl CanonicalTimestamp {
    /// Creates a new CanonicalTimestamp from a chrono DateTime<Utc>.
    pub fn new(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }

    /// Returns the current UTC time as a CanonicalTimestamp.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Returns the underlying DateTime<Utc>.
    pub fn as_datetime(&self) -> DateTime<Utc> {
        self.0
    }

    /// Returns the canonical string representation.
    ///
    /// Format: `YYYY-MM-DDTHH:MM:SS.ffffffZ`
    ///
    /// This format is REQUIRED for hash stability across all CEP implementations.
    pub fn to_canonical_string(&self) -> String {
        self.0.format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string()
    }
}

impl fmt::Display for CanonicalTimestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_canonical_string())
    }
}

impl FromStr for CanonicalTimestamp {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Accept RFC 3339 format and convert to our canonical form
        let dt = DateTime::parse_from_rfc3339(s)?;
        Ok(Self(dt.with_timezone(&Utc)))
    }
}

impl From<DateTime<Utc>> for CanonicalTimestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Self(dt)
    }
}

impl serde::Serialize for CanonicalTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_canonical_string())
    }
}

impl<'de> serde::Deserialize<'de> for CanonicalTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_format() {
        let ts: CanonicalTimestamp = "2025-11-28T14:30:00.123456Z".parse().unwrap();
        assert_eq!(ts.to_canonical_string(), "2025-11-28T14:30:00.123456Z");
    }

    #[test]
    fn test_zero_microseconds() {
        let ts: CanonicalTimestamp = "2025-11-28T14:30:00.000000Z".parse().unwrap();
        // CRITICAL: Must preserve all 6 decimal places even when zero
        assert_eq!(ts.to_canonical_string(), "2025-11-28T14:30:00.000000Z");
    }

    #[test]
    fn test_parse_with_offset() {
        // Should accept offset format and convert to Z
        let ts: CanonicalTimestamp = "2025-11-28T14:30:00.123456+00:00".parse().unwrap();
        assert_eq!(ts.to_canonical_string(), "2025-11-28T14:30:00.123456Z");
    }

    #[test]
    fn test_ordering() {
        let earlier: CanonicalTimestamp = "2025-11-28T14:30:00.000000Z".parse().unwrap();
        let later: CanonicalTimestamp = "2025-11-28T14:30:00.000001Z".parse().unwrap();
        assert!(earlier < later);
    }
}