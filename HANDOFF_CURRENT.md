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

r16 доводить локальний ланцюг:
`Rust core → private Tauri command → window.__TAURI__.core.invoke → vanilla JS UI`.

Synthetic fixture:
- region = UA-46;
- confirmed state = active;
- freshness = stale;
- presentation = active_last_known.

## r17 manual Windows result
**PASS — 2026-11-02.**

Owner launched the checksum-verified `Uvaga.exe` and supplied screenshots showing:
- application starts successfully;
- explicit synthetic / not-real-alerting warning;
- `Стан отримано локально через Tauri IPC.`;
- `Тривога активна`;
- `Дані застаріли`;
- `Активна — останній підтверджений стан`;
- `synthetic=true · issued_at=100`;
- no false clear.

Verified artifact:
- source head `2a9ef7e1d30c50a42dedcf822a2c8728a6d1d490`;
- workflow `41352681448` PASS;
- `Uvaga.exe` SHA256 `220a15adc02ab03ff2f067f01a3f67a31b97fc88c01eb46a0633d4ee9022acd8`;
- `Cargo.lock` SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- ZIP SHA256 `c02fab2a009e85ea3b9218f800e00c8e53be24f6e8b5789f3e188344bf281c98`.

## Safety
No production, public notifications, live API application/credential, signing or release package at this stage.

## Drive
Raw r17 ZIP mirrored to Drive BACKUPS.
Verification document exists in Drive VERIFICATION.
Full Git bundle + restore test remains open.

## One next action
Implement C12 Git-history backup:
1. create bundle from current main/history;
2. verify bundle;
3. clone/restore from bundle in clean CI workspace;
4. confirm restored HEAD;
5. preserve bundle + checksum in Drive BACKUPS;
6. record evidence in VERIFICATION/CORE_EVALUATION.

After C12, perform a clean-session continuity check, then continue richer synthetic scenarios/territory selection before any live source adapter.
