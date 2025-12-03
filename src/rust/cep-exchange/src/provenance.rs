/// Provenance chain tracking for CEP exchanges.
///
/// Traces the compositional flow of funds through the civic graph.
/// This is the Category Theory morphism path implementation.

use cep_core::canonical::{insert_if_present, insert_required, Canonicalize};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// An intermediary entity in the funding chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntermediaryEntity {
    /// Verifiable ID of the intermediary entity.
    pub entity_id: String,

    /// URI specifying the entity's role in the chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_uri: Option<String>,
}

impl IntermediaryEntity {
    pub fn new(entity_id: String) -> Self {
        Self {
            entity_id,
            role_uri: None,
        }
    }

    pub fn with_role(mut self, role_uri: String) -> Self {
        self.role_uri = Some(role_uri);
        self
    }
}

impl Canonicalize for IntermediaryEntity {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        insert_required(&mut map, "entityId", &self.entity_id);
        insert_if_present(&mut map, "roleUri", self.role_uri.as_deref());
        map
    }
}

/// Provenance chain tracing the flow of funds.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceChain {
    /// Human-readable hierarchical trace.
    /// Format: "FEDERAL>STATE>COUNTY>SCHOOL_DISTRICT"
    /// Uses '>' as the composition operator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_chain_tag: Option<String>,

    /// Verifiable ID of the original source of funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimate_source_entity_id: Option<String>,

    /// Ordered list of entities through which value passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediary_entities: Option<Vec<IntermediaryEntity>>,

    /// Verifiable ID of the upstream exchange.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_exchange_id: Option<String>,
}

impl ProvenanceChain {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the funding chain tag.
    /// Example: "FEDERAL>STATE>LOCAL"
    pub fn with_funding_chain_tag(mut self, tag: String) -> Self {
        self.funding_chain_tag = Some(tag);
        self
    }

    /// Sets the ultimate source entity.
    pub fn with_ultimate_source(mut self, entity_id: String) -> Self {
        self.ultimate_source_entity_id = Some(entity_id);
        self
    }

    /// Adds an intermediary entity to the chain.
    pub fn with_intermediary(mut self, entity: IntermediaryEntity) -> Self {
        self.intermediary_entities
            .get_or_insert_with(Vec::new)
            .push(entity);
        self
    }

    /// Sets the parent exchange ID.
    pub fn with_parent_exchange(mut self, exchange_id: String) -> Self {
        self.parent_exchange_id = Some(exchange_id);
        self
    }

    /// Returns true if any provenance information is present.
    pub fn has_any(&self) -> bool {
        self.funding_chain_tag.is_some()
            || self.ultimate_source_entity_id.is_some()
            || self.intermediary_entities.as_ref().map_or(false, |v| !v.is_empty())
            || self.parent_exchange_id.is_some()
    }
}

impl Canonicalize for ProvenanceChain {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();

        insert_if_present(&mut map, "fundingChainTag", self.funding_chain_tag.as_deref());

        // Intermediary entities serialized as array
        if let Some(ref entities) = self.intermediary_entities {
            if !entities.is_empty() {
                let json: Vec<String> = entities.iter()
                    .map(|e| e.to_canonical_string())
                    .collect();
                map.insert("intermediaryEntities".to_string(), format!("[{}]", json.join(",")));
            }
        }

        insert_if_present(&mut map, "parentExchangeId", self.parent_exchange_id.as_deref());
        insert_if_present(&mut map, "ultimateSourceEntityId", self.ultimate_source_entity_id.as_deref());

        map
    }
}

/// Categorization codes for reporting and analysis.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeCategorization {
    /// Catalog of Federal Domestic Assistance number (e.g., "84.010").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfda_number: Option<String>,

    /// NAICS code for goods/services exchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naics_code: Option<String>,

    /// Government-wide Treasury Account Symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtas_account_code: Option<String>,

    /// Jurisdiction-specific category code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_category_code: Option<String>,

    /// Human-readable label for local category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_category_label: Option<String>,
}

impl ExchangeCategorization {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_cfda(mut self, cfda: String) -> Self {
        self.cfda_number = Some(cfda);
        self
    }

    pub fn with_naics(mut self, naics: String) -> Self {
        self.naics_code = Some(naics);
        self
    }

    pub fn with_gtas(mut self, gtas: String) -> Self {
        self.gtas_account_code = Some(gtas);
        self
    }

    pub fn with_local_category(mut self, code: String, label: String) -> Self {
        self.local_category_code = Some(code);
        self.local_category_label = Some(label);
        self
    }

    pub fn has_any(&self) -> bool {
        self.cfda_number.is_some()
            || self.naics_code.is_some()
            || self.gtas_account_code.is_some()
            || self.local_category_code.is_some()
    }
}

impl Canonicalize for ExchangeCategorization {
    fn canonical_fields(&self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        insert_if_present(&mut map, "cfdaNumber", self.cfda_number.as_deref());
        insert_if_present(&mut map, "gtasAccountCode", self.gtas_account_code.as_deref());
        insert_if_present(&mut map, "localCategoryCode", self.local_category_code.as_deref());
        insert_if_present(&mut map, "localCategoryLabel", self.local_category_label.as_deref());
        insert_if_present(&mut map, "naicsCode", self.naics_code.as_deref());
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funding_chain_tag() {
        let chain = ProvenanceChain::new()
            .with_funding_chain_tag("FEDERAL>STATE>LOCAL".to_string());

        let canonical = chain.to_canonical_string();
        assert!(canonical.contains("FEDERAL>STATE>LOCAL"));
    }

    #[test]
    fn test_provenance_chain_full() {
        let chain = ProvenanceChain::new()
            .with_funding_chain_tag("FEDERAL>STATE>SCHOOL_DISTRICT".to_string())
            .with_ultimate_source("cep-entity:sam-uei:USDOE12345AB".to_string())
            .with_intermediary(IntermediaryEntity::new("cep-entity:sam-uei:STATEED123A".to_string()))
            .with_parent_exchange("cep-exchange:treasury:PAY_2025_000001".to_string());

        assert!(chain.has_any());
        let fields = chain.canonical_fields();
        assert!(fields.contains_key("fundingChainTag"));
        assert!(fields.contains_key("ultimateSourceEntityId"));
        assert!(fields.contains_key("intermediaryEntities"));
        assert!(fields.contains_key("parentExchangeId"));
    }

    #[test]
    fn test_categorization() {
        let cat = ExchangeCategorization::new()
            .with_cfda("84.010".to_string())
            .with_local_category("TITLE1".to_string(), "Title I Funds".to_string());

        let canonical = cat.to_canonical_string();
        assert!(canonical.contains("84.010"));
        assert!(canonical.contains("TITLE1"));
    }
}