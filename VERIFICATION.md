# VERIFICATION — ТривогА

**Оновлено:** 2026-11-02

## Integrated product state
r16 product-code base:
`bba0bd82351fa06f20dbe144e8ac487b81314e4f`.

r17 artifact/manual acceptance:
`0ea256b3cc1a72250a8fa06e3098d13fe780b770`.

C12 source/history backup checkpoint:
`81c3855112006d9350077b2a5b38f62e3701f73d`.

Live `main` must always be checked in GitHub.

## r17 manual runtime
**PASS — 2026-11-02.**

Owner screenshot evidence confirms:
- executable starts on Windows;
- explicit synthetic/not-real-alerting warning;
- local Tauri IPC success;
- UA-46;
- confirmed state active;
- source freshness stale;
- presentation active-last-known;
- no false clear.

## C12 Git backup/restore
**PASS — 2026-11-02 for Git repository source/history.**

Post-merge run:
`41405252149` — PASS.

Accepted source main:
`81c3855112006d9350077b2a5b38f62e3701f73d`.

GitHub artifact:
- id `13123246036`;
- ZIP digest `sha256:d236ae8b26179f44e64365b20f08484a65e1730633e7319386d2a14ecbe4de06`.

Bundle:
- `Uvaga-all-refs.bundle`;
- SHA256 `943ee4667b69871d8f851afb2695396a6a5a5ea6c820433c5f3083bcfdcf7a53`.

CI evidence:
- fetch all branches/tags: PASS;
- bundle create/verify: PASS;
- clean restore from bundle: PASS;
- restored HEAD equals source main: PASS;
- `git fsck --full`: PASS;
- restored Rust tests: PASS;
- artifact upload: PASS.

Additional readback:
- downloaded artifact ZIP SHA equals GitHub digest: PASS;
- inner SHA256SUMS: PASS;
- bundle verify: PASS;
- bundle reports complete history: PASS;
- independent clean restore reproduces exact source main SHA: PASS;
- fsck: PASS.

Drive raw backup:
https://drive.google.com/file/d/1ybaj5qBhXWNprZ7v4GLvIZeDXTr4xobp/view?usp=drivesdk

Drive verification document:
`ТривогА — verification — C12 Git bundle restore — 2026-11-02`.

Scope limit:
This C12 evidence does not cover future production databases, secrets, provider credentials, device tokens or user data.

## C13 clean-session continuity
**PASS — 2026-11-02.**

A clean session reconstructed state from project-local documents and live GitHub only.

Verified:
- live start main `3a254e4a6edb3bcd2294cb0905ab7ddae4f1ca61`;
- no open pull requests;
- r17 Windows manual runtime PASS plus successful Actions run `41352681448`;
- C12 clean Git restore PASS plus successful post-merge run `41405252149`;
- current production/live-credential/public-notification restrictions;
- correct next product slice: richer synthetic scenarios / territory selection, without live source adapter.

Detailed evidence: `C13_CLEAN_SESSION.md`.

## r18 synthetic territory/scenario verification
**INTEGRATED / AUTOMATED PASS / MANUAL WINDOWS PASS — 2026-11-02.**

PR #34 verified product source head:
`a2c945034bfea58b3ff5890ea53862831c1b502a`.

Manual-test artifact was built from product head `a2c945034bfea58b3ff5890ea53862831c1b502a`.

After manual acceptance, PR #34 was reconciled with main `1b1c1d011dd427fd5c2a1fcd9b83054e6e3a9bd2`.
Reconciled product head:
`00044595f40b48b19e086e3e8a2a81cfaa48bfb5`.

Compare `main → 00044595...` contains exactly six r18 product/workflow files and no stale README/state/license replay.

Fresh reconciled-head automated evidence:
- Organizational docs checks `41437451193`: PASS;
- Tauri shell product-platform checks `41437451223`: PASS;
- Windows synthetic test artifact `41437451346`: PASS.

Final acceptance-checkpoint head:
`cb444fd31bd4f2e574944c7917df5452f154b12e`.

Final pre-merge evidence:
- Organizational docs checks `41438228543`: PASS;
- Tauri shell product-platform checks `41438228579`: PASS;
- Windows synthetic test artifact `41438228529`: PASS.

PR #34 merged squash to main:
`61c0662d941e388858f5214c14860ec92a586905`.

Post-merge GitHub verification:
- live main equals the merge SHA above;
- open PR count = 0.

Manual-test artifact evidence from verified product head `a2c945...`:
- artifact id `13127022813`;
- ZIP SHA256 `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`;
- `Uvaga.exe` SHA256 `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`;
- `Cargo.lock` SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.

