# Validator

Paste a sample Civic Exchange Protocol record below and validate it
against one of the official schemas.

---

## Schema Endpoints

The validator checks your JSON against one of the official Civic Exchange Protocol schemas:

-   **Entity Schema**  
    [https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.entity.schema.json](https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.entity.schema.json)

-   **Relationship Schema**  
    [https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.relationship.schema.json](https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.relationship.schema.json)

-   **Exchange Schema**  
    [https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.exchange.schema.json](https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.exchange.schema.json)

-   **Identifier Scheme Vocabulary**  
    [https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.entity.identifier-scheme.schema.json](https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schemas/cep.entity.identifier-scheme.schema.json)

---

## Step 1. Choose schema

<select id="schema-select">
  <option selected value="entity">Entity (cep.entity.schema.json)</option>
  <option value="relationship">Relationship (cep.relationship.schema.json)</option>
  <option value="exchange">Exchange (cep.exchange.schema.json)</option>
</select>

---

## Step 2. Input JSON

<textarea id="data-to-validate" rows="18" style="width: 100%; font-family: monospace;">
{
  "schemaVersion": "1.0.0",
  "verifiableId": "cep-entity:sam-uei:J6H4FB3N5YK7",
  "identifiers": {
    "samUei": "J6H4FB3N5YK7",
    "snfei": "d41d8cd98f00b204e9800998ecf8427ed41d8cd98f00b204e9800998ecf8427e",
    "additionalSchemes": [
      {
        "schemeUri": "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/identifier-scheme.json#us-mn-district-id",
        "value": "ISD-0123"
      },
      {
        "schemeUri": "https://opencivicdata.org/id/division",
        "value": "ocd-division/country:us/state:mn/school_district:123"
      }
    ]
  },
  "legalName": "Springfield Public School District 123",
  "legalNameNormalized": "springfield public school district 123",
  "entityTypeUri": "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/entity-type.json#school-district",
  "jurisdictionIso": "US-MN",
  "status": {
    "statusCode": "ACTIVE",
    "statusEffectiveDate": "2001-07-01",
    "statusTerminationDate": null,
    "successorEntityId": null
  },
  "naicsCode": null,
  "resolutionConfidence": {
    "score": 1.0,
    "methodUri": "https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/vocabulary/resolution-method.json#authoritative-sam-sync",
    "sourceRecordCount": 1
  },
  "attestation": {
    "attestorId": "cep-entity:sam-uei:EDFEDERAL0001",
    "attestationTimestamp": "2025-11-28T15:30:45.123456Z",
    "proofType": "Ed25519Signature2020",
    "proofValue": "BASE64_SIGNATURE_VALUE_HERE",
    "verificationMethodUri": "https://keys.civic-interconnect.org/attestors/edfederal-node-1#primary-key",
    "proofPurpose": "assertionMethod",
    "anchorUri": null
  },
  "previousRecordHash": null,
  "revisionNumber": 1
}
</textarea>

---

## Step 3. Validation Result

<div id="validation-result" style="font-weight: bold; padding: 10px; border: 1px solid #ccc;">
  (Waiting for input...)
</div>
