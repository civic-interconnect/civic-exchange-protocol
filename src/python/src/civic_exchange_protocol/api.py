"""CEP Entity Canonicalization Service.

This service generates the Canonical String and Entity Hash for a CEP Entity Record, enforcing strict field ordering and temporal rules.

Dependencies: fastapi, uvicorn, pydantic, hashlib, datetime, decimal
To run: uvicorn cep_entity_service:app --reload
"""

from datetime import date, datetime
import hashlib

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel, Field

# --- 1. CORE CONFIGURATION AND CANONICALIZATION RULES ---

# Rule 1.1: The Canonical Attribute Order (CAOS)
# NOTE: Temporal fields are included here based on the specification order.
CANONICAL_ATTRIBUTE_ORDER = [
    "entityUei",
    "recordId",
    "attestingUei",
    "attestationTimestamp",
    "statusEffectiveDate",
    "legalStatus",
    "legalName",
    "taxId",
    "physicalAddressLine1",
    "physicalAddressCity",
    "physicalAddressPostalCode",
    "isGovernment",
    "statusTerminationDate",
    "statusSuspensionDate",
    "naicsCode",
]

# --- 2. DATA MODEL (Pydantic Schema Validation) ---


class EntityPayload(BaseModel):
    """Defines the structure for the CEP Entity Record."""

    # Section 1: Identity and Attestation
    entity_uei: str = Field(..., pattern=r"^[A-Z0-9]{12}$", alias="entityUei")
    record_id: str = Field(..., max_length=64, alias="recordId")
    attesting_uei: str = Field(..., pattern=r"^[A-Z0-9]{12}$", alias="attestingUei")
    attestation_timestamp: datetime = Field(
        ..., description="ISO 8601 UTC with microsecond precision.", alias="attestationTimestamp"
    )

    # Section 2: Temporal Status and Governance (The critical fields)
    # Section 2: Temporal Status and Governance (The critical fields)
    status_effective_date: datetime = Field(
        ...,
        description="The 'As-of' date/time when this record became valid.",
        alias="statusEffectiveDate",
    )
    status_termination_date: datetime | None = Field(
        None,
        description="The date/time the record ceased to be valid (omitted if null).",
        alias="statusTerminationDate",
    )
    legal_status: str = Field(
        ..., description="e.g., ACTIVE, DISSOLVED, SUSPENDED.", alias="legalStatus"
    )
    status_suspension_date: datetime | None = Field(
        None,
        description="Date/time the entity was suspended (omitted if null).",
        alias="statusSuspensionDate",
    )
    # Section 3: Core Attributes
    legal_name: str = Field(..., max_length=256, alias="legalName")
    tax_id: str = Field(..., max_length=32, alias="taxId")
    physical_address_line1: str = Field(..., max_length=128, alias="physicalAddressLine1")
    physical_address_city: str = Field(..., max_length=64, alias="physicalAddressCity")
    physical_address_postal_code: str = Field(..., max_length=16, alias="physicalAddressPostalCode")
    is_government: bool = Field(
        ..., description="True if a recognized government body.", alias="isGovernment"
    )
    naics_code: str | None = Field(None, max_length=10, alias="naicsCode")

    class Config:
        """Pydantic model configuration for JSON serialization and schema examples."""

        json_encoders = {datetime: lambda v: v.isoformat().replace("+00:00", "Z")}
        populate_by_name = True
        # Example for API documentation
        schema_extra = {
            "example": {
                "entityUei": "1A2B3C4D5E6F",
                "recordId": "CEP-2025-001",
                "attestingUei": "GOV-0000000001",
                "attestationTimestamp": "2025-11-27T17:52:30.123456Z",
                "statusEffectiveDate": "2024-01-01T00:00:00Z",
                # statusTerminationDate is None/omitted, implying current validity
                "legalStatus": "ACTIVE",
                "statusSuspensionDate": None,  # Omitted
                "legalName": "Acme Data Solutions LLC",
                "taxId": "99-1234567",
                "physicalAddressLine1": "123 Main St.",
                "physicalAddressCity": "Springfield",
                "physicalAddressPostalCode": "62704",
                "isGovernment": False,
                "naicsCode": "541512",
            }
        }