Manual Windows 10/11 x64 runtime evidence supplied by owner: **PASS**.

Confirmed mappings:
- UA-46 and UA-30 territory switching: PASS;
- active/fresh → active current: PASS;
- active/stale → active last-known: PASS;
- cleared/fresh → cleared current: PASS;
- cleared/stale → `Відбій потребує повторного підтвердження`: PASS;
- unknown/unavailable → official state unknown + source unavailable + insufficient data: PASS;
- local Tauri IPC remains successful.

Critical safety invariant: **PASS** — stale clear does not render as current safe all-clear.

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

## r20 Windows installer verification
**INTEGRATED / AUTOMATED PASS / OWNER INSTALLER UX PENDING — 2026-11-02.**

PR #47 final head `96a7953a2388811716c9c13001fcf9ca3d5b79c2`; merged main `e913edf6bb62249806afc7c19ac8c19e2cde1166`.

Final evidence:
- docs `41447023650`: PASS;
- Tauri matrix `41447023564`: PASS;
- Windows synthetic artifact `41447023544`: PASS;
- NSIS installer `41447023597`: PASS;
- artifact `13142977634`;
- ZIP SHA256 `a8a00e0c7f20282d13673ba1fd3de353d4500d9cf36b1d40ed7be3c86ee33d3f`;
- installer SHA256 `dc8f128e73983d28c4093edd0434d4a8eed140a29718cd09faa99c13bf9f8588`;
- release EXE SHA256 `636aa7d7ec6e5cf485dfda219d107115dba7d3dc79c0b5a995d4e08efc4aab30`;
- Cargo.lock SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.

Automated install acceptance:
- Authenticode status NotSigned: PASS;
- silent current-user install: PASS;
- HKCU uninstall registration: PASS;
- installed app launch smoke: PASS;
- silent uninstall: PASS;
- executable removed after uninstall: PASS;
- HKCU uninstall registration removed: PASS;
- internal artifact SHA256SUMS: PASS.

Drive readback:
https://drive.google.com/file/d/1A0fAVnfmgnK6xfVwTlXB-muZxTnGrTuX/view?usp=drivesdk
— downloaded back, byte-identical to GitHub artifact, SHA256 exact match.

Scope: automated release-readiness only. Owner installer UX/manual acceptance, code signing, public release and production/live integration remain pending.

## r21 artifact identity verification
**INTEGRATED / AUTOMATED PASS — 2026-11-02.**

PR #49 final head `df8f37f9e981ce336252af5a00b2713186049ed6`; merged main `68223a2b29c04f762c41d6bed6a4fd70def764e4`.

Final runs:
- release metadata `41451415389`: PASS;
- docs `41451415439`: PASS;
- synthetic candidate `41451415417`: PASS;
- installer candidate `41451415391`: PASS, including install/launch/uninstall.

Artifact readback:
- synthetic `13145036714` / `Uvaga-Windows-Synthetic-candidate-34` / ZIP SHA256 `3147ce551e99918cae90ff0752ff998b25fff5df9c71577e71e4d4a8e3ab7630` / internal checksums PASS;
- installer `13144614348` / `Uvaga-Windows-Installer-candidate-3` / ZIP SHA256 `32d69221465da743fe1e7e6503653eda8daf0572591b3a7af3f9302f5a5ca14c` / internal checksums PASS.

