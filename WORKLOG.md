# WORKLOG — ТривогА

## 2026-11-01 — foundation and offline core
- Created public proprietary `RomanZavadaM/Uvaga`.
- Established project-local continuity, safety rules, source/backup policies and dev-strategy v2 RC trial.
- Integrated offline Rust alert state, multi-region store, source freshness, conservative recovery and fail-safe presentation.
- Integrated source trust, provider event-version identity, alerts.in.ua offline parser and localhost synthetic relay.
- No live credentials, production deployment or public notifications used.

## 2026-11-01 — territory/source model
- Integrated source-neutral `Country → Region → Raion → Hromada → Locality` hierarchy.
- Integrated provider→core mapping and fail-closed hierarchical subscription matching.
- Integrated mapped official source normalization.
- KATOTTG policy requires Ministry base + amendment discovery to explicit cutoff; staging validator accepts current reviewed source shape fail-closed.
- UkraineAlarm request page/public offer audited; application/offer acceptance remains owner-only.
- Integrated deterministic attention ranking anchored to shared PresentationState.

## 2026-11-01 — desktop prototype
- Integrated minimal Tauri 2 shell, Windows 10/11 x64 + macOS ARM64/Intel CI.
- Direct pins: tauri 2.12.5; tauri-build 2.7.4.
- Cargo.lock checkpoint SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.
- CI optimized so pure root-core changes do not trigger full desktop matrix.
- r16 integrated: local UI ↔ Rust core bridge through Tauri IPC, no HTTP/live API/plugins.
- r16 Windows bridge unit test and both macOS compile targets PASS.
- r16 squash main: `bba0bd82351fa06f20dbe144e8ac487b81314e4f`.

## 2026-11-01 — r17 Windows synthetic artifact
- PR #28 created for build/test artifact only.
- Final source head: `2a9ef7e1d30c50a42dedcf822a2c8728a6d1d490`.
- Workflow run `41352681448`: PASS.
- `cargo test --locked`: PASS.
- `cargo build --locked`: PASS.
- Artifact downloaded/unpacked/read back; checksums verified.
- Raw ZIP mirrored to Drive BACKUPS.

## 2026-11-02 — r17 manual runtime acceptance
- Owner launched verified `Uvaga.exe` on Windows and supplied screenshots.
- Application opened successfully.
- Synthetic/not-real-alerting warning visible.
- Local Tauri IPC success visible.
- UI showed UA-46, active confirmed state, stale source, active-last-known presentation.
- No false clear displayed.
- Manual runtime acceptance: **PASS**.
- PR #28 merged squash: `0ea256b3cc1a72250a8fa06e3098d13fe780b770`.

## 2026-11-02 — C12 Git backup / restore
- Added Git bundle backup/restore workflow and acceptance doc.
- PR #31 pre-merge run: bundle verify + clean restore + fsck + restored Rust tests PASS.
- PR #31 merged squash: `81c3855112006d9350077b2a5b38f62e3701f73d`.
- Post-merge `main` run `41405252149`: PASS.
- GitHub artifact id `13123246036`.
- Artifact ZIP SHA256 `d236ae8b26179f44e64365b20f08484a65e1730633e7319386d2a14ecbe4de06`.
- Inner `Uvaga-all-refs.bundle` SHA256 `943ee4667b69871d8f851afb2695396a6a5a5ea6c820433c5f3083bcfdcf7a53`.
- Clean restore reproduced exact main SHA `81c3855112006d9350077b2a5b38f62e3701f73d`.
- Additional bundle download/readback/checksum/restore/fsck verification PASS.
- Raw backup copied to Drive BACKUPS.
- C12: **PASS for Git repository source/history**.
- Future production databases/secrets/user state remain outside this checkpoint.

## 2026-11-02 — C13 clean-session continuity
- Formal clean-session continuity trial completed from project-local GitHub docs + live GitHub only.
- Start main independently verified as `3a254e4a6edb3bcd2294cb0905ab7ddae4f1ca61`.
- Open PR count: 0.
- r17 Actions evidence re-read at job level: run `41352681448` PASS.
- C12 post-merge Actions evidence re-read at job/artifact level: run `41405252149` PASS; artifact `13123246036` present and not expired.
- Safety/permission limits recovered correctly.
- C13: **PASS**.
- Detailed record: `C13_CLEAN_SESSION.md`.

