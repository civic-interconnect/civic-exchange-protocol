# Civic Exchange Protocol (CEP) Implementation Examples

This directory provides small, self-contained example files for:
- **Entity** records  
- **Exchange** (grant/contract) records  
- **Relationship** records  
- **SNFEI test corpus** (canonical inputs and expected hashes)  

These examples serve four purposes:

1. **Demonstration** - show what valid CEP records look like.  
2. **Testing** - provide cross-language test fixtures.  
3. **Validation** - e.g., used to test the CLI command  
   ```bash
   cx validate-json --schema entity --file examples/entity/school_district_01.json
   ```
4. **Reference Implementation** - ensure all language SDKs generate identical canonical strings and SNFEI hashes.

---

## Directory Layout

```
examples/
  entity/        # Entity-level examples
  exchange/      # Grants, contracts, financial exchanges
  relationship/  # Entity-to-entity relationships
  snfei/         # SNFEI test vectors (inputs, canonical, hashes)
  README.md
```

Each subfolder contains multiple minimal, schema-valid examples.

---

## Sub-National Federated Entity Identifier (SNFEI) Test Corpus

The SNFEI test vectors include:
- **inputs.jsonl** containing messy real-world inputs  
- **canonical_expected.jsonl** containing the expected canonical strings  
- **snfei_expected.jsonl** containing the calculated final hash output  

These can be used by language SDKs to ensure correctness:

```bash
uv run python -m civic_exchange_protocol.snfei test examples/snfei/
cargo test --features snfei
dotnet test
```

---

## Contributing New Examples

If you add examples:
- keep them small  
- use realistic civic entities  
- ensure they validate against schemas  
- update SNFEI expected hashes if rules change  

---

## License

Examples are licensed under Apache 2.0 (same as the project).