Metadata evidence in both candidates includes app version `0.1.0`, source head, checkout SHA, run ID and exact Cargo.lock SHA `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.

Changed files were CI/docs/scripts only. Product/safety code unchanged.

## README localization verification
PR #35 merged to main at `e3d11e7ac47ee72a9d22ab5205115dac1d8a8956`.
Ukrainian primary README plus EN/FR/DE/ES/KO/JA translations are present; local Markdown link validation PASS.

## Dependency / license verification
**Inventory PASS / machine policy PASS — 2026-11-02.**

Exact locked inventory:
- source r18 Cargo.lock SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- run `41422471078`: PASS;
- artifact `13130467223`;
- artifact ZIP SHA256 `afc0c0a75986fc68ea836a2cf7778353735dfdcb8bb24e5b99368e23ca351f7f`;
- package count 408;
- 406 crates.io + 2 local/path;
- 0 unknown external sources;
- 0 missing Cargo license expressions.

Machine policy:
- run `41423140592`: PASS;
- `cargo deny check licenses sources`: PASS;
- cargo-deny 0.20.2 via pinned action commit `3c6349835b2b7b196a839186cb8b78e02f7b5f25`;
- MPL-2.0 is exception-scoped to 5 exact reviewed crates, not globally allowed;
- unknown registry/git sources are denied;
- GPL/AGPL/LGPL identifiers are not in the global license allow-list.

Broad compliance bundle:
- final PR #40 run `41424208874`: PASS;
- artifact `13130384607`;
- ZIP SHA256 `af4561fd187f588333d263df5064a3c06271e2bd7b7d21546b915425dc6477d2` — local readback exact match;
- 406 crates indexed;
- 371 with captured package-local LICENSE/COPYING/NOTICE/COPYRIGHT/UNLICENSE files;
- 35 without package-local files explicitly reported;
- `MPL-SOURCE-r18.tar.gz` contains complete vendored source trees for all five reviewed MPL-2.0 packages.

Exact Windows r18 license-text completeness:
- final PR #41 run `41426326573`: **PASS**;
- artifact `13131534490`;
- ZIP SHA256 `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e` — local readback exact match;
- cargo-about 0.9.2 binary archive SHA256 `1c03e5890238562497c2d89a3b75b02560af349c1fc3e713d3284f532a5cd748`;
- target `x86_64-pc-windows-msvc`;
- dependency scope normal+build, dev excluded;
- independent Windows cargo-tree external set: 222;
- cargo-about external set: 222;
- missing: 0; extra: 0;
- full Markdown license text present;
- MPL-2.0 and cssparser evidence present.

**Windows r18 third-party text completeness: PASS.**

Drive BACKUPS readback:
- broad compliance bundle:
  https://drive.google.com/file/d/1WRal8jzB9TPiwZUQczYlCf86MZbbQFdU/view?usp=drivesdk
  — size 1,735,444 bytes; SHA256 `af4561fd187f588333d263df5064a3c06271e2bd7b7d21546b915425dc6477d2`, exact GitHub artifact match;
- Windows license-text bundle:
  https://drive.google.com/file/d/11C7VE5QZhyMtkYwbJoDVraLAe9c08bEr/view?usp=drivesdk
  — size 350,789 bytes; SHA256 `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e`, exact GitHub artifact match.

This PASS is target-specific. Future macOS/Android/iOS release targets require their own exact-lock target-specific compliance run and non-expiring release packaging.

## Source/legal checks
- UkraineAlarm public request page/offer checked without accepting/applying.
- KATOTTG uses cutoff/amendment-chain policy.
- alerts.in.ua remains aggregator/proxy candidate only.
- no live API credential used.

## Still open
- target-specific third-party compliance packaging for future macOS/Android/iOS releases;
- Android/iOS validation;
- live API access/security/latency/schema acceptance;
- push/background delivery;
- owner installer UX acceptance, code signing, public release and production.

## r22 CI efficiency / release-checklist verification
**INTEGRATED / AUTOMATED PASS — 2026-11-02.**

Accepted PR #52 head: `4376a23820c5232eaa148526ff17b82e6184964f`.
Merged main: `9087e6fda537333092df8e5010d5a9f3f0d5ac11`.

Pre-merge evidence:
- Release metadata `41453727274`: PASS;
- Organizational docs `41453727294`: PASS;
- CI scope `41453727434`: PASS;
- Windows synthetic `41453727373`: PASS;
- Windows NSIS installer `41453727648`: PASS.

Installer run additionally completed exact dependency-lock validation, application tests, accepted Windows license-text retrieval, unsigned NSIS build, Authenticode/bundle verification, silent current-user install, launch smoke, silent uninstall and artifact upload.

Compare base `8729f905f17d830feba68b62c5ea72ef9e13ede3` → accepted head contains exactly 7 CI/docs/script files. Product logic, safety mapping, dependency lock, live API, signing, updater and version were unchanged.

PR #51 was inspected and closed unmerged as a superseded alternative under `STOP → READ → VERIFY → RECONCILE → CONTINUE`.

Post-merge docs-only trigger smoke: **PASS**. Checkpoint head `a2583cb91d9c580f2b064647cfa541ca6b66a976` produced only Organizational docs run `41455027345` (PASS); Windows synthetic and Windows installer workflows did not start automatically.

## r22 Drive mirror verification
**PASS — 2026-11-02.**

Source GitHub main for the mirror operation: `0a85a94bb65046036f3d95d6372e0e22e58c43a3`.

Verified mirrors: PROJECT_STATE, WORKLOG, HANDOFF_CURRENT, VERIFICATION. After replacing content from GitHub and removing stale mirror-only terminal sentinels, readback produced the same number of non-empty records and the exact same ordered text sequence for every source line in each document. No mismatch remained.

