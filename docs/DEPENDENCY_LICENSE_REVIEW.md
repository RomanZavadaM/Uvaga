# Dependency / license review — r18 exact locked graph

**Date:** 2026-11-02  
**Evidence run:** `41422471078`  
**Inventory artifact:** `13130467223` — `Uvaga-r18-dependency-license-inventory`  
**Artifact ZIP SHA256:** `afc0c0a75986fc68ea836a2cf7778353735dfdcb8bb24e5b99368e23ca351f7f`  
**Evidence Cargo.lock SHA256:** `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`

## Inventory result

- resolved packages: **408**;
- local/path proprietary packages: **2** (`uvaga-core`, `uvaga-app`);
- crates.io registry packages: **406**;
- external git/non-registry packages: **0**;
- packages without Cargo `license` expression: **0**;
- packages relying only on `license_file`: **0**.

The artifact contains:
- `cargo-metadata.json` — raw locked Cargo metadata;
- `license-inventory.tsv` — one record per package;
- `license-summary.json` — counts and anomalies;
- `EVIDENCE.txt` — exact source run/artifact/lock anchor.

## Observed external license families

The locked graph contains expressions based on:

- MIT;
- Apache-2.0;
- Apache-2.0 WITH LLVM-exception;
- BSD-3-Clause;
- ISC;
- Zlib;
- Unicode-3.0;
- Unlicense;
- 0BSD;
- MIT-0;
- CC0-1.0;
- MPL-2.0.

Two versions of `r-efi` declare `MIT OR Apache-2.0 OR LGPL-2.1-or-later`; because this is an OR expression, the graph is not dependent on accepting LGPL as the only available license choice. No package in this inventory is LGPL-only, GPL-only or AGPL-only.

`unicode-ident` declares `(MIT OR Apache-2.0) AND Unicode-3.0`, so both the selected MIT/Apache branch and Unicode-3.0 obligations must be respected.

## MPL-2.0 transitive crates

Five packages are MPL-2.0 only:

| Crate | Version | Dependency path / source | Upstream source |
| --- | --- | --- | --- |
| `cssparser` | `0.37.0` | Tauri → tauri-utils → dom_query | https://github.com/servo/rust-cssparser |
| `cssparser-macros` | `0.7.1` | Tauri → tauri-utils → dom_query → cssparser | https://github.com/servo/rust-cssparser |
| `dtoa-short` | `0.3.5` | Tauri → tauri-utils → dom_query → cssparser | https://github.com/upsuper/dtoa-short |
| `option-ext` | `0.2.0` | Tauri → dirs → dirs-sys | https://github.com/soc/option-ext |
| `selectors` | `0.38.0` | Tauri → tauri-utils → dom_query | https://github.com/servo/stylo |

Mozilla's MPL 2.0 documentation states that MPL's copyleft is file-level, that MPL-covered code may be combined with proprietary code in a Larger Work, and that distribution of executable form requires informing recipients how to obtain the MPL-covered Source Code Form. See:
- https://www.mozilla.org/MPL/2.0/FAQ/
- https://www.mozilla.org/MPL/2.0/

No Uvaga change modifies these third-party crates. Before any public production release, the release package/notices must preserve required notices and provide a reasonable source-availability path for the exact MPL-covered versions.

## Current classification

**Inventory: PASS.** The exact r18 locked graph was collected and read back successfully.

**License policy gate: PENDING.** This document is not legal advice and does not itself approve every license for future distribution. The next technical step is an explicit machine-enforced license/source policy (for example cargo-deny) based on this reviewed inventory, followed by generation/review of distributable third-party notices/license texts before a release.

## Safety / scope

This review changes no runtime behavior and authorizes no live source adapter, credentials, notifications, production deployment, signing or release.
