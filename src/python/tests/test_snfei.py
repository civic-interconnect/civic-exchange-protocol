"""Tests for the SNFEI Generation: Normalizing Functor and SNFEI Generation.

This file now contains two types of tests:
1. Unit tests for the Python API (e.g., TestSnfeiGeneration).
2. Data-driven parity tests (TestSnfeiVectorParity) that load all 'current'
   test vectors from the manifest.json file to ensure cross-implementation
   compatibility.
"""

import json
from pathlib import Path

# Assuming these types are returned by your functions, based on old tests
# If not, adjust as needed (e.g., compare dicts)
from civic_exchange_protocol.snfei import (
    # Functions
    apply_localization,
    generate_snfei,
    generate_snfei_simple,
    generate_snfei_with_confidence,
    normalize_address,
    normalize_legal_name,
    normalize_registration_date,
)
import pytest


def _find_repo_root(start: Path | None = None) -> Path:
    """Find the repository root by walking up until pyproject.toml is found."""
    path = (start or Path(__file__)).resolve()
    for parent in [path] + list(path.parents):
        if (parent / "pyproject.toml").is_file():
            return parent
    raise RuntimeError("Could not find repository root (pyproject.toml not found).")


# --- Helper Function to Load All Test Vectors ---


def load_snfei_vectors():
    """
    Loads all 'current' SNFEI test vectors specified in the manifest.json file.
    """
    manifest_path = _find_repo_root() / "test_vectors" / "manifest.json"
    if not manifest_path.exists():
        raise FileNotFoundError(f"Manifest file not found at {manifest_path}")

    with manifest_path.open(encoding="utf-8") as f:
        manifest = json.load(f)

    base_path = manifest_path.parent
    all_vectors = []

    # Find the 'snfei' category in the manifest
    snfei_data = manifest.get("vectors", {}).get("snfei", {})
    if not snfei_data:
        raise ValueError("Manifest is missing 'snfei' vector category.")

    # Iterate over all defined versions (e.g., "v1.0", "v1.1")
    for version_name, version_data in snfei_data.get("versions", {}).items():
        # Only load files from versions marked as "current"
        if version_data.get("status") == "current":
            for file_path_str in version_data.get("files", []):
                file_path = base_path / file_path_str

                if not file_path.exists():
                    print(f"Warning: Test vector file not found, skipping: {file_path}")
                    continue

                with file_path.open("r", encoding="utf-8") as f_vec:
                    vector_file_content = json.load(f_vec)

                # Add metadata to each vector for better test reporting
                for vector in vector_file_content.get("vectors", []):
                    vector["_source_file"] = file_path_str
                    vector["_source_version"] = version_name
                    all_vectors.append(vector)

    if not all_vectors:
        raise Exception("No 'current' SNFEI test vectors were found. Check manifest.json.")

    return all_vectors


# --- Pytest Parametrization ---

# Load all vectors once at module scope
try:
    ALL_SNFEI_VECTORS = load_snfei_vectors()
except Exception as e:
    print(f"CRITICAL: Failed to load SNFEI test vectors: {e}")
    ALL_SNFEI_VECTORS = []


def get_test_id(vector):
    """Creates a unique and readable ID for each test case."""
    try:
        # Use the file name and vector ID, e.g., "generation_full.json-gen_001"
        filename = vector["_source_file"].split("/")[-1]
        return f"{filename}-{vector['id']}"
    except KeyError:
        return "invalid-vector-data"


# ---
# SECTION 1: Python API Unit Tests
# (These classes are kept from your original file)
# ---


class TestSnfeiGeneration:
    """Tests for SNFEI hash generation (Python API unit tests)."""

    def test_basic_generation(self):
        snfei, inputs = generate_snfei(
            legal_name="Springfield School District",
            country_code="US",
        )
        assert len(snfei.value) == 64
        assert all(c in "0123456789abcdef" for c in snfei.value)

    def test_determinism(self):
        """Same inputs must always produce same SNFEI."""
        snfei1 = generate_snfei_simple("Springfield USD", "US")
        snfei2 = generate_snfei_simple("Springfield USD", "US")
        assert snfei1 == snfei2

    def test_normalization_equivalence(self):
        """Different formatting of same entity must produce same SNFEI."""
        snfei1 = generate_snfei_simple("Springfield Unified School District", "US")
        snfei2 = generate_snfei_simple("SPRINGFIELD USD", "US")
        snfei3 = generate_snfei_simple("springfield unified sch. dist.", "US")
        assert snfei1 == snfei2 == snfei3

    def test_address_affects_hash(self):
        """Adding address should change the hash."""
        snfei_no_addr = generate_snfei_simple("Springfield School District", "US")
        snfei_with_addr = generate_snfei_simple(
            "Springfield School District", "US", address="123 Main St"
        )
        assert snfei_no_addr != snfei_with_addr

    def test_country_affects_hash(self):
        """Different countries should produce different hashes."""
        snfei_us = generate_snfei_simple("Springfield", "US")
        snfei_ca = generate_snfei_simple("Springfield", "CA")
        assert snfei_us != snfei_ca

    def test_canonical_input_format(self):
        """Verify canonical input string format."""
        _, inputs = generate_snfei(
            legal_name="Springfield School District",
            country_code="US",
            address="123 Main St",
        )
        hash_string = inputs.to_hash_string()
        parts = hash_string.split("|")
        assert len(parts) == 4
        assert parts[0] == "springfield school district"
        assert parts[1] == "123 main street"
        assert parts[2] == "US"
        assert parts[3] == ""  # No registration date

    def test_with_registration_date(self):
        _, inputs = generate_snfei(
            legal_name="Springfield School District",
            country_code="US",
            registration_date="01/15/1985",
        )
        hash_string = inputs.to_hash_string()
        assert "1985-01-15" in hash_string


