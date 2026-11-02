# PROJECT_STATE — ТривогА

**Оновлено:** 2026-11-02  
**Repository:** `RomanZavadaM/Uvaga`  
**Методологія:** dev-strategy v2 RC — пробне застосування, не прийнято повністю.

## Інтегрований стан
r16 product-code base:
`bba0bd82351fa06f20dbe144e8ac487b81314e4f`.

r17 Windows synthetic artifact workflow + manual runtime acceptance merged:
`0ea256b3cc1a72250a8fa06e3098d13fe780b770`.

C12 Git backup/restore checkpoint main:
`81c3855112006d9350077b2a5b38f62e3701f73d`.

Live `main` SHA на початку кожної сесії читати з GitHub, а не відновлювати з чату.

До `main` уже інтегровані:
- fail-safe official alert state, freshness, recovery і presentation;
- source trust/normalization та provider lifecycle identity;
- alerts.in.ua offline parser;
- localhost synthetic relay;
- Tauri 2 desktop shell;
- source-neutral territory hierarchy, provider mapping і hierarchical subscriptions;
- mapped source-territory normalization;
- KATOTTG cutoff/amendment policy + staging validator;
- UkraineAlarm public-offer audit;
- deterministic attention ranking;
- r16 local Tauri UI ↔ Rust core bridge;
- r17 checksumed Windows synthetic test-artifact workflow;
- C12 Git bundle backup + clean restore workflow.

## r17 acceptance — PASS
Final verified source head:
`2a9ef7e1d30c50a42dedcf822a2c8728a6d1d490`.

CI run `41352681448`: PASS.
- generate Cargo.lock: PASS;
- `cargo test --locked`: PASS;
- `cargo build --locked`: PASS;
- checksum creation/upload: PASS.

Verified artifact:
- `Uvaga.exe` SHA256 `220a15adc02ab03ff2f067f01a3f67a31b97fc88c01eb46a0633d4ee9022acd8`;
- `Cargo.lock` SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- ZIP SHA256 `c02fab2a009e85ea3b9218f800e00c8e53be24f6e8b5789f3e188344bf281c98`.

**Manual Windows 10/11 x64 runtime acceptance: PASS, 2026-11-02.**

Owner screenshot evidence confirms:
- application starts;
- synthetic/not-real-alerting warning is visible;
- local Tauri IPC succeeds;
- region = UA-46;
- confirmed state = active;
- source freshness = stale;
- presentation = active_last_known;
- no false clear is displayed.

## C12 Git backup / restore — PASS
Accepted source checkpoint:
`81c3855112006d9350077b2a5b38f62e3701f73d`.

Post-merge workflow run:
`41405252149` — PASS.

GitHub artifact:
- id `13123246036`;
- ZIP digest `d236ae8b26179f44e64365b20f08484a65e1730633e7319386d2a14ecbe4de06`.

Inner bundle:
- `Uvaga-all-refs.bundle`;
- SHA256 `943ee4667b69871d8f851afb2695396a6a5a5ea6c820433c5f3083bcfdcf7a53`.

Verified:
- bundle verify: PASS;
- clean restore from bundle: PASS;
- restored HEAD equals source main exactly: PASS;
- `git fsck --full`: PASS;
- restored root Rust tests in CI: PASS;
- artifact readback/checksum: PASS.

Raw bundle ZIP preserved in Drive BACKUPS:
https://drive.google.com/file/d/1ybaj5qBhXWNprZ7v4GLvIZeDXTr4xobp/view?usp=drivesdk

Scope: C12 PASS applies to Git repository source/history. Future production databases, secrets, provider credentials, device tokens or user data require separate backup/restore procedures if introduced.

## Safety invariants
- source outage/timeout/TTL/auth/schema error never creates official clear;
- aggregator/supplemental data cannot self-promote to official state;
- stale/unavailable previous clear is not current clear;
- provider territory IDs become core identity only through audited mapping;
- no live military target tracking/trajectories;
- no production deployment/public notifications/live credentials without separate owner decision.

## Виконання
Один активний виконавець за замовчуванням. Паралельність — лише за рішенням власника.
Якщо пам'ять сесії суперечить GitHub: STOP → READ → VERIFY → RECONCILE → CONTINUE.