## 2026-11-02 — r18 synthetic scenarios / territory selection
- Predeclared acceptance created before implementation.
- PR #34 opened: `feat(r18): add synthetic territory and fail-safe scenarios`.
- Verified product source head: `a2c945034bfea58b3ff5890ea53862831c1b502a`.
- Added two whitelisted synthetic territories: `UA-46`, `UA-30`.
- Added five fail-safe scenarios: active/fresh, active/stale, cleared/fresh, cleared/stale, unknown/unavailable.
- Unknown territory/scenario rejected; local Tauri IPC only; IPC failure never creates clear.
- Automated CI/artifact verification: PASS.
- Verified artifact id `13127022813`; ZIP SHA256 `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`; `Uvaga.exe` SHA256 `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`.
- Subsequent PR-head changes through `cccce86708e77e6a1cd670179ebf9547d0a5f671` are README-only; comparison confirms no r18 product files changed after the verified artifact build.
- Latest PR-head workflows also PASS: docs `41417199329`, Tauri matrix `41417199299`, Windows artifact `41417199372`.
- Owner completed manual Windows 10/11 x64 acceptance with screenshots: **PASS**.
- UA-46 and UA-30 switching: PASS.
- active/fresh: PASS.
- active/stale: PASS; stale active remains last-known active.
- cleared/fresh: PASS.
- unknown/unavailable: PASS; insufficient data shown.
- critical cleared/stale: PASS; UI shows stale data and `Відбій потребує повторного підтвердження`, never a current safe all-clear.
- PR #34 reconciled with main `1b1c1d011dd427fd5c2a1fcd9b83054e6e3a9bd2`; reconciled product head `00044595f40b48b19e086e3e8a2a81cfaa48bfb5`.
- Fresh reconciled-head checks PASS: docs `41437451193`, Tauri matrix `41437451223`, Windows artifact `41437451346`.
- r18 manual acceptance: **PASS**.
- Final acceptance-checkpoint head `cb444fd31bd4f2e574944c7917df5452f154b12e`: docs `41438228543` PASS, Tauri matrix `41438228579` PASS, Windows artifact `41438228529` PASS.
- PR #34 merged squash to main: `61c0662d941e388858f5214c14860ec92a586905`.
- Post-merge open PR count: 0.
- r18 is **integrated and accepted**.

## 2026-11-02 — README localization
- PR #35 merged to main: `e3d11e7ac47ee72a9d22ab5205115dac1d8a8956`.
- Ukrainian README remains primary/canonical.
- Added English, French, German, Spanish, Korean and Japanese README variants.
- Every README includes the verified r18 synthetic Windows download link, hashes and non-release/non-production warning.
- Organizational docs/link checks: PASS.

## 2026-11-02 — dependency/license inventory and policy
- Exact verified r18 Cargo.lock reused; no newly resolved graph used as evidence.
- Inventory run `41422471078`: PASS.
- Inventory artifact `13130467223`, ZIP SHA256 `afc0c0a75986fc68ea836a2cf7778353735dfdcb8bb24e5b99368e23ca351f7f`.
- 408 packages: 2 local proprietary path packages + 406 crates.io packages.
- No external git/non-registry dependencies and no missing Cargo license expressions.
- `THIRD_PARTY_NOTICES.md` corrected from stale docs-only wording.
- MPL-2.0 reviewed as exact transitive exceptions: cssparser 0.37.0, cssparser-macros 0.7.1, dtoa-short 0.3.5, option-ext 0.2.0, selectors 0.38.0.
- Machine policy run `41423140592`: PASS.
- cargo-deny 0.20.2 pinned by action commit; `licenses sources` check PASS.
- Unknown git/registry sources denied; GPL/AGPL/LGPL not globally allowed.
- Broad compliance bundle PR #40 merged: exact lock, 406 crates indexed, 371 package-local license/notice captures, 35 explicitly unresolved by package-local-file scan, all 5 MPL source trees archived.
- Final Windows license-text PR #41 merged.
- Final Windows license-text run `41426326573`: PASS.
- Artifact `13131534490`, ZIP SHA256 `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e`.
- Exact Windows normal+build external closure: 222; cargo-about external set: 222; missing 0; extra 0.
- Full Markdown license texts and MPL evidence present.
- Windows r18 third-party text completeness: **PASS**.
- Drive BACKUPS readback verified exact:
  - broad compliance bundle `1WRal8jzB9TPiwZUQczYlCf86MZbbQFdU` → SHA256 `af4561fd187f588333d263df5064a3c06271e2bd7b7d21546b915425dc6477d2`;
  - Windows license-text bundle `11C7VE5QZhyMtkYwbJoDVraLAe9c08bEr` → SHA256 `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e`.
- Future macOS/Android/iOS releases require their own target-specific completeness run and non-expiring distribution packaging.

## r19 — product hardening / desktop UX
**INTEGRATED / AUTOMATED PASS / OWNER VISUAL SANITY PENDING — 2026-11-02.**

PR #45 merged squash to main:
`1af2e5d2c53a00619172056bab9fde746daff988`.

Final accepted PR head:
`cc4992975b73f0211cdd44b05f7ebd52d76a5415`.

Implemented:
- compact critical-state summary above technical explanations;
- semantic presentation styling with explicit text, not color-only;
- accessible form labels, live status region and `aria-busy`;
- technical explanation collapsed by default;
- app version surfaced from Rust payload;
- IPC error clears previous snapshot and forces error/unknown;
- static UI safety-contract CI;
- self-describing Windows r19 artifact with README, proprietary license, third-party notices and checksums;
- stale README r18 status corrected in UA/EN/FR/DE/ES/KO/JA.

Important defects found and fixed before merge:
- presentation selector collision that could replace bridge contents;
- UI-contract checker false failure on escaped Rust JSON source;
- silent Cargo.lock drift (`cc 1.10.0 → 1.11.0`) discovered by artifact readback.

