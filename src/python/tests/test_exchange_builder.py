"""Tests for exchange builder functionality."""

from civic_exchange_protocol.exchange import (
    ExchangeRecord,
    ExchangeStatusCode,
)
from civic_exchange_protocol.exchange.exchange_builder import (
    EXCHANGE_REQUIRED,
    build_attestation,
    build_categorization,
    build_exchange,
    build_exchange_party,
    build_exchange_value,
    build_source_reference,
    map_fields,
    parse_exchange_type_uri,
    parse_timestamp,
    validate_required,
)
import pytest


class TestMapFields:
    """Tests for field mapping."""

    def test_grant_style_fields(self):
        raw = {"grantorEntityId": "A", "granteeEntityId": "B"}
        mapped, warnings = map_fields(raw)
        assert mapped["source_entity_id"] == "A"
        assert mapped["recipient_entity_id"] == "B"
        assert warnings == []

    def test_generic_style_fields(self):
        raw = {"sourceEntityId": "A", "recipientEntityId": "B"}
        mapped, _ = map_fields(raw)
        assert mapped["source_entity_id"] == "A"
        assert mapped["recipient_entity_id"] == "B"

    def test_unknown_fields_warned(self):
        raw = {"exchangeId": "123", "unknownField": "value"}
        mapped, warnings = map_fields(raw)
        assert "exchange_id" in mapped
        assert "unknownField" not in mapped
        assert len(warnings) == 1
        assert "unknownField" in warnings[0]

    def test_all_standard_fields(self):
        raw = {
            "exchangeId": "EX-001",
            "exchangeType": "GRANT",
            "grantorEntityId": "A",
            "granteeEntityId": "B",
            "grantAmount": 1000.0,
            "currency": "USD",
            "awardDate": "2024-01-01",
            "description": "Test grant",
        }
        mapped, warnings = map_fields(raw)
        assert mapped["exchange_id"] == "EX-001"
        assert mapped["exchange_type"] == "GRANT"
        assert mapped["amount"] == 1000.0
        assert mapped["currency"] == "USD"
        assert mapped["occurred_date"] == "2024-01-01"
        assert warnings == []


class TestValidateRequired:
    """Tests for required field validation."""

    def test_missing_fields_raises(self):
        mapped = {"exchange_id": "123"}
        with pytest.raises(ValueError, match="Missing required fields"):
            validate_required(mapped)

    def test_null_field_raises(self):
        mapped: dict = dict.fromkeys(EXCHANGE_REQUIRED, "value")
        mapped["exchange_id"] = None
        with pytest.raises(ValueError, match="cannot be null"):
            validate_required(mapped)

    def test_all_required_present(self):
        mapped = {
            "exchange_id": "EX-001",
            "exchange_type": "GRANT",
            "source_entity_id": "A",
            "recipient_entity_id": "B",
            "amount": 1000.0,
            "currency": "USD",
            "occurred_date": "2024-01-01",
            "attestation": {"attestedBy": "Test"},
        }
        validate_required(mapped)  # Should not raise


class TestParseExchangeTypeUri:
    """Tests for exchange type URI parsing."""

    def test_known_types(self):
        assert parse_exchange_type_uri("GRANT") == "https://civic-exchange.org/types/grant"
        assert parse_exchange_type_uri("CONTRACT") == "https://civic-exchange.org/types/contract"
        assert parse_exchange_type_uri("PAYMENT") == "https://civic-exchange.org/types/payment"

    def test_case_insensitive(self):
        assert parse_exchange_type_uri("grant") == "https://civic-exchange.org/types/grant"
        assert parse_exchange_type_uri("Grant") == "https://civic-exchange.org/types/grant"

    def test_unknown_type(self):
        assert parse_exchange_type_uri("CUSTOM") == "https://civic-exchange.org/types/custom"
        assert parse_exchange_type_uri("MyType") == "https://civic-exchange.org/types/mytype"


class TestParseTimestamp:
    """Tests for timestamp parsing."""

    def test_date_only(self):
        ts = parse_timestamp("2024-05-15")
        canonical = ts.to_canonical_string()
        assert "2024-05-15" in canonical
        assert "T00:00:00" in canonical

    def test_full_datetime(self):
        ts = parse_timestamp("2024-05-15T14:02:10.491823Z")
        canonical = ts.to_canonical_string()
        assert "2024-05-15" in canonical
        assert "14:02:10" in canonical


class TestBuildAttestation:
    """Tests for attestation building."""

    def test_basic_attestation(self):
        raw = {
            "attestedBy": "John Doe",
            "attestationTimestamp": "2024-05-15T14:02:10.491823Z",
        }
        att = build_attestation(raw)
        assert att.attestor_id == "John Doe"
        assert "john-doe" in att.verification_method_uri.lower()
        assert att.proof_type == "ManualAttestation"

    def test_missing_attested_by(self):
        raw = {"attestationTimestamp": "2024-05-15T14:02:10.491823Z"}
        att = build_attestation(raw)
        assert att.attestor_id == "unknown"


