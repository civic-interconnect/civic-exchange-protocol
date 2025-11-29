"""Attestation and cryptographic proof types for CEP records.

Every CEP record includes an attestation block that proves:
- Who attested to the record (attestor_id)
- When it was attested (attestation_timestamp)
- Cryptographic proof of integrity (proof_type, proof_value, verification_method_uri)
"""

from dataclasses import dataclass, field
from enum import Enum

from .canonical import Canonicalize, insert_if_present, insert_required
from .timestamp import CanonicalTimestamp


class ProofPurpose(Enum):
    """The purpose of a cryptographic proof."""

    ASSERTION_METHOD = "assertionMethod"
    AUTHENTICATION = "authentication"
    CAPABILITY_DELEGATION = "capabilityDelegation"

    def as_str(self) -> str:
        """Return the canonical string representation."""
        return self.value


@dataclass
class Attestation(Canonicalize):
    """Cryptographic attestation proving record authenticity and integrity.

    This structure aligns with W3C Verifiable Credentials Data Integrity.
    """

    # Verifiable ID of the entity or node attesting to this record
    attestor_id: str

    # When the attestation was created
    attestation_timestamp: CanonicalTimestamp

    # The proof algorithm identifier
    # Examples: "Ed25519Signature2020", "EcdsaSecp256k1Signature2019", "DataIntegrityProof"
    proof_type: str

    # The cryptographic signature or proof value
    proof_value: str

    # URI resolving to the public key or DID document for verification
    verification_method_uri: str

    # The purpose of the proof
    proof_purpose: ProofPurpose = field(default=ProofPurpose.ASSERTION_METHOD)

    # Optional URI to a timestamping authority or DLT anchor
    anchor_uri: str | None = None

    @classmethod
    def new(
        cls,
        attestor_id: str,
        attestation_timestamp: CanonicalTimestamp,
        proof_type: str,
        proof_value: str,
        verification_method_uri: str,
    ) -> "Attestation":
        """Create a new Attestation with required fields."""
        return cls(
            attestor_id=attestor_id,
            attestation_timestamp=attestation_timestamp,
            proof_type=proof_type,
            proof_value=proof_value,
            verification_method_uri=verification_method_uri,
        )

    def with_purpose(self, purpose: ProofPurpose) -> "Attestation":
        """Return a new Attestation with the specified proof purpose."""
        return Attestation(
            attestor_id=self.attestor_id,
            attestation_timestamp=self.attestation_timestamp,
            proof_type=self.proof_type,
            proof_value=self.proof_value,
            verification_method_uri=self.verification_method_uri,
            proof_purpose=purpose,
            anchor_uri=self.anchor_uri,
        )

    def with_anchor(self, uri: str) -> "Attestation":
        """Return a new Attestation with the specified anchor URI."""
        return Attestation(
            attestor_id=self.attestor_id,
            attestation_timestamp=self.attestation_timestamp,
            proof_type=self.proof_type,
            proof_value=self.proof_value,
            verification_method_uri=self.verification_method_uri,
            proof_purpose=self.proof_purpose,
            anchor_uri=uri,
        )

    def canonical_fields(self) -> dict[str, str]:
        """Return the canonical fields in alphabetical order."""
        fields: dict[str, str] = {}

        # Fields in alphabetical order
        insert_if_present(fields, "anchorUri", self.anchor_uri)
        insert_required(
            fields,
            "attestationTimestamp",
            self.attestation_timestamp.to_canonical_string(),
        )
        insert_required(fields, "attestorId", self.attestor_id)
        insert_required(fields, "proofPurpose", self.proof_purpose.as_str())
        insert_required(fields, "proofType", self.proof_type)
        insert_required(fields, "proofValue", self.proof_value)
        insert_required(fields, "verificationMethodUri", self.verification_method_uri)

        return fields
