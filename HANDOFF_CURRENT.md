# HANDOFF_CURRENT — ТривогА

**Оновлено:** 2026-11-01  
**Методологія:** dev-strategy v2 RC — trial, **не прийнято повністю**.

## Start
GitHub source of truth: https://github.com/RomanZavadaM/Uvaga  
Review gate: https://github.com/RomanZavadaM/Uvaga/issues/9

Перед записом перевір live `main`, PR heads/bases і останні CI. Не відтворювати стан із чату.

## Confirmed integrated base
Initial main commit: `ad8653ebcdf16507be54a929b690c8a1060e035b`.  
Жоден draft PR #1–#11 ще не інтегрований у main, якщо live GitHub не показує інше. Release/production відсутні.

## Owner decisions
- public product, вся Україна, alerts-plus;
- desktop + mobile + web; Telegram later;
- Rust + Tauri 2; Windows 7 excluded;
- lightweight relay/cache, no PostgreSQL requirement;
- proprietary Roman Zavada;
- secret source tokens only server/relay side;
- no production/public notifications without separate decision.

## Safety invariants
- source outage/timeout/TTL/HTTP error never creates official clear;
- aggregator/supplemental data cannot self-promote to official state;
- stale previous clear is not a current safe state;
- official state and source freshness are separate;
- no live military target tracking/trajectories.

## Active draft stack
- PR #1 docs
- PR #2 r1 offline state core
- PR #3 r2 multi-region store
- PR #4 r3 Tauri offline shell
- PR #5 r4 source trust/normalization
- PR #6 r5 localhost synthetic relay
- PR #7 r6 alerts.in.ua v2 parser
- PR #8 r7 freshness states
- PR #10 r8 conservative recovery snapshot
- PR #11 r9 fail-safe presentation states
Issue #9 defines independent review/integration order.

## Verified CI evidence
Known successful runs:
- docs `41338991422`;
- r1 `41335891826`;
- r2 `41336873203`;
- Tauri product-platform matrix `41338709796`: Windows x64 + macOS ARM64 + macOS Intel successful;
- r4 `41337182862`;
- r5 `41337322695`;
- r6 `41337434827`;
- r7 `41337648092`;
- r8 after consistency fix `41338929570`;
- r9 `41338934425`.

PR #1 documentation changed after the listed docs run; check newest docs CI before review/merge.

## Tauri dependency checkpoint
CI artifact contains generated `src-tauri/Cargo.lock`:
- lock SHA256 `420880998b64ca0e1a882c781b7d4f1577b9873be8b943ae3f425482bfb514fd`;
- artifact ZIP SHA256 `f4ab56495655c1e0a6b1c8d312940c4776697d06d7491961d8ff6bd0931d464e`.
Direct Tauri pins: 2.12.5 / tauri-build 2.7.4. Full Cargo.lock still needs repository commit before release.
Direct dependency license audit exists; transitive release gate remains open.

## Sources
- Official UkraineAlarm API application/offer reviewed from public documentation; no access key obtained, no live call made.
- alerts.in.ua docs reviewed; public service requires own proxy and secret token handling; parser PR #7 is fixture/offline only.
- No third-party credential exists in repository.

## Google Drive / backup
Drive root: https://drive.google.com/drive/folders/1sPq7Ajmi4uhydjK7Wy1B-vR5IwTFVt8b  
PROJECT mirror has START/RULES/STATE/WORKLOG/HANDOFF. Write + readback verified with GitHub blob markers and normalized content.
Verification record: https://docs.google.com/document/d/1W7cGX-l0ozK7Se8Q0OEN8WM-WqanVBwv-V53JRqAlgA/edit

This is a mirror, NOT full Git backup. Full Git bundle/restore remains OPEN because the container could not resolve github.com.

## One next action
Run **independent review of PR #1 using Issue #9**. If PASS, integrate/retarget one slice at a time with fresh CI. Do not merge child PRs out of order.

For ordinary continuation: `START_HERE.md → PROJECT_RULES.md → PROJECT_STATE.md → WORKLOG.md → HANDOFF_CURRENT.md → live GitHub`. **Центральний dev-strategy не потрібний щодня.**
