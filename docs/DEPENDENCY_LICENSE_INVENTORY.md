# Dependency / license inventory — predeclared acceptance

**Date:** 2026-11-02  
**Scope:** evidence-only dependency/license inventory for the current r18 Windows synthetic build.  
**Product behavior:** no change.

## Purpose

`THIRD_PARTY_NOTICES.md` still describes Rust/Tauri dependencies as future candidates. That statement is now stale because the Tauri shell and its Rust dependency graph are already part of the prototype.

Before changing third-party notices or defining a license allow-list, collect reproducible evidence from the exact dependency lock used by the verified r18 test artifact.

## Evidence anchor

Use the already verified r18 artifact from GitHub Actions run `41415006414`:

- artifact: `Uvaga-Windows-Synthetic-r18`;
- artifact id: `13127022813`;
- verified `Cargo.lock` SHA256:
  `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.

The inventory workflow must fail if the downloaded lock does not match this hash.

## Acceptance criteria

1. No source/product files are changed.
2. The workflow uses the exact verified r18 `Cargo.lock`, not a newly resolved dependency graph.
3. `cargo metadata --locked` succeeds for `src-tauri/Cargo.toml`.
4. A machine-readable inventory is produced for every package in the resolved graph.
5. Each package record contains name, version, Cargo license expression, license-file value and source.
6. The report separately identifies private/path workspace packages, crates.io packages, non-registry external sources, missing license expressions, license-file-only packages and license-expression counts.
7. Raw metadata plus JSON/TSV summaries are uploaded as a GitHub Actions artifact.
8. This stage is inventory-only. It must not silently turn observed licenses into an approved allow-list.
9. After readback, human/AI review classifies the observed licenses and only then updates `THIRD_PARTY_NOTICES.md` / future CI license policy.
10. No live alert source, credentials, notifications, production deployment, signing or release work is introduced.

## Result state

Until the inventory artifact is read back and reviewed, dependency/license gate remains **PENDING**.
