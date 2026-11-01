# HANDOFF_CURRENT — ТривогА

**Оновлено:** 2026-11-01  
**Методологія:** dev-strategy v2 RC — trial, не прийнято повністю.

## Start
GitHub source of truth: https://github.com/RomanZavadaM/Uvaga  
Verification/integration gate: https://github.com/RomanZavadaM/Uvaga/issues/9

Перед записом перевір live `main`, PR heads/bases і останні CI. Не відтворювати стан із чату.

## Integrated base
Current confirmed `main`: `1d7281c944ecfded2d70c7d7540bebdba06b0f15`.

Already integrated:
- #1 project foundation;
- #2 r1 alert state;
- #3 r2 multi-region store;
- #8 freshness;
- #10 recovery snapshot;
- #11 presentation states;
- docs checkpoint #22.

## Owner decisions
- public product, вся Україна, alerts-plus;
- desktop + mobile + web; Telegram later;
- Rust + Tauri 2; Windows 7 excluded;
- lightweight relay/cache, no PostgreSQL requirement;
- proprietary Roman Zavada;
- secret source tokens only relay/server side;
- no production/public notifications/live API acceptance without separate owner decision;
- один активний виконавець за замовчуванням; паралельність запускає тільки власник.

## Safety invariants
- source outage/timeout/TTL/HTTP error never creates official clear;
- aggregator/supplemental data cannot self-promote to official state;
- stale previous clear is not a current safe state;
- official state and source freshness are separate;
- no live military target tracking/trajectories.

## Current queue
Issue #9 веде порядок.
Next: #5 source contract → #7 parser → #6 local relay → #4 Tauri shell → territory/source/attention slices.

Старі stacked heads після squash треба rebuild/retarget на current main, а не тягнути стару історію.

## Sources
- UkraineAlarm public API/application/offer досліджено; credentials не отримувалися, live call не робився.
- alerts.in.ua docs досліджено; токен не в клієнтах, public service потребує relay/proxy.
- KATOTTG: production catalog має бути відтворюваним з Ministry base + applicable amendments.

## Google Drive / backup
Drive root: https://drive.google.com/drive/folders/1sPq7Ajmi4uhydjK7Wy1B-vR5IwTFVt8b  
PROJECT mirror існує. GitHub залишається source of truth.
BACKUPS зараз порожній.
Повний raw/git backup + fresh restore test залишається відкритим.

## One next action
Очистити PR #5 на current main, fresh CI, перевірка, squash merge.

For ordinary continuation: `START_HERE.md → PROJECT_RULES.md → PROJECT_STATE.md → WORKLOG.md → HANDOFF_CURRENT.md → live GitHub`. Центральний dev-strategy не потрібний щодня.