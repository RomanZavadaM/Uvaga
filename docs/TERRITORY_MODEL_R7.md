# Territory model r7 — source-neutral hierarchy

Purpose: represent territorial scope independently from any one provider before mapping oblast/raion/hromada/city identifiers.

## Model
- TerritoryKey = level + source-neutral code.
- TerritoryNode = key + Ukrainian name + optional parent.
- Levels: country, oblast, raion, hromada, city.
- Country is the only root without a parent.
- A child parent must already exist and be at a broader level.
- Codes are deliberately generic; provider-specific IDs will be mapped later instead of becoming the core identity automatically.

## Predeclared acceptance
1. Country → oblast → raion → hromada chain works.
2. Non-country orphan is rejected.
3. Same/deeper-level parent is rejected.
4. Invalid/duplicate keys fail closed.
5. Descendant lookup works across multiple levels.
6. Existing r1/r2/r4 tests remain green.

## Limits
This is not the official Ukrainian administrative directory. Test names/codes beyond UA and UA-46 are synthetic placeholders. Before production, import and verify a canonical territorial dataset and explicit provider-ID mappings. City semantics vary and are not inferred by this generic hierarchy.