# --- 3. CANONICALIZATION CORE LOGIC ---


# Helper function to ensure microsecond precision formatting
def _format_datetime(dt: datetime) -> str:
    """Format datetime to ISO 8601 UTC with mandatory six fractional seconds and 'Z'."""
    dt_str = dt.isoformat().replace("+00:00", "Z")

    # Ensure exactly six digits after the decimal point
    if "." not in dt_str:
        return dt_str.replace("Z", ".000000Z")

    fractional_part = dt_str.split(".")[1].split("Z")[0]
    if len(fractional_part) < 6:
        return dt_str.replace("Z", fractional_part + "0" * (6 - len(fractional_part)) + "Z")

    return dt_str


def generate_canonical_string(data: EntityPayload) -> str:
    """Generate the pipe-delimited Canonical String (C-String) based on CAOS."""
    parts = []

    # Convert Pydantic model to a dictionary. Use by_alias=False to use the Python field names.
    # Exclude None is handled explicitly in the loop for consistency with CEP Rule 1.2
    data_dict = data.dict(exclude_none=False)

    for field_name in CANONICAL_ATTRIBUTE_ORDER:
        value = data_dict.get(field_name)

        # Rule 1.2: Field Omission (Null/Empty Exclusion)
        if value is None or (isinstance(value, str) and value == ""):
            continue

        formatted_value = ""

        # Specific rule for microsecond precision on attestationTimestamp
        if field_name == "attestationTimestamp":
            if not isinstance(value, datetime):
                raise TypeError(f"attestationTimestamp must be datetime, got {type(value)!r}")
            formatted_value = _format_datetime(value)

        # Standard ISO 8601 for other temporal fields
        elif field_name in ["statusEffectiveDate", "statusTerminationDate", "statusSuspensionDate"]:
            # These are date-like; accept both date and datetime
            if not isinstance(value, datetime | date):
                raise TypeError(f"{field_name} must be date/datetime, got {type(value)!r}")
            formatted_value = value.isoformat().replace("+00:00", "Z")

        # Standard Boolean Formatting
        elif field_name == "isGovernment":
            formatted_value = "true" if value else "false"

        # Standard String/Integer Fields
        else:
            formatted_value = str(value)

        parts.append(formatted_value)

    # Rule 1.3: Join all parts with the pipe delimiter
    return "|".join(parts)


def generate_entity_hash(c_string: str) -> str:
    """Generate the final SHA-256 Entity Hash."""
    return hashlib.sha256(c_string.encode("utf-8")).hexdigest()


# --- 4. API ENDPOINT DEFINITION ---

app = FastAPI(
    title="CEP Entity Canonicalization Service",
    description="Microservice for generating the cryptographic Entity Hash for the Entity Record.",
    version="1.0.0",
)


class CanonicalResponse(BaseModel):
    """Defines the structure of the API's successful output."""

    c_string: str = Field(
        ..., description="The Canonical String (C-String) used as the hash input."
    )
    entity_hash: str = Field(..., description="The final 64-character SHA-256 Entity Hash.")


@app.post("/api/v1/entity/canonicalize", response_model=CanonicalResponse, status_code=200)
async def canonicalize_entity(payload: EntityPayload):
    """Receives an Entity Record, performs canonical serialization (CAOS), and returns the cryptographic entity hash.

    Performs canonical serialization (CAOS) on the entity record.
    """
    try:
        # 1. Generate the Canonical String (C-String)
        c_string = generate_canonical_string(payload)

        # 2. Generate the Entity Hash
        entity_hash = generate_entity_hash(c_string)

        return CanonicalResponse(c_string=c_string, entity_hash=entity_hash)

    except Exception as e:
        print(f"Error during canonicalization: {e}")
        # In a production system, detailed logs would be captured here.
        raise HTTPException(status_code=500, detail="Internal canonicalization error.") from e
