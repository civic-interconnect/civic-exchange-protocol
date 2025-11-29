"""Provenance chain tracking for CEP exchanges.

Traces the compositional flow of funds through the civic graph.
This is the Category Theory morphism path implementation.
"""

from dataclasses import dataclass

from civic_exchange_protocol.core import Canonicalize, insert_if_present, insert_required


@dataclass
class IntermediaryEntity(Canonicalize):
    """An intermediary entity in the funding chain."""

    entity_id: str
    role_uri: str | None = None

    def with_role(self, role_uri: str) -> "IntermediaryEntity":
        """Return a new IntermediaryEntity with role set."""
        return IntermediaryEntity(entity_id=self.entity_id, role_uri=role_uri)

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields for the intermediary entity.

        Returns:
        -------
        dict[str, str]
            A dictionary containing the canonical field names and values.
        """
        fields: dict[str, str] = {}
        insert_required(fields, "entityId", self.entity_id)
        insert_if_present(fields, "roleUri", self.role_uri)
        return fields


@dataclass
class ProvenanceChain(Canonicalize):
    """Provenance chain tracing the flow of funds."""

    funding_chain_tag: str | None = None
    ultimate_source_entity_id: str | None = None
    intermediary_entities: list[IntermediaryEntity] | None = None
    parent_exchange_id: str | None = None

    def with_funding_chain_tag(self, tag: str) -> "ProvenanceChain":
        """Return a new ProvenanceChain with funding chain tag set."""
        return ProvenanceChain(
            funding_chain_tag=tag,
            ultimate_source_entity_id=self.ultimate_source_entity_id,
            intermediary_entities=self.intermediary_entities,
            parent_exchange_id=self.parent_exchange_id,
        )

    def with_ultimate_source(self, entity_id: str) -> "ProvenanceChain":
        """Return a new ProvenanceChain with ultimate source set."""
        return ProvenanceChain(
            funding_chain_tag=self.funding_chain_tag,
            ultimate_source_entity_id=entity_id,
            intermediary_entities=self.intermediary_entities,
            parent_exchange_id=self.parent_exchange_id,
        )

    def with_intermediary(self, entity: IntermediaryEntity) -> "ProvenanceChain":
        """Return a new ProvenanceChain with an intermediary added."""
        entities = list(self.intermediary_entities) if self.intermediary_entities else []
        entities.append(entity)
        return ProvenanceChain(
            funding_chain_tag=self.funding_chain_tag,
            ultimate_source_entity_id=self.ultimate_source_entity_id,
            intermediary_entities=entities,
            parent_exchange_id=self.parent_exchange_id,
        )

    def with_parent_exchange(self, exchange_id: str) -> "ProvenanceChain":
        """Return a new ProvenanceChain with parent exchange set."""
        return ProvenanceChain(
            funding_chain_tag=self.funding_chain_tag,
            ultimate_source_entity_id=self.ultimate_source_entity_id,
            intermediary_entities=self.intermediary_entities,
            parent_exchange_id=exchange_id,
        )

    def has_any(self) -> bool:
        """Return True if any provenance information is present."""
        return (
            self.funding_chain_tag is not None
            or self.ultimate_source_entity_id is not None
            or (self.intermediary_entities is not None and len(self.intermediary_entities) > 0)
            or self.parent_exchange_id is not None
        )

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields for the provenance chain.

        Returns:
        -------
        dict[str, str]
            A dictionary containing the canonical field names and values.
        """
        fields: dict[str, str] = {}

        insert_if_present(fields, "fundingChainTag", self.funding_chain_tag)

        # Intermediary entities serialized as array
        if self.intermediary_entities:
            entities_json = ",".join(e.to_canonical_string() for e in self.intermediary_entities)
            fields["intermediaryEntities"] = f"[{entities_json}]"

        insert_if_present(fields, "parentExchangeId", self.parent_exchange_id)
        insert_if_present(fields, "ultimateSourceEntityId", self.ultimate_source_entity_id)

        return fields


@dataclass
class ExchangeCategorization(Canonicalize):
    """Categorization codes for reporting and analysis."""

    cfda_number: str | None = None
    naics_code: str | None = None
    gtas_account_code: str | None = None
    local_category_code: str | None = None
    local_category_label: str | None = None

    def with_cfda(self, cfda: str) -> "ExchangeCategorization":
        """Return a new ExchangeCategorization with CFDA set."""
        return ExchangeCategorization(
            cfda_number=cfda,
            naics_code=self.naics_code,
            gtas_account_code=self.gtas_account_code,
            local_category_code=self.local_category_code,
            local_category_label=self.local_category_label,
        )

    def with_naics(self, naics: str) -> "ExchangeCategorization":
        """Return a new ExchangeCategorization with NAICS set."""
        return ExchangeCategorization(
            cfda_number=self.cfda_number,
            naics_code=naics,
            gtas_account_code=self.gtas_account_code,
            local_category_code=self.local_category_code,
            local_category_label=self.local_category_label,
        )

    def with_gtas(self, gtas: str) -> "ExchangeCategorization":
        """Return a new ExchangeCategorization with GTAS set."""
        return ExchangeCategorization(
            cfda_number=self.cfda_number,
            naics_code=self.naics_code,
            gtas_account_code=gtas,
            local_category_code=self.local_category_code,
            local_category_label=self.local_category_label,
        )

    def with_local_category(self, code: str, label: str) -> "ExchangeCategorization":
        """Return a new ExchangeCategorization with local category set."""
        return ExchangeCategorization(
            cfda_number=self.cfda_number,
            naics_code=self.naics_code,
            gtas_account_code=self.gtas_account_code,
            local_category_code=code,
            local_category_label=label,
        )

    def has_any(self) -> bool:
        """Return True if any categorization is present."""
        return (
            self.cfda_number is not None
            or self.naics_code is not None
            or self.gtas_account_code is not None
            or self.local_category_code is not None
        )

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields for the exchange categorization.

        Returns:
        -------
        dict[str, str]
            A dictionary containing the canonical field names and values.
        """
        fields: dict[str, str] = {}
        insert_if_present(fields, "cfdaNumber", self.cfda_number)
        insert_if_present(fields, "gtasAccountCode", self.gtas_account_code)
        insert_if_present(fields, "localCategoryCode", self.local_category_code)
        insert_if_present(fields, "localCategoryLabel", self.local_category_label)
        insert_if_present(fields, "naicsCode", self.naics_code)
        return fields
