# Changelog

All notable changes to this project will be documented in this file.

The format follows **[Keep a Changelog](https://keepachangelog.com/en/1.1.0/)**
and this project adheres to **[Semantic Versioning](https://semver.org/spec/v2.0.0.html)**.

---

## [Unreleased]

### Added
- (placeholder) Notes for the next release.

---

## [0.0.4] - 2025-11-29

### Added
- **Versioned Test Vector Schemas Directory:** Introduced versioned directory structure for test vectors (`test_vectors/schemas/v1.0/` to work with `test_vectors/snfei/v1.0/`) to help with stability and immutability of historical test runs.
- **Initial Markdown Weekly Link Checker:** Added a new GitHub action to test Markdown links. 

### Changed
- **Updated Data-driven SNFEI generation:** Integrated and improved the external, serialized JSON test vector files for canonicalization and hashing validation to facilitate cross-language parity testing.
- **Normalized Schema Versioning:** Updated and ordered the `$id` and `$schema` keywords in test vector schemas to lock the contract to a specific version.
- - **Improved Normalization Vector Sets:** Updated test files (e.g., `normalization_full.json`, etc.) that **isolate and test data normalization functions** (e.g., legal name cleanup) independently of the final hashing process - kept spaces when normalizing midname dashes, slashes, etc. 

### Architectural Improvements
- **Decoupled Testing:** Separation of concerns enforced by using distinct schemas for SNFEI Generation and specific Normalization steps. This allows implementers to test discrete functions (like address or name cleaning) without having to perform (and debug) a full end-to-end hash.
- **Platform-Agnostic Validation:** The use of declarative JSON vector files tested with Rust and Python enable additional implementations (e.g. Java, C#) to use the same test input/output files for validation, making cross-platform compliance checks simpler and more reliable.
---

## [0.0.3] - 2025-11-28

### Added
- Initial SNFEI generation
  
---

## [0.0.2] - 2025-11-28

### Added
- Initial cli (placeholder)
  
---

## [0.0.1] - 2025-11-28

### Added
- Initial draft (pre-alpha).
- Baseline Python/Rust workspace structure.
- Canonical hash rules and preliminary schemas.
- Documentation scaffolding.
  
---

## Notes on versioning and releases

- **SemVer policy**
  - **MAJOR** - breaking API/schema or CLI changes.
  - **MINOR** - backward-compatible additions and enhancements.
  - **PATCH** - documentation, tooling, or non-breaking fixes.
- Versions are driven by git tags via `setuptools_scm`.
  Tag the repository with `vX.Y.Z` to publish a release.
- Documentation and badges are updated per tag and aliased to **latest**.

[Unreleased]: https://github.com/civic-interconnect/civic-exchange-protocol/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/civic-interconnect/civic-exchange-protocol/releases/tag/v0.0.1
