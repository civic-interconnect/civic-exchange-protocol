# Standard Evolution Policy and Versioning

This policy defines the process, requirements, and responsibilities for introducing changes to the Protocol.

## 1. Versioning System

The standard utilizes Semantic Versioning (MAJOR.MINOR.PATCH). The version number applies to the entire monorepo and its core protocol requirements.

### 1.1 MAJOR Version Change (X.0.0)

A MAJOR change is reserved for non-backwards-compatible modifications that require all certified nodes to update their serialization logic, otherwise, they will lose hash parity.

| Change Type              | Impact                                                                 | Example |
| ------------------------ | ---------------------------------------------------------------------- | ------- |
| Data Type Change         | Changing a field type (e.g., transactionAmount from number to string). |         |
| Field Order Change       | Modifying the CANONICAL_FIELD_ORDER.                                   |         |
| Cryptographic Change     | Switching from SHA-256 to a new algorithm (e.g., SHA-384).             |         |
| Mandatory Field Addition | Adding a new field to the required list in the JSON Schema.            |         |

Requirement: Requires unanimous approval from the Interconnect Standards Board (ISB) and a mandatory 6-month deprecation period.

### 1.2 MINOR Version Change (0.X.0)

A MINOR change is a backwards-compatible modification that adds new functionality or optional fields. Existing certified nodes maintain hash parity, but new nodes may need to account for new optional data.

| Change Type  | Impact   | Example   |
| ---------- | ------------- | --------------- |
| Optional Field Addition | Adding a new field to the end of the CANONICAL_FIELD_ORDER (which is omitted if null).   | Adding funding_source_project_id (optional). |
| Enum Value Addition     | Adding a new value to an existing enum (e.g., adding TEMPORARILY_SUSPENDED to transactionStatus). |                                              |
| Tooling Upgrade         | Significant upgrade to the build system (build.sh or testing dependencies).                       |                                              |

Requirement: Requires simple majority approval from the ISB (2 out of 3 votes).

### 1.3 PATCH Version Change (0.0.X)

A PATCH change is a small, fully backwards-compatible correction to documentation, tooling, or non-protocol code.

| Change Type       | Impact                                                                  | Example |
| ----------------- | ----------------------------------------------------------------------- | ------- |
| Documentation Fix | Correcting a typo in the Governance Charter.                            |         |
| Test Data Update  | Adding a new test vector to /test_data that uses only existing fields.  |         |
| Non-Core Bug Fix  | Fixing a non-critical bug in a reference implementation's example code. |         |

Requirement: Can be approved by the ISB Chair alone.

## 2. Change Submission and Review Cycle

1. Proposal Submission: The contributor submits a Pull Request (PR) against the main branch.

2. Versioning Assignment: The ISB Chair assigns the PR a tentative version bump (MAJOR, MINOR, or PATCH) based on Section 1.

3. ISB Review and Vote: The ISB reviews the proposal (focusing on need and impact) and votes according to the requirement for the assigned version type.

4. Hash Parity Enforcement: Once the vote passes, the PR is merged, and the automated CI system executes the build.sh script to confirm 100% hash parity across all five certified languages (Python, Rust, Java, C#, TypeScript). No release can occur until 100% parity is confirmed.

5. Release: The new version is tagged, and all certified node operators are notified.
