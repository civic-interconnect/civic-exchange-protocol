"""Tests for Rust parity across all CEP record types.

These tests use the EXACT same inputs as the Rust test vectors
and verify that the Python implementation produces identical hashes.
"""

from civic_exchange_protocol.core import Attestation, CanonicalTimestamp
from civic_exchange_protocol.entity import (
    EntityIdentifiers,
    EntityRecord,
    EntityStatus,
    EntityStatusCode,
    SamUei,
)
from civic_exchange_protocol.exchange import (
    ExchangeCategorization,
    ExchangeParty,
    ExchangeRecord,
    ExchangeStatus,
    ExchangeStatusCode,
    ExchangeValue,
    ProvenanceChain,
)
from civic_exchange_protocol.relationship import (
    BilateralParties,
    FinancialTerms,
    Party,
    RelationshipRecord,
    RelationshipStatus,
    RelationshipStatusCode,
)
import pytest


def create_test_attestation() -> Attestation:
    """Creates the standard test attestation used across all test vectors."""
    return Attestation.new(
        attestor_id="cep-entity:sam-uei:ATTESTOR123A",
        attestation_timestamp=CanonicalTimestamp.parse("2025-11-28T14:30:00.000000Z"),
        proof_type="Ed25519Signature2020",
        proof_value="z3FXQqFwbZxKBxGxqFpCDabcdef1234567890",
        verification_method_uri="did:web:example.gov#key-1",
    )


class TestEntityRustParity:
    """Tests that Entity records produce identical hashes to Rust."""

    def test_basic_entity_hash(self):
        """Must match Rust test_vector_basic_entity output."""
        identifiers = EntityIdentifiers(
            sam_uei=SamUei("J6H4FB3N5YK7"),
        )

        status = EntityStatus(
            status_code=EntityStatusCode.ACTIVE,
            status_effective_date="2020-01-15",
        )

        entity = EntityRecord.new(
            verifiable_id="cep-entity:sam-uei:J6H4FB3N5YK7",
            identifiers=identifiers,
            legal_name="Acme Consulting LLC",
            jurisdiction_iso="US-CA",
            status=status,
            attestation=create_test_attestation(),
        )

        # This hash MUST match the Rust output
        expected_hash = "2dea875a9a7c8531dd787c7be0d9321bcf5347f7b9be731995f3bcfb15bc3249"
        actual_hash = entity.calculate_hash().as_hex()

        print(f"\n{'=' * 60}")
        print("TEST VECTOR: Basic Entity Record")
        print(f"{'=' * 60}")
        print(f"\nCanonical String:\n{entity.to_canonical_string()}")
        print(f"\nExpected Hash: {expected_hash}")
        print(f"Actual Hash:   {actual_hash}")
        print(f"{'=' * 60}\n")

        assert actual_hash == expected_hash, (
            f"Entity hash mismatch!\n"
            f"Expected: {expected_hash}\n"
            f"Actual:   {actual_hash}\n"
            f"Canonical: {entity.to_canonical_string()}"
        )


class TestRelationshipRustParity:
    """Tests that Relationship records produce identical hashes to Rust."""

    def test_bilateral_relationship_hash(self):
        """Must match Rust test_vector_bilateral_relationship output."""
        parties = BilateralParties(
            party_a=Party(
                entity_id="cep-entity:sam-uei:AGENCY12345A",
                role_uri="https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/party-role.json#grantor",
            ),
            party_b=Party(
                entity_id="cep-entity:sam-uei:VENDOR67890B",
                role_uri="https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/party-role.json#grantee",
            ),
        )

        status = RelationshipStatus(
            status_code=RelationshipStatusCode.ACTIVE,
            status_effective_timestamp=CanonicalTimestamp.parse("2025-01-01T00:00:00.000000Z"),
        )

        relationship = RelationshipRecord.new_bilateral(
            verifiable_id="cep-relationship:usaspending:CONT_AWD_12345",
            relationship_type_uri="https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/relationship-type.json#prime-contract",
            parties=parties,
            effective_timestamp=CanonicalTimestamp.parse("2025-01-01T00:00:00.000000Z"),
            status=status,
            jurisdiction_iso="US",
            attestation=create_test_attestation(),
        ).with_financial_terms(
            FinancialTerms(
                total_value=500000.00,
                obligated_value=250000.00,
                currency_code="USD",
            )
        )

        # This hash MUST match the Rust output
        expected_hash = "cc1f44ff2cc6e121d698c840a4ad9596a9f90feb76386182a57fbde3b04971bf"
        actual_hash = relationship.calculate_hash().as_hex()

        print(f"\n{'=' * 60}")
        print("TEST VECTOR: Bilateral Relationship Record")
        print(f"{'=' * 60}")
        print(f"\nCanonical String:\n{relationship.to_canonical_string()}")
        print(f"\nExpected Hash: {expected_hash}")
        print(f"Actual Hash:   {actual_hash}")
        print(f"{'=' * 60}\n")

        assert actual_hash == expected_hash, (
            f"Relationship hash mismatch!\n"
            f"Expected: {expected_hash}\n"
            f"Actual:   {actual_hash}\n"
            f"Canonical: {relationship.to_canonical_string()}"
        )


class TestExchangeRustParity:
    """Tests that Exchange records produce identical hashes to Rust."""

    def test_basic_exchange_canonical_structure(self):
        """Verify exchange canonical string is properly formed."""
        source = ExchangeParty(
            entity_id="cep-entity:sam-uei:AGENCY12345A",
            role_uri="https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/exchange-role.json#disbursing-agency",
        )

        recipient = ExchangeParty(
            entity_id="cep-entity:sam-uei:SCHOOL67890B",
            role_uri="https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/exchange-role.json#grantee",
        )

        value = ExchangeValue.usd(50000.00)

        status = ExchangeStatus(
            status_code=ExchangeStatusCode.COMPLETED,
            status_effective_timestamp=CanonicalTimestamp.parse("2025-09-15T14:03:22.500000Z"),
        )

        exchange = (
            ExchangeRecord.new(
                verifiable_id="cep-exchange:treasury:PAY_2025_001234",
                relationship_id="cep-relationship:usaspending:GRANT_84010_2025",
                exchange_type_uri="https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/exchange-type.json#grant-disbursement",
                source_entity=source,
                recipient_entity=recipient,
                value=value,
                occurred_timestamp=CanonicalTimestamp.parse("2025-09-15T14:03:22.500000Z"),
                status=status,
                attestation=create_test_attestation(),
            )
            .with_provenance(
                ProvenanceChain()
                .with_funding_chain_tag("FEDERAL>STATE>SCHOOL_DISTRICT")
                .with_ultimate_source("cep-entity:sam-uei:USDOE12345AB")
            )
            .with_categorization(ExchangeCategorization().with_cfda("84.010"))
        )

        canonical = exchange.to_canonical_string()
        hash_val = exchange.calculate_hash()

        print(f"\n{'=' * 60}")
        print("TEST VECTOR: Basic Exchange Record")
        print(f"{'=' * 60}")
        print(f"\nCanonical String:\n{canonical}")
        print(f"\nSHA-256 Hash:\n{hash_val.as_hex()}")
        print(f"{'=' * 60}\n")

        # Verify structure
        assert '"attestation":' in canonical
        assert '"exchangeTypeUri":' in canonical
        assert '"occurredTimestamp":' in canonical
        assert '"provenanceChain":' in canonical
        assert '"categorization":' in canonical
        assert '"sourceEntity":' in canonical
        assert '"recipientEntity":' in canonical
        assert '"value":' in canonical


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-s"])
