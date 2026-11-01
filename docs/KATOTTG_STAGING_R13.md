# KATOTTG import staging r13

Purpose: validate the structural shape of a future KATOTTG import row without pretending that a cached dataset is current and without assigning object-category semantics prematurely.

## Input retained
- first level;
- second level;
- third level;
- fourth level;
- additional level;
- source category string;
- Ukrainian object name.

## Current staging contract (checked 2026-11-01)
- first level is required;
- non-empty levels must be contiguous;
- current accepted code shape is exactly `UA` + 17 ASCII digits (19 characters), matching the Ministry examples used by this importer revision;
- name/category cannot be blank;
- deepest source code is derived only after full row validation.

This is deliberately **fail-closed**. If a later Ministry artifact changes the code shape, the importer must stop and receive an explicit reviewed update instead of silently widening acceptance.

## Deliberate non-decisions
- Code length is used only as current input-shape validation; no hierarchy/category semantics are inferred from code length.
- Category values are kept opaque; this layer does not translate `O/P/H/C/X/B` into core `TerritoryLevel`.
- No real KATOTTG snapshot is bundled.
- No amendment application, CSV/XLSX/PDF parsing, network retrieval or production mapping exists.

## Predeclared acceptance
1. published four-level-shaped fixture validates;
2. published additional-level-shaped fixture validates;
3. hierarchy gap fails closed;
4. non-UA/non-numeric/short/long code fails;
5. category remains opaque and an unknown future category is retained;
6. blank name/category fail;
7. all earlier core/source/territory/relay tests stay green.

The next importer stage must operate on preserved raw artifacts with an identified Ministry base revision, amendment chain and explicit cutoff as defined by `TERRITORY_DATA_SOURCE_R12.md`.
