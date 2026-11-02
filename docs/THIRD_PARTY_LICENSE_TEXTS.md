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
- Windows x86_64 MSVC release asset SHA256: `1c03e5890238562497c2d89a3b75b02560af349c1fc3e713d3284f532a5cd748`.

## Acceptance criteria

1. Verify the exact r18 Cargo.lock before generation.
2. Run on `windows-2025`, matching the host OS class used by the r18 Windows build; download cargo-about 0.9.2 Windows x86_64 MSVC from its GitHub release and verify the pinned binary archive SHA256.
3. Fetch the exact locked dependency graph once, then run cargo-about with `--locked --offline --fail`.
4. `cargo-about` is explicitly scoped to target `x86_64-pc-windows-msvc`, includes build dependencies, and excludes dev-only dependencies.
5. Derive an independent Windows package set with `cargo tree --target x86_64-pc-windows-msvc --edges normal,build`; keep filtered Cargo metadata only to distinguish registry vs local packages.
6. The set of external package name/version pairs emitted by cargo-about must equal that independent `cargo tree` Windows normal+build closure on the Windows host. This intentionally includes Windows-host build dependencies such as SDK-discovery helpers.
7. Global accepted licenses match the reviewed permissive policy; GPL/AGPL/LGPL are not globally accepted.
8. MPL-2.0 is accepted only for the five reviewed crate names in cargo-about config; cargo-deny remains the version-exact enforcement layer.
9. Run cargo-about only once to generate machine-readable JSON, then deterministically render human-readable Markdown from that same verified JSON; both outputs contain full license texts and the crates using each text.
10. The Markdown must identify that it is generated from the exact r18 Windows lock/target and is not a substitute for Uvaga's proprietary LICENSE.
11. The generation must fail if cargo-about cannot reasonably determine any dependency license.
12. Upload JSON/Markdown output, config, exact lock, Windows closure summary and evidence anchor as an Actions artifact.
13. Download/read back the artifact before declaring Windows text-completeness PASS.
14. No live source adapter, credentials, public notifications, production deployment, signing or release is introduced.

## Result state

**PASS — 2026-11-02.**

Final verification:
- head `19a317df2e1a3169d26b63036fafc21f02438748`;
- docs run `41426326526`: PASS;
- Windows license-text run `41426326573`: PASS;
- artifact `13131534490`;
- artifact ZIP SHA256 `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e` — download/readback exact match;
- exact Cargo.lock SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- Windows normal+build external closure 222;
- cargo-about external set 222;
- missing 0; extra 0;
- full Markdown license text present;
- MPL-2.0 + cssparser evidence present.

**Windows r18 third-party text completeness: PASS.**

Scope remains target-specific; future non-Windows release targets require their own completeness run.

## Rejected validation attempts

- Linux-host raw `cargo metadata.resolve` comparison was rejected because it over-counted target/optional dependencies.
- Linux-host `cargo tree --target windows` comparison was rejected because host-side Windows build dependencies (`vswhom`, `vswhom-sys`, `winreg`) are not present when the host itself is Linux.
- Final acceptance therefore requires Windows-host set equality rather than suppressing those three packages manually.

## CI efficiency

The initial proof run used cargo-about twice (JSON + Handlebars) and passed. The final workflow intentionally uses one cargo-about `--locked --offline --fail` scan and renders Markdown from that exact JSON to avoid a redundant second dependency/license analysis.
