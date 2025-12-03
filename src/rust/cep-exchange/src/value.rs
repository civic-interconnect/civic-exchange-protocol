/// Value types for CEP exchanges.
///
/// Supports monetary values (with currency) and in-kind contributions.

use cep_core::canonical::{format_amount, insert_if_present, insert_required, Canonicalize};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The type of value being exchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueType {
    /// URI identifying the value type.
    pub type_uri: String,
}

impl ValueType {
    pub fn monetary() -> Self {
        Self {
            type_uri: "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/value-type.json#monetary".to_string(),
        }
    }

    pub fn in_kind() -> Self {
        Self {
            type_uri: "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/value-type.json#in-kind".to_string(),
        }
    }

    pub fn service_hours() -> Self {
        Self {
            type_uri: "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/value-type.json#service-hours".to_string(),
        }
    }
}

/// The value being exchanged.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeValue {
    /// Numeric value of the exchange.
    /// Must be >= 0 for standard exchanges; negative for reversals/corrections.
    pub amount: f64,

    /// ISO 4217 currency code for monetary exchanges.
    #[serde(default = "default_currency")]
    pub currency_code: String,

    /// URI indicating the nature of value.
    #[serde(default = "default_value_type")]
    pub value_type_uri: String,

    /// Description for non-monetary exchanges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_kind_description: Option<String>,
}

fn default_currency() -> String {
    "USD".to_string()
}

fn default_value_type() -> String {
    "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/value-type.json#monetary".to_string()
}

impl ExchangeValue {
    /// Creates a new monetary value.
    pub fn monetary(amount: f64, currency_code: &str) -> Self {
        Self {
            amount,
            currency_code: currency_code.to_string(),
            value_type_uri: default_value_type(),
            in_kind_description: None,
        }
    }

    /// Creates a new USD monetary value.
    pub fn usd(amount: f64) -> Self {
        Self::monetary(amount, "USD")
    }

    /// Creates an in-kind value with description.
    pub fn in_kind(amount: f64, description: &str) -> Self {
        Self {
            amount,
            currency_code: "USD".to_string(),
            value_type_uri: ValueType::in_kind().type_uri,
            in_kind_description: Some(description.to_string()),
        }
    }
}

impl Canonicalize for ExchangeValue {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        // Amount formatted to exactly 2 decimal places
        insert_required(&mut map, "amount", &format_amount(self.amount));
        insert_required(&mut map, "currencyCode", &self.currency_code);
        insert_if_present(&mut map, "inKindDescription", self.in_kind_description.as_deref());
        insert_required(&mut map, "valueTypeUri", &self.value_type_uri);
        map
    }
}

/// A party in an exchange (source or recipient).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeParty {
    /// Verifiable ID of the entity.
    pub entity_id: String,

    /// URI specifying the party's role in this exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_uri: Option<String>,

    /// Optional account or fund identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<String>,
}

impl ExchangeParty {
    pub fn new(entity_id: String) -> Self {
        Self {
            entity_id,
            role_uri: None,
            account_identifier: None,
        }
    }

    pub fn with_role(mut self, role_uri: String) -> Self {
        self.role_uri = Some(role_uri);
        self
    }

    pub fn with_account(mut self, account: String) -> Self {
        self.account_identifier = Some(account);
        self
    }
}

impl Canonicalize for ExchangeParty {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        insert_if_present(&mut map, "accountIdentifier", self.account_identifier.as_deref());
        insert_required(&mut map, "entityId", &self.entity_id);
        insert_if_present(&mut map, "roleUri", self.role_uri.as_deref());
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monetary_value() {
        let value = ExchangeValue::usd(50000.00);
        assert_eq!(value.amount, 50000.00);
        assert_eq!(value.currency_code, "USD");
    }

    #[test]
    fn test_value_canonical_format() {
        let value = ExchangeValue::usd(50000.756);
        let fields = value.canonical_fields();
        // Should be rounded to 2 decimal places
        assert_eq!(fields.get("amount").unwrap(), "50000.76");
    }

    #[test]
    fn test_in_kind_value() {
        let value = ExchangeValue::in_kind(10000.00, "Office equipment - 10 laptops");
        assert!(value.in_kind_description.is_some());
        let canonical = value.to_canonical_string();
        assert!(canonical.contains("inKindDescription"));
    }

    #[test]
    fn test_exchange_party() {
        let party = ExchangeParty::new("cep-entity:sam-uei:ABC123DEF456".to_string())
            .with_role("https://example.com/role/disbursing-agency".to_string())
            .with_account("TAS-123-456".to_string());

        let canonical = party.to_canonical_string();
        assert!(canonical.contains("accountIdentifier"));
        assert!(canonical.contains("entityId"));
        assert!(canonical.contains("roleUri"));
    }
}