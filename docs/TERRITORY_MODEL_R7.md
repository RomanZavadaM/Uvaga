# Territory model r7 — source-neutral hierarchy

## Purpose
Represent territorial scope independently from any one provider before mapping provider-specific oblast/raion/hromada/locality identifiers.

## Model
- `TerritoryKey = level + core code`.
- `TerritoryNode = key + Ukrainian name + optional parent`.
- Levels: `Country → Region → Raion → Hromada → Locality`.
- `Region` deliberately means the first administrative level and can represent an oblast, ARC, or a city with equivalent special status.
- `Locality` means a settlement/locality below administrative layers; it does **not** automatically mean KATOTTG category `C` or “city”.
- Country is the only root without a parent.
- A child parent must already exist and be at a broader level.
- Codes are deliberately generic; provider-specific IDs are mapped later rather than becoming core identity.

## Predeclared acceptance
1. Country → region → raion → hromada → locality chain works.
2. A special-status city can be represented at Region level without being misclassified as a Locality.
3. Non-country orphan is rejected.
4. Same/deeper-level parent is rejected.
5. Invalid/duplicate keys fail closed.
6. Descendant lookup works across multiple levels.
7. Existing core/source/parser tests remain green.

## Limits
This is **not** the official current Ukrainian administrative directory. Test names/codes are fixtures. Before production, import and verify a canonical versioned KATOTTG dataset and explicit provider-ID mappings. KATOTTG object categories remain a separate concern from this hierarchy.
