"""Tests for CEP Core module.

These tests verify that the Python implementation produces identical
canonical strings and hashes to the Rust reference implementation.
"""

from civic_exchange_protocol.core import (
    SCHEMA_VERSION,
    Attestation,
    CanonicalHash,
    CanonicalTimestamp,
    format_amount,
)
import pytest


class TestSchemaVersion:
    def test_schema_version(self):
        assert SCHEMA_VERSION == "1.0.0"


class TestCanonicalTimestamp:
    def test_canonical_format(self):
        ts = CanonicalTimestamp.parse("2025-11-28T14:30:00.123456Z")
        assert ts.to_canonical_string() == "2025-11-28T14:30:00.123456Z"

    def test_zero_microseconds(self):
        ts = CanonicalTimestamp.parse("2025-11-28T14:30:00.000000Z")
        # CRITICAL: Must preserve all 6 decimal places even when zero
        assert ts.to_canonical_string() == "2025-11-28T14:30:00.000000Z"

    def test_parse_with_offset(self):
        ts = CanonicalTimestamp.parse("2025-11-28T14:30:00.123456+00:00")
        assert ts.to_canonical_string() == "2025-11-28T14:30:00.123456Z"

    def test_ordering(self):
        earlier = CanonicalTimestamp.parse("2025-11-28T14:30:00.000000Z")
        later = CanonicalTimestamp.parse("2025-11-28T14:30:00.000001Z")
        assert earlier < later


class TestCanonicalHash:
    def test_hash_empty_string(self):
        hash_val = CanonicalHash.from_canonical_string("")
        # SHA-256 of empty string is well-known
        assert (
            hash_val.as_hex() == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        )

    def test_hash_hello(self):
        hash_val = CanonicalHash.from_canonical_string("hello")
        assert (
            hash_val.as_hex() == "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        )

    def test_from_hex_valid(self):
        hex_str = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        hash_val = CanonicalHash.from_hex(hex_str)
        assert hash_val is not None
        assert hash_val.as_hex() == hex_str

    def test_from_hex_invalid_length(self):
        assert CanonicalHash.from_hex("abc123") is None

    def test_from_hex_invalid_chars(self):
        invalid = "g" * 64
        assert CanonicalHash.from_hex(invalid) is None

    def test_uppercase_normalized(self):
        hex_str = "2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824"
        hash_val = CanonicalHash.from_hex(hex_str)
        assert hash_val is not None
        # Should be normalized to lowercase
        assert (
            hash_val.as_hex() == "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        )


class TestFormatAmount:
    def test_whole_number(self):
        assert format_amount(100.0) == "100.00"

    def test_one_decimal(self):
        assert format_amount(100.5) == "100.50"

    def test_rounding(self):
        assert format_amount(100.756) == "100.76"

    def test_zero(self):
        assert format_amount(0.0) == "0.00"

    def test_large_number(self):
        assert format_amount(1234567.89) == "1234567.89"


class TestAttestation:
    @staticmethod
    def create_test_attestation() -> Attestation:
        return Attestation.new(
            attestor_id="cep-entity:sam-uei:J6H4FB3N5YK7",
            attestation_timestamp=CanonicalTimestamp.parse("2025-11-28T14:30:00.000000Z"),
            proof_type="Ed25519Signature2020",
            proof_value="z3FXQqFwbZxKBxGxqFpCD...",
            verification_method_uri="did:web:example.gov#key-1",
        )

    def test_canonical_field_order(self):
        attestation = self.create_test_attestation()
        fields = attestation.canonical_fields()

        keys = list(fields.keys())
        # Verify alphabetical order
        assert keys == sorted(keys)
        assert keys == [
            "attestationTimestamp",
            "attestorId",
            "proofPurpose",
            "proofType",
            "proofValue",
            "verificationMethodUri",
        ]

    def test_canonical_string(self):
        attestation = self.create_test_attestation()
        canonical = attestation.to_canonical_string()

        assert canonical.startswith('"attestationTimestamp":"2025-11-28T14:30:00.000000Z"')
        assert '"attestorId":"cep-entity:sam-uei:J6H4FB3N5YK7"' in canonical
        assert '"proofPurpose":"assertionMethod"' in canonical

    def test_with_anchor(self):
        attestation = self.create_test_attestation().with_anchor(
            "https://blockchain.example.com/tx/abc123"
        )
        fields = attestation.canonical_fields()
        assert "anchorUri" in fields

    def test_hash_stability(self):
        a1 = self.create_test_attestation()
        a2 = self.create_test_attestation()

        assert a1.calculate_hash() == a2.calculate_hash()


class TestRustParity:
    """Tests that verify exact hash parity with Rust implementation."""

    def test_attestation_canonical_string_matches_rust(self):
        """The canonical string must match Rust's output exactly."""
        attestation = Attestation.new(
            attestor_id="cep-entity:sam-uei:ATTESTOR123A",
            attestation_timestamp=CanonicalTimestamp.parse("2025-11-28T14:30:00.000000Z"),
            proof_type="Ed25519Signature2020",
            proof_value="z3FXQqFwbZxKBxGxqFpCDabcdef1234567890",
            verification_method_uri="did:web:example.gov#key-1",
        )

        canonical = attestation.to_canonical_string()

        # This is the expected output from the Rust implementation
        expected = (
            '"attestationTimestamp":"2025-11-28T14:30:00.000000Z",'
            '"attestorId":"cep-entity:sam-uei:ATTESTOR123A",'
            '"proofPurpose":"assertionMethod",'
            '"proofType":"Ed25519Signature2020",'
            '"proofValue":"z3FXQqFwbZxKBxGxqFpCDabcdef1234567890",'
            '"verificationMethodUri":"did:web:example.gov#key-1"'
        )

        assert canonical == expected, f"\nGot:\n{canonical}\n\nExpected:\n{expected}"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
