# Third-party license-text result — r18 Windows

**Date:** 2026-11-02  
**Product/runtime behavior:** unchanged.  
**Scope:** verified r18 Windows `x86_64-pc-windows-msvc` normal+build dependency closure.

## Result

**PASS.** Full third-party license-text generation for the current verified r18 Windows dependency closure is complete and read back.

Evidence:
- merged implementation: PR #41;
- main integration commit: `9b43681e85276db82f750ffd144c81febb59b149`;
- final Windows workflow run: `41426326573` — PASS;
- artifact: `13131534490` (`Uvaga-r18-third-party-license-texts`);
- artifact ZIP SHA256: `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e` — downloaded/read back, exact match;
- exact r18 Cargo.lock SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- pinned cargo-about: `0.9.2`;
- cargo-about Windows x86_64 MSVC archive SHA256: `1c03e5890238562497c2d89a3b75b02560af349c1fc3e713d3284f532a5cd748`.

## Coverage

Independent Windows-host `cargo tree --target x86_64-pc-windows-msvc --edges normal,build` external set:
**222 packages**.

cargo-about external set:
**222 packages**.

- missing from cargo-about: **0**;
- extra in cargo-about: **0**;
- dev-only dependencies: excluded from this distributable Windows scope;
- full human-readable license-text Markdown: present;
- machine-readable JSON: present;
- MPL-2.0: present for the five reviewed exact dependency families.

The workflow first exposed two invalid comparison approaches and rejected them rather than weakening acceptance:
- raw Linux-host Cargo metadata over-counted target/optional dependencies;
- Linux-host `cargo tree --target windows` omitted Windows-host build helpers (`vswhom`, `vswhom-sys`, `winreg`).

Final validation therefore runs on `windows-2025`, matching the r18 build host class, and requires exact 1:1 package/version set equality.

## Related broad compliance evidence

The broad exact-lock compliance bundle remains complementary evidence for all 406 crates.io packages in Cargo.lock:
- package-local notices/licenses captured for 371;
- 35 package-local-file gaps explicitly recorded;
- complete vendored source archive preserved for the five MPL-2.0 packages.

The Windows cargo-about report closes license-text coverage for the actual 222-package Windows normal+build closure even where a published crate did not carry a package-local LICENSE/NOTICE file.

## Drive backup

Final Windows license-text artifact mirrored to Drive BACKUPS:
https://drive.google.com/file/d/11C7VE5QZhyMtkYwbJoDVraLAe9c08bEr/view?usp=drivesdk

Broad compliance bundle also mirrored to Drive BACKUPS:
https://drive.google.com/file/d/1WRal8jzB9TPiwZUQczYlCf86MZbbQFdU/view?usp=drivesdk

## What remains for a future production release

This PASS applies to the current r18 Windows prototype dependency lock and target.

Before an actual public production release:
1. regenerate all dependency/license evidence against the exact final release lock;
2. regenerate target-specific license text coverage for every released platform;
3. place third-party license texts and MPL source/source-availability material in a durable non-expiring release location/package;
4. rerun machine policy after any dependency change.

This document is compliance engineering evidence, not legal advice.
