# Civic Exchange Protocol (CEP)

[![PyPI](https://img.shields.io/pypi/v/civic-exchange-protocol.svg)](https://pypi.org/project/civic-exchange-protocol/)
[![Python versions](https://img.shields.io/pypi/pyversions/civic-exchange-protocol.svg)](https://pypi.org/project/civic-exchange-protocol/)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0)
[![CI Status](https://github.com/civic-interconnect/civic-exchange-protocol/actions/workflows/ci-python.yml/badge.svg)](https://github.com/civic-interconnect/civic-exchange-protocol/actions/workflows/ci-python.yml)
[![Docs](https://img.shields.io/badge/docs-mkdocs--material-blue)](https://civic-interconnect.github.io/civic-exchange-protocol/)
[![Security Policy](https://img.shields.io/badge/security-policy-orange)](SECURITY.md)
[![Link Check](https://github.com/civic-interconnect/civic-exchange-protocol/actions/workflows/weekly_link_checker.yml/badge.svg)](https://github.com/civic-interconnect/civic-exchange-protocol/actions/workflows/weekly_link_checker.yml)

> Interoperable data standards for describing entities, relationships, and value exchanges across civic systems.

## Overview

The Civic Exchange Protocol defines a coherent, verifiable way to describe:

- **Entities** (organizations, agencies, districts, people)  
- **Relationships** (grant awards, contracts, reporting relationships)  
- **Exchanges** of value (payments, disbursements, transfers)  

CEP records are:

- JSON Schema–validated  
- Fully typed  
- Deterministic and versioned  
- Extensible across jurisdictions and data ecosystems  
- Designed for cross-system interoperability

Documentation: <https://civic-interconnect.github.io/civic-exchange-protocol/>


## Repository Structure

```text
/
├── schema/                     # Official CEP JSON Schemas
│   ├── cep.entity.schema.json
│   ├── cep.relationship.schema.json
│   ├── cep.exchange.schema.json
│   └── cep.entity.identifier-scheme.schema.json
│
├── vocabulary/                 # Versioned controlled vocabularies
│   ├── entity-type.v1.0.0.json
│   ├── exchange-type.v1.0.0.json
│   ├── exchange-role.v1.0.0.json
│   ├── party-role.v1.0.0.json
│   ├── value-type.v1.0.0.json
│   └── resolution-method.v1.0.0.json
│
├── src/python/                 # Python reference implementation
│   ├── src/civic_exchange_protocol/
│   └── tests/
│
├── src/rust/                   # Rust reference implementation
│   ├── cep-core/
│   ├── cep-entity/
│   ├── cep-exchange/
│   └── cep-relationship/
│
├── docs/                       # MkDocs documentation site
└── .github/workflows/          # CI, Docs deploy, PyPI release
```

## Python Reference Implementation

Install from PyPI:

```bash
pip install civic-exchange-protocol
```

CLI entrypoint:

```bash
cx --help
```

Includes:

- Pydantic models for Entity / Relationship / Exchange  
- Deterministic record hashing  
- Attestation helpers  
- Built-in JSON Schema validator  
- Test vectors for conformance  

---

## Rust Reference Implementation

The repository includes a Rust workspace with:

- `cep-core`  
- `cep-entity`  
- `cep-exchange`  
- `cep-relationship`  

To build:

```bash
cd src/rust
cargo build
```

Each crate inherits version, license, and repository metadata from the workspace root.

## Schemas

Official schemas live under **/schema** and are published with stable URLs such as:

```text
https://raw.githubusercontent.com/civic-interconnect/civic-exchange-protocol/main/schema/cep.entity.schema.json
```

Documentation includes a browser-embedded validator using Ajv.


## Security Policy

We support responsible disclosure through GitHub’s **Private Vulnerability Report** feature.

See: [SECURITY.md](SECURITY.md)

## Contributing

- See [CONTRIBUTING.md](./CONTRIBUTING.md)
- See [DEVELOPER.md](./DEVELOPER.md)