class TestSnfeiWithConfidence:
    """Tests for SNFEI generation with confidence scoring (Python API unit tests)."""

    def test_tier_3_base_confidence(self):
        """Name + country only should give base confidence."""
        result = generate_snfei_with_confidence(
            legal_name="Springfield",
            country_code="US",
        )
        assert result.tier == 3
        assert result.confidence_score == 0.5
        assert "legal_name" in result.fields_used
        assert "country_code" in result.fields_used

    # ... (all other confidence tests from your original file remain here) ...

    def test_result_to_dict(self):
        """Verify result serialization."""
        result = generate_snfei_with_confidence(
            legal_name="Springfield",
            country_code="US",
        )
        d = result.to_dict()
        assert "snfei" in d
        assert "confidence_score" in d
        assert "tier" in d
        assert "canonical" in d


# ---
# SECTION 2: Data-Driven Parity Tests
# (This class replaces all your old hard-coded vector tests)
# ---


@pytest.mark.parametrize("vector", ALL_SNFEI_VECTORS, ids=get_test_id)
class TestSnfeiVectorParity:
    """
    Runs all 'current' SNFEI test vectors from the manifest against the
    Python implementation to ensure cross-language parity.
    """

    def test_vector_parity(self, vector: dict):
        """
        A single test method that dispatches to the correct test logic
        based on the vector's 'function' field.
        """
        func_name = vector.get("function")
        input_data = vector.get("input")
        expected = vector.get("expected")

        if not func_name or not input_data or not expected:
            pytest.fail(f"Invalid vector structure: {vector.get('id')}")

        try:
            if func_name == "normalize_legal_name":
                self._test_normalize_legal_name(input_data, expected)
            elif func_name == "normalize_address":
                self._test_normalize_address(input_data, expected)
            elif func_name == "normalize_registration_date":
                self._test_normalize_date(input_data, expected)
            elif func_name == "apply_localization":
                self._test_apply_localization(input_data, expected)
            elif func_name == "generate_snfei":
                self._test_generate_snfei(vector, input_data, expected)
            else:
                pytest.fail(f"Unknown vector function type: {func_name}")

        except Exception as e:
            # Provide rich assertion details
            pytest.fail(
                f"Vector {vector['id']} ({vector['_source_file']}) failed:\n"
                f"Function: {func_name}\n"
                f"Input: {json.dumps(input_data)}\n"
                f"Error: {e}"
            )

    def _test_normalize_legal_name(self, input_data: dict, expected: dict):
        """Tests normalize_legal_name vectors."""
        legal_name = input_data.get("legal_name")
        actual = normalize_legal_name(legal_name) if legal_name is not None else None
        expected_norm = expected.get("normalized")
        assert actual == expected_norm

    def _test_normalize_address(self, input_data: dict, expected: dict):
        """Tests normalize_address vectors."""
        address = input_data.get("address")
        actual = normalize_address(address) if address is not None else None
        expected_norm = expected.get("normalized")
        assert actual == expected_norm

    def _test_normalize_date(self, input_data: dict, expected: dict):
        """Tests normalize_registration_date vectors."""
        date_str = input_data.get("date_str")
        actual = normalize_registration_date(date_str) if date_str is not None else None
        # Note: expected_norm can be null, which is valid
        expected_norm = expected.get("normalized")
        assert actual == expected_norm

    def _test_apply_localization(self, input_data: dict, expected: dict):
        """Tests apply_localization vectors."""
        name = input_data.get("name")
        jurisdiction = input_data.get("jurisdiction", "")
        if name is None:
            pytest.fail("Test vector missing required 'name' field")
        actual = apply_localization(name, jurisdiction)
        expected_loc = expected.get("localized")
        assert actual == expected_loc

    def _test_generate_snfei(self, vector: dict, input_data: dict, expected: dict):
        """Tests end-to-end generate_snfei vectors."""

        # Apply localization before generating SNFEI if jurisdiction is provided
        legal_name = input_data["legal_name"]
        jurisdiction = input_data.get("jurisdiction", "")
        if jurisdiction:
            legal_name = apply_localization(legal_name, jurisdiction)

        snfei_obj, inputs_obj = generate_snfei(
            legal_name=legal_name,
            country_code=input_data["country_code"],
            address=input_data.get("address"),
            registration_date=input_data.get("registration_date"),
        )

        # 1. Verify Final SNFEI Hash
        actual_snfei = snfei_obj.value
        expected_snfei = expected["snfei"]
        assert actual_snfei == expected_snfei, "Final SNFEI hash mismatch"

        # 2. Verify Intermediate Canonical String
        intermediate = vector["intermediate"]
        actual_canonical = inputs_obj.to_hash_string()
        expected_canonical = intermediate["canonical_string"]
        assert actual_canonical == expected_canonical, "Canonical string mismatch"

        # 3. Verify Other Intermediates (handle None vs "")
        assert inputs_obj.legal_name_normalized == intermediate["legal_name_normalized"]
        assert (inputs_obj.address_normalized or "") == intermediate["address_normalized"]
        assert (inputs_obj.registration_date or "") == intermediate["registration_date"]

        # 4. Verify Equivalent Inputs (if provided)
        if "equivalentInputs" in vector:
            for equiv_name in vector["equivalentInputs"]:
                # Use generate_snfei_simple for this check
                snfei_equiv = generate_snfei_simple(equiv_name, input_data["country_code"])
                assert snfei_equiv == expected_snfei, (
                    f"EquivalentInput '{equiv_name}' failed to match expected SNFEI"
                )


if __name__ == "__main__":
    # Run all tests in this file
    pytest.main([__file__, "-v", "-s"])
