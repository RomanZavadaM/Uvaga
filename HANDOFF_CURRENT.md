# HANDOFF_CURRENT — ТривогА

**Оновлено:** 2026-11-02  
**Методологія:** dev-strategy v2 RC — trial, не прийнято повністю.

## Start
GitHub source of truth: https://github.com/RomanZavadaM/Uvaga  
Verification/integration register: https://github.com/RomanZavadaM/Uvaga/issues/9

На початку сесії:
1. прочитати START_HERE / PROJECT_RULES / PROJECT_STATE / WORKLOG / HANDOFF_CURRENT;
2. перевірити live `main`, відкриті PR і CI;
3. якщо пам'ять сесії не збігається з GitHub — зупинити запис і відновити фактичну хронологію з GitHub.

## Integrated prototype
r16 product-code base:
`bba0bd82351fa06f20dbe144e8ac487b81314e4f`.

r17 artifact workflow/manual runtime acceptance merged:
`0ea256b3cc1a72250a8fa06e3098d13fe780b770`.

C12 source/history backup checkpoint:
`81c3855112006d9350077b2a5b38f62e3701f73d`.

r16 доводить локальний ланцюг:
`Rust core → private Tauri command → window.__TAURI__.core.invoke → vanilla JS UI`.

## r17 manual Windows result
**PASS — 2026-11-02.**

Owner launched checksum-verified `Uvaga.exe`; screenshots show:
- explicit synthetic / not-real-alerting warning;
- local Tauri IPC success;
- confirmed active state;
- stale source;
- active-last-known presentation;
- no false clear.

Artifact:
- `Uvaga.exe` SHA256 `220a15adc02ab03ff2f067f01a3f67a31b97fc88c01eb46a0633d4ee9022acd8`;
- ZIP SHA256 `c02fab2a009e85ea3b9218f800e00c8e53be24f6e8b5789f3e188344bf281c98`.

## C12 Git backup/restore
**PASS — 2026-11-02.**

Post-merge run `41405252149`:
- create/verify full refs bundle: PASS;
- clean restore of source main: PASS;
- exact restored/source SHA equality: PASS;
- `git fsck --full`: PASS;
- restored Rust tests: PASS;
- artifact upload/readback/checksum: PASS.

Bundle:
- source main `81c3855112006d9350077b2a5b38f62e3701f73d`;
- bundle SHA256 `943ee4667b69871d8f851afb2695396a6a5a5ea6c820433c5f3083bcfdcf7a53`;
- artifact ZIP SHA256 `d236ae8b26179f44e64365b20f08484a65e1730633e7319386d2a14ecbe4de06`.

Drive BACKUPS:
https://drive.google.com/file/d/1ybaj5qBhXWNprZ7v4GLvIZeDXTr4xobp/view?usp=drivesdk

C12 scope is Git source/history only. Production state/data backup remains a separate future concern if such state is introduced.

## r18 integrated state
**AUTOMATED PASS / MANUAL WINDOWS PASS / MERGED — 2026-11-02.**

PR #34 merged squash to main:
`61c0662d941e388858f5214c14860ec92a586905`.

Verified manual-test product source head: `a2c945034bfea58b3ff5890ea53862831c1b502a`.
Reconciled product head: `00044595f40b48b19e086e3e8a2a81cfaa48bfb5`.
Final acceptance-checkpoint head: `cb444fd31bd4f2e574944c7917df5452f154b12e`.

PR #34 was reconciled with current main `1b1c1d011dd427fd5c2a1fcd9b83054e6e3a9bd2` without replaying stale README/state/license changes.

Fresh reconciled-head checks:
- docs `41437451193`: PASS;
- Tauri Windows/macOS matrix `41437451223`: PASS;
- Windows synthetic artifact `41437451346`: PASS.

Final acceptance-checkpoint checks:
- docs `41438228543`: PASS;
- Tauri Windows/macOS matrix `41438228579`: PASS;
- Windows synthetic artifact `41438228529`: PASS.

Post-merge live main: `61c0662d941e388858f5214c14860ec92a586905`.
Open PR count after merge: 0.

Owner manual Windows screenshots confirm:
- UA-46 and UA-30 switching;
- active/fresh → active current;
- active/stale → active last-known;
- cleared/fresh → cleared current;
- cleared/stale → clear requires reconfirmation;
- unknown/unavailable → insufficient data.
Critical stale-clear fail-safe: **PASS**.

Manual-test artifact:
- id `13127022813`;
- ZIP SHA256 `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`;
- `Uvaga.exe` SHA256 `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`.

r18 implements synthetic territory selection and five fail-safe scenarios only. No live adapter/credentials/public notifications/production/signing/release.

README localization is already merged independently via PR #35; Ukrainian is primary, with EN/FR/DE/ES/KO/JA translations.

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

## Dependency / license status
**Inventory PASS + machine policy PASS — 2026-11-02.**

Inventory:
- run `41422471078`;
- artifact `13130467223`;
- exact r18 Cargo.lock SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- 408 packages total; 406 crates.io; 0 external git/non-registry; 0 missing license expressions.

Machine policy:
- run `41423140592`: PASS;
- cargo-deny 0.20.2 pinned;
- explicit permissive allow-list;
- MPL only for 5 exact reviewed crates;
- unknown git/registry dependencies denied.

Compliance packaging:
- PR #40 broad exact-lock bundle: generation/readback PASS; 406 crates indexed; all 5 MPL source trees preserved; package-local-file scan is partial (371/406).
- PR #41 exact Windows license-text generation: **PASS**.
- run `41426326573`; artifact `13131534490`; ZIP SHA256 `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e`.
- Windows `x86_64-pc-windows-msvc` normal+build closure: 222/222 external packages, missing 0, extra 0.
- full license text + MPL evidence present.

Drive BACKUPS:
- broad compliance bundle:
  https://drive.google.com/file/d/1WRal8jzB9TPiwZUQczYlCf86MZbbQFdU/view?usp=drivesdk
  — readback SHA256 `af4561fd187f588333d263df5064a3c06271e2bd7b7d21546b915425dc6477d2`;
- Windows license-text bundle:
  https://drive.google.com/file/d/11C7VE5QZhyMtkYwbJoDVraLAe9c08bEr/view?usp=drivesdk
  — readback SHA256 `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e`.

For future non-Windows release targets, repeat target-specific license-text completeness and publish required materials in a non-expiring release location.

## Safety
No production, public notifications, live API application/credential, signing or release package at this stage.

## C13 continuity result
**PASS — 2026-11-02.**

The clean session recovered the actual state from project-local documents + live GitHub without relying on prior chat. It independently confirmed the start main `3a254e4a6edb3bcd2294cb0905ab7ddae4f1ca61`, no open PR, r17 manual PASS, C12 PASS, safety limits and the correct next direction.

Detailed evidence: `C13_CLEAN_SESSION.md`.

## One next action
Continue with r20 Windows release-readiness only: unsigned NSIS test installer candidate, distribution/security docs, checksums and readback. Do not start live-source integration automatically.