## Sources
- UkraineAlarm public request page/offer rechecked 2026-11-01; no application/credentials/live call.
- alerts.in.ua public docs researched; token belongs only behind relay.
- KATOTTG production snapshot must use Ministry base + discovered amendment chain to explicit cutoff.

## Drive / recovery
GitHub — source of truth.
Drive PROJECT mirror exists.
BACKUPS contains:
- raw Tauri Cargo.lock checkpoint;
- verified r17 Windows ZIP;
- verified C12 Git bundle ZIP;
- exact r18 broad third-party compliance bundle:
  https://drive.google.com/file/d/1WRal8jzB9TPiwZUQczYlCf86MZbbQFdU/view?usp=drivesdk
  — SHA256 `af4561fd187f588333d263df5064a3c06271e2bd7b7d21546b915425dc6477d2`, Drive readback exact match;
- exact r18 Windows third-party license-text bundle:
  https://drive.google.com/file/d/11C7VE5QZhyMtkYwbJoDVraLAe9c08bEr/view?usp=drivesdk
  — SHA256 `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e`, Drive readback exact match.

## r18 — richer synthetic scenarios / territory selection
**INTEGRATED / AUTOMATED PASS / MANUAL WINDOWS PASS — 2026-11-02.**

PR #34 `feat(r18): add synthetic territory and fail-safe scenarios` merged squash to main:
`61c0662d941e388858f5214c14860ec92a586905`.

Verified manual-test product source head:
`a2c945034bfea58b3ff5890ea53862831c1b502a`.

After owner acceptance, PR #34 was reconciled with main `1b1c1d011dd427fd5c2a1fcd9b83054e6e3a9bd2`.
Reconciled product head:
`00044595f40b48b19e086e3e8a2a81cfaa48bfb5`.

The reconciliation preserved current main documentation/license checkpoints and applied only six r18 product/workflow files over main. Fresh checks on reconciled product head:
- Organizational docs checks `41437451193`: PASS;
- Tauri shell product-platform checks `41437451223`: PASS;
- Windows synthetic artifact `41437451346`: PASS.

Final acceptance-checkpoint head `cb444fd31bd4f2e574944c7917df5452f154b12e` also passed:
- Organizational docs checks `41438228543`;
- Tauri shell product-platform checks `41438228579`;
- Windows synthetic artifact `41438228529`.

After merge, live main = `61c0662d941e388858f5214c14860ec92a586905`; open PR count = 0.

Verified manual-test artifact remains the r18 artifact built from `a2c945...`:
- artifact id `13127022813`;
- ZIP SHA256 `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`;
- `Uvaga.exe` SHA256 `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`.

Scope implemented in PR #34:
- whitelisted synthetic territories `UA-46` and `UA-30`;
- five fail-safe scenarios: active/fresh, active/stale, cleared/fresh, cleared/stale, unknown/unavailable;
- local Tauri IPC only;
- unknown territory/scenario rejected;
- IPC failure never becomes a clear state;
- no live source adapter, credentials, notifications, geolocation, production deployment, signing or release.

**Manual Windows 10/11 x64 runtime acceptance: PASS — 2026-11-02.**

Owner screenshot evidence confirms both territories and all five mappings:
- UA-46 and UA-30 selection works;
- active/fresh → active current;
- active/stale → active last-known;
- cleared/fresh → cleared current;
- cleared/stale → **clear needs refresh / повторне підтвердження**;
- unknown/unavailable → insufficient data / unknown.

Critical invariant confirmed manually: a stale clear is not displayed as a current safe all-clear.

## README localization
Merged to `main` via PR #35 at `e3d11e7ac47ee72a9d22ab5205115dac1d8a8956`.
Ukrainian is primary/canonical; translations: English, French, German, Spanish, Korean, Japanese. All README variants link to the verified r18 synthetic Windows test artifact and mark it clearly as non-release/non-production.

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

## r20 — Windows installer / release-readiness
**INTEGRATED / AUTOMATED INSTALLER PASS / OWNER INSTALLER UX PENDING — 2026-11-02.**

