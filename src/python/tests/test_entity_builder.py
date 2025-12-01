from civic_exchange_protocol.entity.entity_builder import (
    localize_name,
    map_fields,
    normalize_entity_type,
    validate_required,
)
import pytest


def test_map_fields_known():
    raw = {"legalName": "Acme", "countryCode": "US"}
    mapped, warnings = map_fields(raw)
    assert mapped == {"legal_name": "Acme", "country_code": "US"}
    assert warnings == []


def test_map_fields_unknown():
    raw = {"legalName": "Acme", "countryCode": "US", "foo": "bar"}
    mapped, warnings = map_fields(raw)
    assert "foo" not in mapped
    assert len(warnings) == 1


def test_validate_required_missing():
    with pytest.raises(ValueError, match="Missing required fields"):
        validate_required({"legal_name": "Acme"})


def test_validate_required_null():
    with pytest.raises(ValueError, match="cannot be null"):
        validate_required({"legal_name": "Acme", "country_code": None})


def test_normalize_entity_type_known():
    assert normalize_entity_type("MUNICIPALITY") == "municipality"


def test_normalize_entity_type_unknown():
    assert normalize_entity_type("WIDGET") == "widget"


def test_normalize_entity_type_none():
    assert normalize_entity_type(None) == "OTHER"


def test_localize_name_with_jurisdiction():
    result = localize_name("MTA", "US-NY", "US")
    # depends on your localization rules
    assert isinstance(result, str)
