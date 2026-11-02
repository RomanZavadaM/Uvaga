# Third-party license texts — cargo-about acceptance

**Date:** 2026-11-02  
**Scope:** exact r18 locked dependency graph; evidence/release-preparation only.  
**Product/runtime behavior:** unchanged.

## Goal

Complement the broad 406-crate lock inventory/compliance bundle with a distributable license-text report for the **actual Windows r18 normal+build dependency closure**, using pinned `cargo-about`. The broad bundle remains evidence for every package in Cargo.lock; this report is release-scope evidence for the Windows executable.

## Evidence anchor

- r18 source artifact run: `41415006414`;
- artifact: `Uvaga-Windows-Synthetic-r18` / id `13127022813`;
- exact `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- cargo-about version: `0.9.2`;
- Linux x86_64 musl release asset SHA256: `9099a59e820c38a68b9d65f300662a567d56562f9a10f6aa4c7e86c17c2566af`.

## Acceptance criteria

1. Verify the exact r18 Cargo.lock before generation.
2. Download cargo-about 0.9.2 from its GitHub release and verify the pinned binary archive SHA256.
3. Fetch the exact locked dependency graph once, then run cargo-about with `--locked --offline --fail`.
4. `cargo-about` is explicitly scoped to target `x86_64-pc-windows-msvc`, includes build dependencies, and excludes dev-only dependencies.
5. Generate a separate Cargo metadata view filtered to the Windows target and traverse the root graph over normal+build edges.
6. The set of external package name/version pairs emitted by cargo-about must equal that independently derived Windows normal+build closure.
7. Global accepted licenses match the reviewed permissive policy; GPL/AGPL/LGPL are not globally accepted.
8. MPL-2.0 is accepted only for the five reviewed crate names in cargo-about config; cargo-deny remains the version-exact enforcement layer.
9. Generate both machine-readable JSON and human-readable Markdown containing full license texts and the crates using each text.
10. The Markdown must identify that it is generated from the exact r18 Windows lock/target and is not a substitute for Uvaga's proprietary LICENSE.
11. The generation must fail if cargo-about cannot reasonably determine any dependency license.
12. Upload output, config, template, exact lock, Windows closure summary and evidence anchor as an Actions artifact.
13. Download/read back the artifact before declaring Windows text-completeness PASS.
14. No live source adapter, credentials, public notifications, production deployment, signing or release is introduced.

## Result state

**PENDING** until workflow output is read back.