PR #47 merged squash to main:
`e913edf6bb62249806afc7c19ac8c19e2cde1166`.

Final accepted PR head:
`96a7953a2388811716c9c13001fcf9ca3d5b79c2`.

Installer profile:
- Windows 10/11 x64;
- NSIS current-user install, no admin requirement;
- installer languages UA/EN/FR/DE/ES/KO/JA;
- embedded WebView2 bootstrapper;
- `allowDowngrades=false`;
- unsigned test artifact; no updater or production release;
- exact accepted application Cargo.lock enforced fail-closed;
- Tauri CLI pinned to 2.12.4.

Final CI:
- docs `41447023650`: PASS;
- Tauri Windows/macOS matrix `41447023564`: PASS;
- Windows synthetic artifact `41447023544`: PASS;
- Windows NSIS installer `41447023597`: **PASS**.

Installer smoke verified automatically:
- exact dependency lock restored;
- application tests PASS;
- NSIS installer built;
- installer and release EXE verified `NotSigned`;
- silent current-user install PASS;
- HKCU uninstall registration PASS;
- installed application launch smoke PASS;
- silent uninstall PASS;
- installed executable and uninstall registration removed.

Final r20 artifact:
- artifact id `13142977634`;
- ZIP SHA256 `a8a00e0c7f20282d13673ba1fd3de353d4500d9cf36b1d40ed7be3c86ee33d3f`;
- installer SHA256 `dc8f128e73983d28c4093edd0434d4a8eed140a29718cd09faa99c13bf9f8588`;
- release `Uvaga.exe` SHA256 `636aa7d7ec6e5cf485dfda219d107115dba7d3dc79c0b5a995d4e08efc4aab30`;
- Cargo.lock SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- internal SHA256SUMS: all entries PASS;
- package includes proprietary license, SECURITY, Windows distribution notes, full Windows third-party license texts and MPL source-availability notice.

Drive BACKUPS:
https://drive.google.com/file/d/1A0fAVnfmgnK6xfVwTlXB-muZxTnGrTuX/view?usp=drivesdk
— Drive readback is byte-identical to GitHub artifact; SHA256 exact match.

This is **not a production release**. Owner manual installer/visual sanity remains pending. Code signing, public release and live-source integration remain separate owner gates.

## r21 — artifact identity / version metadata hardening
**INTEGRATED / AUTOMATED PASS — 2026-11-02.**

PR #49 merged squash to main:
`68223a2b29c04f762c41d6bed6a4fd70def764e4`.

Final accepted head:
`df8f37f9e981ce336252af5a00b2713186049ed6`.

Implemented:
- active synthetic workflow now emits generic `Uvaga-Windows-Synthetic-candidate-<run>`;
- active installer workflow now emits generic `Uvaga-Windows-Installer-candidate-<run>`;
- historical r19/r20 artifact IDs and hashes remain immutable evidence;
- old historical `windows-installer-r20.yml` retired;
- app version consistency gate requires `src-tauri/Cargo.toml == tauri.conf.json`;
- current prototype app version remains `0.1.0`;
- candidate metadata records app version, source head, checkout SHA, run ID and exact dependency lock;
- release metadata gate also protects identifier, publisher, NSIS/currentUser, downgrade policy, WebView2 policy, installer languages and updater absence.

Final CI:
- release metadata `41451415389`: PASS;
- docs `41451415439`: PASS;
- synthetic candidate `41451415417`: PASS;
- installer candidate `41451415391`: PASS including install/launch/uninstall smoke.

Synthetic candidate evidence:
- artifact `13145036714`;
- name `Uvaga-Windows-Synthetic-candidate-34`;
- ZIP SHA256 `3147ce551e99918cae90ff0752ff998b25fff5df9c71577e71e4d4a8e3ab7630`;
- EXE SHA256 `6da7e7ae5267301f46715ede343b417c34296dbd27e71b6d22c28d75dc77d46b`;
- internal SHA256SUMS: PASS.

