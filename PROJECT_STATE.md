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
- verified C12 Git bundle ZIP.

## r18 — richer synthetic scenarios / territory selection
**AUTOMATED PASS / MANUAL WINDOWS PENDING — 2026-11-02.**

Open PR: #34 `feat(r18): add synthetic territory and fail-safe scenarios`.

Verified product source head:
`a2c945034bfea58b3ff5890ea53862831c1b502a`.

Current PR head after README-only documentation commits:
`cccce86708e77e6a1cd670179ebf9547d0a5f671`.

Comparison `a2c945... → cccce867...` changes only README files; no r18 product/source file changed after the verified artifact was built.

Automated checks on the current PR head:
- Organizational docs checks `41417199329`: PASS;
- Tauri shell product-platform checks `41417199299`: PASS;
- Windows synthetic artifact `41417199372`: PASS.

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

**Manual Windows 10/11 runtime acceptance is still required before PR #34 may be accepted/merged.**

## README localization
Merged to `main` via PR #35 at `e3d11e7ac47ee72a9d22ab5205115dac1d8a8956`.
Ukrainian is primary/canonical; translations: English, French, German, Spanish, Korean, Japanese. All README variants link to the verified r18 synthetic Windows test artifact and mark it clearly as non-release/non-production.

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
Виконати **ручне Windows 10/11 x64 runtime-приймання r18** з перевіреного artifact `13127022813`. До PASS не зливати PR #34 і не починати live source adapter.
