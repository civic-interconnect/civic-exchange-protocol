/// Version information for CEP schemas.

/// Current schema version (major.minor.patch).
pub const SCHEMA_VERSION: &str = "1.0.0";

/// Get the major version number.
pub fn major_version() -> u32 {
    SCHEMA_VERSION
        .split('.')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
}

/// Get the minor version number.
pub fn minor_version() -> u32 {
    SCHEMA_VERSION
        .split('.')
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

/// Get the patch version number.
pub fn patch_version() -> u32 {
    SCHEMA_VERSION
        .split('.')
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_parsing() {
        assert_eq!(major_version(), 1);
        assert_eq!(minor_version(), 0);
        assert_eq!(patch_version(), 0);
    }
}