Installer candidate evidence:
- artifact `13144614348`;
- name `Uvaga-Windows-Installer-candidate-3`;
- ZIP SHA256 `32d69221465da743fe1e7e6503653eda8daf0572591b3a7af3f9302f5a5ca14c`;
- installer SHA256 `d236a650c691a530fb96cfd4ac8424cec26d3ed73fb7d9c315dc43bf239e4471`;
- release EXE SHA256 `dd11e4310d879e62094469080b17e6841bf4df00069efbba4b18af57d89fe191`;
- Cargo.lock SHA256 remains `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- internal SHA256SUMS: PASS.

No product/safety mapping code changed in r21.

## Dependency / license gate
Exact r18 dependency inventory: **PASS**.
- inventory run `41422471078`;
- artifact `13130467223`;
- 408 packages = 2 local/path + 406 crates.io;
- no external git/non-registry dependencies;
- no missing Cargo license expressions.

Machine license/source policy: **PASS**.
- policy run `41423140592`;
- cargo-deny 0.20.2, action pinned to `3c6349835b2b7b196a839186cb8b78e02f7b5f25`;
- permissive licenses explicitly allowed;
- MPL-2.0 only as exact exceptions for 5 reviewed crates;
- unknown registries/git dependencies denied;
- GPL/AGPL/LGPL are not globally allowed.

Third-party compliance packaging checkpoints:
- broad exact-lock bundle: PASS for generation/readback; 406 crates indexed, 371 with package-local notice/license files, 35 without package-local files explicitly reported;
- complete vendored source for all 5 reviewed MPL-2.0 packages preserved in `MPL-SOURCE-r18.tar.gz`;
- exact Windows r18 normal+build license-text completeness: **PASS** — 222/222 external packages, missing 0, extra 0;
- Windows license-text artifact id `13131534490`, ZIP SHA256 `bff4a206a3cdf7fae85a671b23bd566c3613beb8d19d99729b13d261cb531f6e`;
- target scope is `x86_64-pc-windows-msvc`, normal+build dependencies, dev excluded.

For any future macOS/Android/iOS release, run the same target-specific completeness check for that exact final release lock. Production distribution must place the required notices/license texts and MPL source-availability material in a non-expiring release location.

## Відкрите
- target-specific third-party compliance packaging for future macOS/Android/iOS release artifacts;
- Android/iOS build/runtime validation;
- live API credentials/schema/latency/webhook/security acceptance only after separate owner decision;
- push/background delivery;
- production packaging/signing/release.

## C13 continuity
**PASS — 2026-11-02.** Clean-session state reconstruction from project-local docs + live GitHub recovered the current state, r17 PASS, C12 PASS, safety limits and next product direction without previous-chat state. See `C13_CLEAN_SESSION.md`.

## Одна наступна дія
Owner-independent CI efficiency / release-checklist hardening. Owner manual Windows installer/visual sanity для r20 лишається окремим gate; code signing, public release та live source adapter не починати автоматично.

## r22 — CI efficiency + release checklist
**INTEGRATED / AUTOMATED PASS — 2026-11-02.**

PR #52 final head `4376a23820c5232eaa148526ff17b82e6184964f`; squash-merged to main:
`9087e6fda537333092df8e5010d5a9f3f0d5ac11`.

Final pre-merge checks:
- Release metadata `41453727274`: PASS;
- Organizational docs `41453727294`: PASS;
- CI scope `41453727434`: PASS;
- Windows synthetic `41453727373`: PASS;
- Windows NSIS installer `41453727648`: PASS, including exact-lock test, unsigned verification, install/launch/uninstall and artifact upload.

r22 removes documentation-only automatic triggers from heavy Windows synthetic/installer workflows, preserves `workflow_dispatch` for exact-ref final candidates, adds cheap CI-scope regression protection and establishes `RELEASE_CHECKLIST.md` as the project-local release gate.

Unexpected PR #51 was reconciled from live GitHub and closed unmerged as a superseded alternative. No product/safety/live/signing/version change from #51 was integrated.

This post-merge documentation PR is also the docs-only trigger smoke for r22. Heavy Windows workflows must remain absent from its automatic runs.

Owner r20 installer UX/manual sanity remains pending. Code signing, public release, live credentials/source integration, push notifications and production deployment remain owner-only gates.

