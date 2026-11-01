# Source territory normalization r10

Purpose: convert provider-specific territorial IDs into source-neutral core territory keys before an observation can become an OfficialEvent.

## Pipeline
provider observation -> source_id + provider region code -> TerritoryMappingRegistry -> core TerritoryKey -> existing official source trust normalization.

## Fail-safe rules
- Unknown provider territory mapping is rejected.
- Mapping never bypasses trusted_adapter or SourceClass::Official checks.
- Aggregator observations cannot become official merely because they have a territory mapping.
- OfficialEvent.region receives the core territory code, not a provider code.
- No mapping guesses or prefix heuristics are allowed.

## Predeclared acceptance
1. Known mapped provider region produces the expected core territory.
2. Unmapped provider region fails closed.
3. Untrusted adapter remains rejected.
4. Aggregator remains rejected.
5. Provider-specific region string does not leak into core OfficialEvent.region.
6. All previous core/territory tests remain green.

## Limits
No real provider mapping dataset or live API is used. Before production, mappings must be versioned, audited and generated from documented provider metadata plus a canonical territorial dataset.
