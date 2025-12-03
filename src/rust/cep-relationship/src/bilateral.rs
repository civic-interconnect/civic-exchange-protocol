/// Bilateral party definitions for two-party relationships.
///
/// Bilateral relationships have clear directionality:
/// - **Party A**: The initiating, granting, or contracting party
/// - **Party B**: The receiving, performing, or beneficiary party

use cep_core::canonical::{insert_required, Canonicalize};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A party in a bilateral relationship.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Party {
    /// Verifiable ID of the entity.
    pub entity_id: String,
    /// URI specifying the party's role.
    pub role_uri: String,
}

impl Party {
    pub fn new(entity_id: String, role_uri: String) -> Self {
        Self { entity_id, role_uri }
    }
}

impl Canonicalize for Party {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        insert_required(&mut map, "entityId", &self.entity_id);
        insert_required(&mut map, "roleUri", &self.role_uri);
        map
    }
}

/// Bilateral parties in a two-party relationship.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BilateralParties {
    /// The initiating, granting, or contracting party.
    pub party_a: Party,
    /// The receiving, performing, or beneficiary party.
    pub party_b: Party,
}

impl BilateralParties {
    pub fn new(party_a: Party, party_b: Party) -> Self {
        Self { party_a, party_b }
    }
}

impl Canonicalize for BilateralParties {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        // Nested objects serialized as their canonical strings
        insert_required(&mut map, "partyA", &self.party_a.to_canonical_string());
        insert_required(&mut map, "partyB", &self.party_b.to_canonical_string());
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_party_canonical() {
        let party = Party::new(
            "cep-entity:sam-uei:J6H4FB3N5YK7".to_string(),
            "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/party-role.json#grantor".to_string(),
        );

        let canonical = party.to_canonical_string();
        assert!(canonical.contains(r#""entityId":"cep-entity:sam-uei:J6H4FB3N5YK7""#));
        assert!(canonical.contains(r#""roleUri":"#));
    }

    #[test]
    fn test_bilateral_canonical_order() {
        let parties = BilateralParties::new(
            Party::new("entity-a".to_string(), "role-a".to_string()),
            Party::new("entity-b".to_string(), "role-b".to_string()),
        );

        let fields = parties.canonical_fields();
        let keys: Vec<&String> = fields.keys().collect();
        assert_eq!(keys, vec!["partyA", "partyB"]);
    }
}