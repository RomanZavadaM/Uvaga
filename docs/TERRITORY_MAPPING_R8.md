# Territory mapping r8 — provider IDs do not become core IDs

Purpose: map each provider's territorial identifiers to the source-neutral territory hierarchy from r7.

## Rules
- Mapping key = source_id + provider_code.
- Target must already exist in TerritoryCatalog.
- Same provider_code from different sources is not assumed equivalent.
- One provider key cannot be silently remapped; duplicate insertion fails closed.
- Blank provenance is rejected.
- No provider-specific production mapping table is bundled yet.

## Predeclared acceptance
1. A provider code resolves to a known source-neutral territory.
2. Equal codes from different sources stay independent.
3. Unknown target territories are rejected.
4. Duplicate provider keys fail closed.
5. Blank source/code values are rejected.
6. All prior core tests remain green.

## Limits
The test code uses synthetic example provider identifiers. Before production, every real mapping must be generated from documented provider metadata and checked against a canonical Ukrainian territorial dataset. Mapping changes require explicit versioning/audit because a wrong map can misdirect alerts.
