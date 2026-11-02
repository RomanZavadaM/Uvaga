# Third-party compliance bundle — predeclared acceptance

**Date:** 2026-11-02  
**Scope:** exact r18 dependency evidence; no product/runtime change.

## Goal

Build a concrete compliance-support artifact from the same verified r18 `Cargo.lock` that was used for the dependency inventory and machine policy.

## Evidence anchor

- source Actions run: `41415006414`;
- source artifact: `Uvaga-Windows-Synthetic-r18` / id `13127022813`;
- exact `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.

## Acceptance criteria

1. Download and verify the exact r18 lock before any dependency operation.
2. Run `cargo metadata --locked` against `src-tauri/Cargo.toml`.
3. Vendor the exact registry sources with `cargo vendor --locked --versioned-dirs`.
4. For every crates.io package, build an index containing package/version/license/repository and captured license/notice file paths.
5. Capture published-package files named like `LICENSE*`, `COPYING*`, `NOTICE*`, `COPYRIGHT*` or `UNLICENSE*` into a structured bundle.
6. Report packages for which no such file was found; do not silently treat them as resolved.
7. Verify the five reviewed MPL-2.0 package/version pairs are present and unchanged.
8. Create an archive containing the complete vendored source trees for those five MPL-2.0 packages.
9. Create an MPL source-availability note listing exact versions and upstream repositories.
10. Upload the index, summary, captured notices/license files, MPL source archive and evidence anchor as one GitHub Actions artifact.
11. This artifact is a prototype/release-preparation checkpoint only. A production release must preserve the equivalent bundle for its exact final lock in a non-expiring release location.
12. No live source adapter, credentials, public notifications, production deployment, signing or release is introduced.

## Result

Gate remains **PENDING** until the generated artifact is downloaded/read back and unresolved license-file cases are reviewed.
