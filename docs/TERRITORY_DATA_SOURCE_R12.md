# Territory data source policy r12 — KATOTTG

Date checked: 2026-11-01  
Scope: source policy only; no production import and no live territorial dataset is bundled.

## Authoritative basis

The project treats the Ministry of Development of Communities and Territories of Ukraine as the authoritative source for KATOTTG.

Base codifier:
- Order No. 290 of 26.11.2020, in the edition of Order No. 48 of 19.01.2024.
- Published codifier PDF:
  https://mindev.gov.ua/storage/app/sites/1/uploaded-files/kodifikator-19-06-24.pdf
- The published table contains first/second/third/fourth/additional level, object category and object name.

The 2024 publication is **not sufficient by itself as the current 2026 state**. The Ministry publishes later amending orders.

## Confirmed amendment examples

The following amendments were confirmed during the 2026-11-01 source audit. This list is evidence that the codifier changes over time; it is **not a claim that these are all amendments up to the present date**.

- Order No. 872 dated 16.05.2025:
  https://mindev.gov.ua/storage/app/sites/1/uploaded-files/nakaz-872-vid-16-05-2025.pdf
- Order No. 276 dated 12.02.2026:
  https://mindev.gov.ua/storage/app/sites/1/uploaded-files/nakaz-ministerstva-rozvitku-gromad-ta-teritorii-ukrayini-vid-12-liutogo-2026-r-no-276-pro-vnesennia-zmin-do-kodifikatora-administrativno-teritorialnix-odinic-ta-teritorii-teritorialnix-gromad.pdf
- Order No. 459 dated 09.03.2026:
  https://mindev.gov.ua/storage/app/sites/1/uploaded-files/nakaz-459-09.pdf
- Order No. 1193 dated 22.06.2026:
  https://mindev.gov.ua/storage/app/sites/1/uploaded-files/nakaz-1193-vid-22062026.pdf

The Ministry also issued other amendments in 2025. Therefore production must never infer “current KATOTTG” from one cached PDF or from the examples above.

## Cutoff-based reproducibility rule

Every accepted production catalog snapshot must be reproducible to an explicit cutoff date.

For a snapshot with cutoff `YYYY-MM-DD`, record:
1. exact base codifier revision;
2. all Ministry amendment orders discovered and applied up to that cutoff;
3. source URL for every raw artifact;
4. retrieval timestamp;
5. checksum of each preserved raw artifact;
6. importer version/commit;
7. generated catalog checksum;
8. validation result;
9. reviewed diff from the previous accepted catalog snapshot.

Before each new production import, re-run discovery against the Ministry source. A previously recorded amendment list is historical evidence, not proof that no later amendment exists.

## Project rules

1. Do not hard-code a copied 2021/2024 table and call it current.
2. Production territory data must be reproducible from an identified Ministry base plus the applicable amendment chain up to a recorded cutoff.
3. Provider territory IDs remain separate from KATOTTG/core IDs and are connected only through the audited mapping registry.
4. Unknown, removed, renamed or conflicting territories fail closed and require review; do not guess by name, prefix or approximate string match.
5. A provider mapping update must not silently change historical alert meaning.
6. No geolocation or personal user data is required to maintain this catalog.
7. Raw source artifacts used for an accepted snapshot must be preserved independently before the snapshot is called reproducible.

## Format caution

Official KATOTTG examples use codes such as:
- `UA01000000000013043` — category O;
- `UA01020000000022387` — category P;
- `UA01020010000048857` — category H;
- `UA01020010010075540` — category C;
- additional-level records can use other categories, including X/B in published amendments.

A documented storage field length is not sufficient evidence to infer hierarchy or category semantics from string length alone. The importer must preserve published columns/categories and validate them against the source specification.

## Auxiliary/open-data copies

Open-data portal copies can be useful as auxiliary/test material, but they are not automatically authoritative for the current production state. Verify publisher, update date, license and consistency against the Ministry source before use.

## Acceptance before production territory import

- current Ministry base identified;
- amendment discovery performed through the chosen cutoff;
- raw source artifacts preserved with checksums;
- importer has fixture tests for additions, removals, renames/category changes and duplicate/conflicting records;
- generated catalog passes parent/hierarchy and uniqueness checks;
- diff against the previous accepted snapshot is reviewed;
- rollback to the previous accepted catalog snapshot is possible;
- restore/readback of the preserved source snapshot is tested.

Until then, territory fixtures in Rust tests are synthetic and must not be presented as a complete current administrative directory of Ukraine.