Dependency determinism is now fail-closed:
- all desktop CI regenerates the graph, restores `cc 1.10.0`, then requires exact Cargo.lock SHA256
  `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- test/build then runs with `--locked`;
- lock gate passed on Windows, macOS ARM and macOS Intel.

Final CI:
- UI contract `41442359517`: PASS;
- docs `41442359398`: PASS;
- Windows artifact `41442359441`: PASS;
- Tauri matrix `41442359542`: PASS.

Final Windows r19 artifact:
- artifact id `13140138112`;
- ZIP SHA256 `7f2dc03cd58686e2555026a544d78673f9fdcf7b452c44913b40013ee34bffad`;
- `Uvaga.exe` SHA256 `c9afe5873aad563dd6f07b51a6b05cc64fac3057e91f1127d2794b3372b7b593`;
- embedded Cargo.lock SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- internal SHA256SUMS readback: all entries PASS.

Drive BACKUPS copy:
https://drive.google.com/file/d/1Gjza0jCCQlGX8TDuPEUF49lX0g3iZ2BH/view?usp=drivesdk
— readback ZIP SHA256 matches GitHub exactly.

r19 is not a production release. Owner visual sanity of the redesigned layout remains pending; r18 remains the last owner-manually accepted UI baseline.

## Execution continuity
- One active executor by default; parallel work only by owner.
- State divergence is recovered from live GitHub rather than assumed parallel work.
- Issue #9 is the live integration register.

## NEXT
Owner-independent CI efficiency / release-checklist hardening. Owner r20 installer/visual sanity remains pending; no code signing, public release or live-source integration automatically.

## 2026-11-02 — r20 Windows installer / release-readiness
- PR #47 merged squash: `e913edf6bb62249806afc7c19ac8c19e2cde1166`.
- Final PR head: `96a7953a2388811716c9c13001fcf9ca3d5b79c2`.
- Final CI PASS: docs `41447023650`, Tauri matrix `41447023564`, Windows artifact `41447023544`, NSIS installer `41447023597`.
- Automated installer smoke PASS: current-user silent install, HKCU registration, launch smoke, silent uninstall.
- Installer/release EXE confirmed unsigned by Authenticode as intended for r20 test candidate.
- Artifact id `13142977634`; ZIP SHA256 `a8a00e0c7f20282d13673ba1fd3de353d4500d9cf36b1d40ed7be3c86ee33d3f`.
- Installer SHA256 `dc8f128e73983d28c4093edd0434d4a8eed140a29718cd09faa99c13bf9f8588`.
- Release EXE SHA256 `636aa7d7ec6e5cf485dfda219d107115dba7d3dc79c0b5a995d4e08efc4aab30`.
- Exact Cargo.lock remains `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.
- Internal SHA256SUMS readback: PASS.
- Drive BACKUPS copy `1A0fAVnfmgnK6xfVwTlXB-muZxTnGrTuX` downloaded back and matched GitHub artifact byte-for-byte.
- Owner installer UX/manual sanity: **PENDING**. Not a production release.

## 2026-11-02 — r21 artifact identity / version metadata
- PR #49 merged squash: `68223a2b29c04f762c41d6bed6a4fd70def764e4`.
- Final head `df8f37f9e981ce336252af5a00b2713186049ed6`.
- Release metadata/docs/synthetic/installer checks: PASS.
- Active artifact names are now provenance-neutral candidates with run numbers; historical r19/r20 evidence remains immutable.
- Version consistency gate enforces Cargo app version == Tauri config version; current prototype version remains 0.1.0.
- Initial Windows metadata failures were diagnostic-only cp1252 Unicode output; assertions had passed; fixed with ASCII-safe diagnostics.
- Synthetic candidate artifact `13145036714`, ZIP SHA256 `3147ce551e99918cae90ff0752ff998b25fff5df9c71577e71e4d4a8e3ab7630`.
- Installer candidate artifact `13144614348`, ZIP SHA256 `32d69221465da743fe1e7e6503653eda8daf0572591b3a7af3f9302f5a5ca14c`.

## 2026-11-02 — r22 CI efficiency + release checklist integrated
- Live pre-merge main: `8729f905f17d830feba68b62c5ea72ef9e13ede3`.
- PR #52 exact head: `4376a23820c5232eaa148526ff17b82e6184964f`.
- Final checks PASS: release metadata `41453727274`, docs `41453727294`, CI scope `41453727434`, Windows synthetic `41453727373`, Windows installer `41453727648`.
- PR #52 squash-merged to main: `9087e6fda537333092df8e5010d5a9f3f0d5ac11`.
- Diff was exactly 7 CI/docs/script files; no product/safety/live/signing/version changes.
- PR #51 was discovered during clean-session reconciliation, reviewed, documented and closed unmerged as a superseded alternative.
- r22 result: docs-only edits no longer auto-run expensive Windows synthetic/installer builds; heavy workflows keep manual `workflow_dispatch`; `RELEASE_CHECKLIST.md` and cheap CI-scope guard added.
- This post-merge docs PR is the practical docs-only trigger smoke; heavy Windows workflows must not auto-run.

