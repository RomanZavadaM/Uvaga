# HANDOFF_CURRENT — ТривогА

**Оновлено:** 2026-11-01  
**Методологія:** dev-strategy v2 RC — trial, не прийнято повністю.

## Start
GitHub source of truth: https://github.com/RomanZavadaM/Uvaga  
Verification/integration register: https://github.com/RomanZavadaM/Uvaga/issues/9

На початку сесії:
1. прочитати START_HERE / PROJECT_RULES / PROJECT_STATE / WORKLOG / HANDOFF_CURRENT;
2. перевірити live `main`, відкриті PR і CI;
3. якщо пам'ять сесії не збігається з GitHub — зупинити запис і відновити фактичну хронологію з GitHub.

## Integrated product base
Last integrated product-code base: `bba0bd82351fa06f20dbe144e8ac487b81314e4f` (r16).

r16 доводить локальний ланцюг:
`Rust core → private Tauri command → window.__TAURI__.core.invoke → vanilla JS UI`.

Synthetic fixture:
- region = UA-46;
- confirmed state = active;
- freshness = stale;
- presentation = active_last_known.

Windows unit test + macOS ARM64/Intel compile checks: PASS.

## Current work
Draft PR #28 — r17 Windows synthetic test artifact.

Source PR head:
`2a9ef7e1d30c50a42dedcf822a2c8728a6d1d490`

Workflow run `41352681448`: PASS.
Verified ZIP:
- `Uvaga.exe` SHA256 `220a15adc02ab03ff2f067f01a3f67a31b97fc88c01eb46a0633d4ee9022acd8`;
- `Cargo.lock` SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- ZIP SHA256 `c02fab2a009e85ea3b9218f800e00c8e53be24f6e8b5789f3e188344bf281c98`.

PR #28 stays draft until **manual Windows runtime acceptance**.

## Manual acceptance
Expected:
1. application starts;
2. obvious synthetic/not-real-alerting banner;
3. UI shows active + stale + active-last-known;
4. refresh button repeats local IPC read;
5. no network/live API/push/geolocation;
6. failure state, if any, must never look like a clear.

## Safety
No production, public notifications, live API application/credential, signing or release package at this stage.

## Drive
Raw r17 ZIP mirrored to Drive BACKUPS.
Verification document exists in Drive VERIFICATION.
Full Git bundle + restore test remains open.

## Next after owner result
- PASS: merge #28, record runtime evidence, then implement Git bundle + restore checkpoint for C12.
- FAIL: record exact symptom in #28, fix r17/r16 boundary, rebuild and retest.

After C12 checkpoint, next product work is richer synthetic scenarios/territory selection before any live source adapter.
