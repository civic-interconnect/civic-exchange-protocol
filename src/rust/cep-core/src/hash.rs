/// Cryptographic hashing utilities for CEP records.
///
/// All CEP hashes are SHA-256, represented as lowercase hexadecimal strings.

use sha2::{Digest, Sha256};
use std::fmt;

/// A SHA-256 hash value represented as a 64-character lowercase hex string.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanonicalHash(String);

impl CanonicalHash {
    /// Computes the SHA-256 hash of the given canonical string.
    pub fn from_canonical_string(canonical: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        let result = hasher.finalize();
        Self(format!("{:x}", result))
    }

    /// Creates a CanonicalHash from a pre-computed hex string.
    ///
    /// Returns None if the string is not a valid 64-character hex string.
    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() == 64 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
            Some(Self(hex.to_lowercase()))
        } else {
            None
        }
    }

    /// Returns the hash as a lowercase hex string.
    pub fn as_hex(&self) -> &str {
        &self.0
    }

    /// Returns the hash as bytes (32 bytes).
    pub fn as_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];
        for (i, chunk) in self.0.as_bytes().chunks(2).enumerate() {
            let hex_str = std::str::from_utf8(chunk).unwrap();
            bytes[i] = u8::from_str_radix(hex_str, 16).unwrap();
        }
        bytes
    }
}

impl fmt::Display for CanonicalHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl serde::Serialize for CanonicalHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for CanonicalHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_hex(&s).ok_or_else(|| {
            serde::de::Error::custom("invalid hash: must be 64 lowercase hex characters")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_empty_string() {
        let hash = CanonicalHash::from_canonical_string("");
        assert_eq!(
            hash.as_hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_hash_hello() {
        let hash = CanonicalHash::from_canonical_string("hello");
        assert_eq!(
            hash.as_hex(),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_from_hex_valid() {
        let hex = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
        let hash = CanonicalHash::from_hex(hex).unwrap();
        assert_eq!(hash.as_hex(), hex);
    }

    #[test]
    fn test_from_hex_invalid_length() {
        assert!(CanonicalHash::from_hex("abc123").is_none());
    }

    #[test]
    fn test_from_hex_invalid_chars() {
        let invalid = "gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg";
        assert!(CanonicalHash::from_hex(invalid).is_none());
    }

    #[test]
    fn test_uppercase_normalized() {
        let hex = "2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824";
        let hash = CanonicalHash::from_hex(hex).unwrap();
        assert_eq!(
            hash.as_hex(),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }
}