# Territory mapping r8 — provider IDs do not become core IDs

## Purpose
Map each provider's territorial identifiers to the source-neutral territory hierarchy from r7 without allowing provider naming/numbering to become core identity.

## Rules
- Mapping key = `source_id + provider_code`.
- Surrounding whitespace is trimmed at the registry boundary.
- Source IDs and provider codes otherwise remain case-sensitive and opaque; the core does not lowercase, parse or guess their meaning.
- Target must already exist in `TerritoryCatalog`.
- The same provider code from different sources is not assumed equivalent.
- One canonical provider key cannot be silently remapped; duplicate insertion fails closed.
- Blank provenance is rejected.
- No provider-specific production mapping table is bundled yet.

## Predeclared acceptance
1. A provider code resolves to a known source-neutral `Region`.
2. Surrounding whitespace cannot create a second mapping for the same key.
3. Equal provider codes from different sources stay independent.
4. Case is not silently normalized.
5. Unknown target territories are rejected.
6. Blank source/code values are rejected.
7. All prior core/relay tests remain green.

## Limits
The test code uses synthetic provider identifiers. Before production, every real mapping must be generated from documented provider metadata and checked against the accepted versioned Ukrainian territorial dataset. Mapping changes require explicit versioning/audit because a wrong map can misdirect alerts.


## Clean-rebuild gate
This PR was rebuilt onto the accepted current `main`. Integration requires fresh Rust core + relay compatibility CI on the current PR head.
