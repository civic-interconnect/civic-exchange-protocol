//! Multilateral member definitions for n-ary relationships.
//!
//! Multilateral relationships involve more than two parties, such as:
//! - Consortia
//! - Joint ventures
//! - Board memberships
//!
//! Members are stored in a `BTreeSet` to guarantee deterministic ordering
//! for hash stability across all implementations.

use cep_core::canonical::{insert_required, Canonicalize};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

/// A member in a multilateral relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    /// Verifiable ID of the member entity.
    pub entity_id: String,
    /// URI specifying the member's role.
    pub role_uri: String,
    /// Fractional participation (0.0-1.0) for ownership or cost-sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participation_share: Option<f64>,
}

impl Member {
    pub fn new(entity_id: String, role_uri: String) -> Self {
        Self {
            entity_id,
            role_uri,
            participation_share: None,
        }
    }

    pub fn with_share(mut self, share: f64) -> Self {
        self.participation_share = Some(share);
        self
    }
}

// Implement ordering based on entity_id for BTreeSet
impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl Eq for Member {}

impl PartialOrd for Member {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Member {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entity_id.cmp(&other.entity_id)
    }
}

impl Canonicalize for Member {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        insert_required(&mut map, "entityId", &self.entity_id);
        if let Some(share) = self.participation_share {
            insert_required(&mut map, "participationShare", &format!("{:.4}", share));
        }
        insert_required(&mut map, "roleUri", &self.role_uri);
        map
    }
}

/// A collection of members in a multilateral relationship.
///
/// Uses `BTreeSet` to guarantee members are always sorted by `entity_id`,
/// ensuring hash stability regardless of insertion order.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultilateralMembers(BTreeSet<Member>);

impl MultilateralMembers {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    /// Adds a member to the set.
    pub fn add(&mut self, member: Member) {
        self.0.insert(member);
    }

    /// Returns the number of members.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if there are no members.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over members in sorted order.
    pub fn iter(&self) -> impl Iterator<Item = &Member> {
        self.0.iter()
    }

    /// Validates that all participation shares sum to 1.0 (if present).
    pub fn validate_shares(&self) -> Result<(), String> {
        let shares: Vec<f64> = self.0.iter()
            .filter_map(|m| m.participation_share)
            .collect();

        if shares.is_empty() {
            return Ok(());
        }

        if shares.len() != self.0.len() {
            return Err("All members must have participation shares if any do".to_string());
        }

        let total: f64 = shares.iter().sum();
        if (total - 1.0).abs() > 0.0001 {
            return Err(format!("Participation shares must sum to 1.0, got {:.4}", total));
        }

        Ok(())
    }
}

impl Canonicalize for MultilateralMembers {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        // Serialize as a JSON array for the canonical form
        // Members are already sorted by entity_id due to BTreeSet
        let members_json: Vec<String> = self.0.iter()
            .map(|m| m.to_canonical_string())
            .collect();
        
        let mut map = BTreeMap::new();
        if !members_json.is_empty() {
            map.insert("members".to_string(), format!("[{}]", members_json.join(",")));
        }
        map
    }
}

impl FromIterator<Member> for MultilateralMembers {
    fn from_iter<I: IntoIterator<Item = Member>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_member_ordering() {
        let m1 = Member::new("entity-a".to_string(), "role".to_string());
        let m2 = Member::new("entity-b".to_string(), "role".to_string());
        let m3 = Member::new("entity-c".to_string(), "role".to_string());

        // Insert in reverse order
        let mut members = MultilateralMembers::new();
        members.add(m3.clone());
        members.add(m1.clone());
        members.add(m2.clone());

        // Should iterate in sorted order
        let ids: Vec<&str> = members.iter().map(|m| m.entity_id.as_str()).collect();
        assert_eq!(ids, vec!["entity-a", "entity-b", "entity-c"]);
    }

    #[test]
    fn test_insertion_order_invariance() {
        // Create two sets with different insertion orders
        let mut set_a = MultilateralMembers::new();
        set_a.add(Member::new("bank-001".to_string(), "role".to_string()));
        set_a.add(Member::new("citizen-005".to_string(), "role".to_string()));
        set_a.add(Member::new("regulator-002".to_string(), "role".to_string()));

        let mut set_b = MultilateralMembers::new();
        set_b.add(Member::new("regulator-002".to_string(), "role".to_string()));
        set_b.add(Member::new("bank-001".to_string(), "role".to_string()));
        set_b.add(Member::new("citizen-005".to_string(), "role".to_string()));

        // Canonical strings must be identical
        assert_eq!(set_a.to_canonical_string(), set_b.to_canonical_string());
    }

    #[test]
    fn test_participation_shares_valid() {
        let mut members = MultilateralMembers::new();
        members.add(Member::new("a".to_string(), "role".to_string()).with_share(0.5));
        members.add(Member::new("b".to_string(), "role".to_string()).with_share(0.3));
        members.add(Member::new("c".to_string(), "role".to_string()).with_share(0.2));

        assert!(members.validate_shares().is_ok());
    }

    #[test]
    fn test_participation_shares_invalid_sum() {
        let mut members = MultilateralMembers::new();
        members.add(Member::new("a".to_string(), "role".to_string()).with_share(0.5));
        members.add(Member::new("b".to_string(), "role".to_string()).with_share(0.3));
        // Missing 0.2 - should fail

        assert!(members.validate_shares().is_err());
    }
}