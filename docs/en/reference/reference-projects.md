# Reference Projects: GitHub Data Standards

There are categories of open-source projects on GitHub that offer great examples, particularly concerning common data schemas, multi-language support, and provenance tracking.

## 1. Interoperability & Event Specifications (Cross-Platform)

These standards focus on defining a common data format to ensure different systems and languages can communicate seamlessly.
They typically define schemas and transport rules.

### CloudEvents Specification

This specification describes event data in a common way.
It is designed to simplify event declaration and delivery across services, platforms, and languages (e.g., Go, Java, Python, C#).

It's a good example of a cross-platform specification managed openly on GitHub.
It defines a mandatory set of attributes (like a base entity identifier) that must be present in every data payload.

Link: [https://github.com/cloudevents/spec](https://github.com/cloudevents/spec)

### CDEvents Specification

This is a popular specification for Continuous Delivery events, extending CloudEvents by introducing purpose and semantics to the event data.

It shows how a standard is built on top of another standard (CloudEvents), specializing the common metadata for a specific domain (CI/CD provenance and flow).

Link: [https://github.com/cdevents/spec](https://github.com/cdevents/spec)

## 2. Provenance and Data Tracking Standards

These projects relate to provenance aspects, focusing on tracking the history, inputs, and derivation of data.

### PROV-CPL (Core Provenance Library)

This is the Core Provenance Library for collecting data provenance with multiple language bindings (C/C++, Java, Python, R).
It uses the [W3C PROV](https://www.w3.org/TR/prov-overview/) standard as its foundation.

It demonstrates a multi-language implementation of a provenance standard, providing APIs to record who/what/when/where data was created, which is central to provenance.

Link: [https://github.com/ProvTools/prov-cpl](https://github.com/ProvTools/prov-cpl)

## 3. General Data Schemas and Monorepo Structure

These focus on using JSON Schema to define strict data structures and managing them in a versioned repository.

### JSON Schema Specification

This is the official specification for JSON Schema, a declarative language used to annotate and validate JSON documents.

This foundational tool is by many standards (including CloudEvents) to define specific fields and types.
This repo illustrates how a core schema standard is defined and versioned.

Link: [https://github.com/json-schema-org/json-schema-spec](https://github.com/json-schema-org/json-schema-spec)

### Consumer Data Standards (Australian DSB Schemas)

This repository holds a collection of JSON schema files derived from the Australian Consumer Data Standards, used for robust schema validation in banking and energy sectors.

This offers a practical example of a large-scale data standard implementation in a monorepo (single repository), organized by release version, providing strict, enforceable JSON schemas for real-world data exchange.

Link: [https://github.com/ConsumerDataStandardsAustralia/dsb-schema-tools](https://github.com/ConsumerDataStandardsAustralia/dsb-schema-tools)