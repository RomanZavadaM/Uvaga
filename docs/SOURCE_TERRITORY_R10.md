# Source territory normalization r10

Purpose: convert provider-specific territorial IDs into source-neutral core territory keys before an observation can become an OfficialEvent.

## Pipeline
provider observation -> source_id + provider region code -> TerritoryMappingRegistry -> core TerritoryKey -> existing official source trust normalization.

## Fail-safe rules
- Unknown provider territory mapping is rejected.
- Mapping never bypasses `trusted_adapter` or `SourceClass::Official`.
- Aggregator observations cannot become official merely because they have a territory mapping.
- `OfficialEvent.region` receives the core territory code, not a provider code.
- No mapping guesses or prefix heuristics are allowed.
- Surrounding transport whitespace in source/provider IDs is canonicalized before final event identity is produced.

## Predeclared acceptance
1. Known mapped provider region produces the expected core `Region` territory.
2. Unmapped provider region fails closed.
3. Untrusted adapter remains rejected.
4. Aggregator remains rejected.
5. Provider-specific region string does not leak into `OfficialEvent.region`.
6. Surrounding whitespace does not create a second logical event identity.
7. All previous core/territory/mapping/subscription/relay tests remain green.

## Limits
No real provider mapping dataset or live API is used. Before production, mappings must be versioned, audited and generated from documented provider metadata plus a canonical territorial dataset.