class TestBuildExchangeParty:
    """Tests for exchange party building."""

    def test_basic_party(self):
        party = build_exchange_party("US-FED-001")
        assert party.entity_id == "US-FED-001"
        assert party.role_uri is None


class TestBuildExchangeValue:
    """Tests for exchange value building."""

    def test_monetary_value(self):
        value = build_exchange_value(1250000.0, "USD")
        assert value.amount == 1250000.0
        assert value.currency_code == "USD"

    def test_other_currency(self):
        value = build_exchange_value(500.0, "EUR")
        assert value.currency_code == "EUR"


class TestBuildCategorization:
    """Tests for categorization building."""

    def test_no_categorization(self):
        mapped = {"exchange_id": "123"}
        result = build_categorization(mapped)
        assert result is None

    def test_with_cfda(self):
        mapped = {"cfda_number": "84.010"}
        result = build_categorization(mapped)
        assert result is not None
        assert result.cfda_number == "84.010"

    def test_with_local_category(self):
        mapped = {
            "local_category_code": "ED-TITLEI",
            "description": "Title I funding",
        }
        result = build_categorization(mapped)
        assert result is not None
        assert result.local_category_code == "ED-TITLEI"
        assert result.local_category_label == "Title I funding"


class TestBuildSourceReference:
    """Tests for source reference building."""

    def test_no_source_reference(self):
        mapped = {"exchange_id": "123"}
        result = build_source_reference(mapped)
        assert result is None

    def test_incomplete_source_reference(self):
        mapped = {"source_system": "http://example.com"}
        result = build_source_reference(mapped)
        assert result is None

    def test_complete_source_reference(self):
        mapped = {
            "source_system": "http://example.com",
            "source_record_id": "REC-001",
            "source_url": "http://example.com/rec/001",
        }
        result = build_source_reference(mapped)
        assert result is not None
        assert result.source_system_uri == "http://example.com"
        assert result.source_record_id == "REC-001"
        assert result.source_url == "http://example.com/rec/001"


class TestBuildExchange:
    """Integration tests for full exchange building."""

    def test_minimal_exchange(self):
        raw = {
            "exchangeId": "EX-001",
            "exchangeType": "GRANT",
            "grantorEntityId": "US-FED-001",
            "granteeEntityId": "US-CA-001",
            "grantAmount": 1000.0,
            "currency": "USD",
            "awardDate": "2024-01-01",
            "attestation": {
                "attestedBy": "Test User",
                "attestationTimestamp": "2024-01-01T12:00:00.000000Z",
            },
        }
        result = build_exchange(raw)
        assert result.warnings == []
        assert isinstance(result.exchange, ExchangeRecord)
        assert result.exchange.verifiable_id == "EX-001"

    def test_full_exchange(self):
        raw = {
            "exchangeId": "EX-2024-1011",
            "exchangeType": "GRANT",
            "grantorEntityId": "US-FED-ED-001",
            "granteeEntityId": "US-CA-SD-0001",
            "grantAmount": 1250000.00,
            "currency": "USD",
            "awardDate": "2024-05-15",
            "programCode": "ED-TITLEI",
            "description": "Title I funding allocation.",
            "attestation": {
                "attestedBy": "Maria Lopez, Federal Grants Specialist",
                "attestationTimestamp": "2024-05-15T14:02:10.491823Z",
            },
        }
        result = build_exchange(raw)
        exchange = result.exchange

        assert exchange.verifiable_id == "EX-2024-1011"
        assert exchange.value.amount == 1250000.00
        assert exchange.value.currency_code == "USD"
        assert exchange.source_entity.entity_id == "US-FED-ED-001"
        assert exchange.recipient_entity.entity_id == "US-CA-SD-0001"
        assert exchange.status.status_code == ExchangeStatusCode.COMPLETED
        assert exchange.categorization is not None
        assert exchange.categorization.local_category_code == "ED-TITLEI"

    def test_missing_required_field(self):
        raw = {
            "exchangeId": "EX-001",
            "exchangeType": "GRANT",
        }
        with pytest.raises(ValueError, match="Missing required fields"):
            build_exchange(raw)

    def test_unknown_fields_warned(self):
        raw = {
            "exchangeId": "EX-001",
            "exchangeType": "GRANT",
            "grantorEntityId": "A",
            "granteeEntityId": "B",
            "grantAmount": 100.0,
            "currency": "USD",
            "awardDate": "2024-01-01",
            "attestation": {
                "attestedBy": "Test",
                "attestationTimestamp": "2024-01-01T00:00:00.000000Z",
            },
            "customField": "ignored",
        }
        result = build_exchange(raw)
        assert len(result.warnings) == 1
        assert "customField" in result.warnings[0]
