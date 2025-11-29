//! Entity identifier types for CEP.

use cep_core::canonical::{insert_if_present, Canonicalize};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// SAM.gov Unique Entity Identifier (12 alphanumeric characters).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SamUei(String);

impl SamUei {
    pub fn new(value: &str) -> Option<Self> {
        if value.len() == 12 && value.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
            Some(Self(value.to_string()))
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Legal Entity Identifier per ISO 17442 (20 alphanumeric characters).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Lei(String);

impl Lei {
    pub fn new(value: &str) -> Option<Self> {
        if value.len() == 20 && value.chars().all(|c| c.is_ascii_alphanumeric()) {
            Some(Self(value.to_uppercase()))
        } else {
            None
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Sub-National Federated Entity Identifier (SHA-256 hash, 64 hex chars).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Snfei(String);

impl Snfei {
    pub fn from_hash(hash: &str) -> Option<Self> {
        if hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
            Some(Self(hash.to_lowercase()))
        } else {
            None
        }
    }

    pub fn generate(normalized_name: &str, jurisdiction_iso: &str) -> Self {
        let input = format!("{}|{}", normalized_name, jurisdiction_iso);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        Self(format!("{:x}", result))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Canadian Business Number with program account.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanadianBn(String);

impl CanadianBn {
    pub fn new(value: &str) -> Option<Self> {
        if value.len() == 15 {
            let (digits1, rest) = value.split_at(9);
            let (letters, digits2) = rest.split_at(2);
            if digits1.chars().all(|c| c.is_ascii_digit())
                && letters.chars().all(|c| c.is_ascii_uppercase())
                && digits2.chars().all(|c| c.is_ascii_digit())
            {
                return Some(Self(value.to_string()));
            }
        }
        None
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// An additional identifier scheme.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalScheme {
    pub scheme_uri: String,
    pub value: String,
}

/// Collection of all known identifiers for an entity.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityIdentifiers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sam_uei: Option<SamUei>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<Lei>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snfei: Option<Snfei>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canadian_bn: Option<CanadianBn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_schemes: Option<Vec<AdditionalScheme>>,
}

impl EntityIdentifiers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sam_uei(mut self, uei: SamUei) -> Self {
        self.sam_uei = Some(uei);
        self
    }

    pub fn with_lei(mut self, lei: Lei) -> Self {
        self.lei = Some(lei);
        self
    }

    pub fn with_snfei(mut self, snfei: Snfei) -> Self {
        self.snfei = Some(snfei);
        self
    }

    pub fn has_any(&self) -> bool {
        self.sam_uei.is_some()
            || self.lei.is_some()
            || self.snfei.is_some()
            || self.canadian_bn.is_some()
            || self.additional_schemes.as_ref().map_or(false, |v| !v.is_empty())
    }

    pub fn primary_identifier(&self) -> Option<String> {
        if let Some(ref lei) = self.lei {
            return Some(format!("cep-entity:lei:{}", lei.as_str()));
        }
        if let Some(ref uei) = self.sam_uei {
            return Some(format!("cep-entity:sam-uei:{}", uei.as_str()));
        }
        if let Some(ref snfei) = self.snfei {
            return Some(format!("cep-entity:snfei:{}", snfei.as_str()));
        }
        if let Some(ref bn) = self.canadian_bn {
            return Some(format!("cep-entity:canadian-bn:{}", bn.as_str()));
        }
        None
    }
}

impl Canonicalize for EntityIdentifiers {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        if let Some(ref schemes) = self.additional_schemes {
            if !schemes.is_empty() {
                let mut sorted: Vec<_> = schemes.iter().collect();
                sorted.sort_by(|a, b| a.scheme_uri.cmp(&b.scheme_uri));
                let json = serde_json::to_string(&sorted).unwrap_or_default();
                map.insert("additionalSchemes".to_string(), json);
            }
        }
        insert_if_present(&mut map, "canadianBn", self.canadian_bn.as_ref().map(|x| x.as_str()));
        insert_if_present(&mut map, "lei", self.lei.as_ref().map(|x| x.as_str()));
        insert_if_present(&mut map, "samUei", self.sam_uei.as_ref().map(|x| x.as_str()));
        insert_if_present(&mut map, "snfei", self.snfei.as_ref().map(|x| x.as_str()));
        map
    }
}