# HANDOFF_CURRENT — ТривогА

**Оновлено:** 2026-11-01  
**Методологія:** dev-strategy v2 RC — trial, не прийнято повністю.

## Start
GitHub source of truth: https://github.com/RomanZavadaM/Uvaga  
Verification/integration gate: https://github.com/RomanZavadaM/Uvaga/issues/9

Перед записом перевір live `main`, PR heads/bases і останні CI. Не відтворювати стан із чату.

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

## Work structure
Проєкт має stacked PR для малих перевірюваних slices. Live порядок, heads і integration rules ведуться в Issue #9. Після base merge наступний slice retarget/reconcile → fresh CI → перевірка → merge.

## Verified evidence
CI вже знаходив і допомагав виправляти реальні дефекти Tauri icon/build; Windows x64 + macOS ARM64 + macOS Intel product-platform checks проходили. Rust core/relay/parser/freshness/recovery/presentation/territory slices також мають успішні CI checkpoints. Перед інтеграцією завжди перевіряти current head, а не старий номер run із цього файла.

## Sources
- UkraineAlarm public API/application/offer досліджено; credentials не отримувалися, live call не робився.
- alerts.in.ua docs досліджено; токен не в клієнтах, public service потребує relay/proxy.
- KATOTTG: production catalog має бути відтворюваним з Ministry base + applicable amendments, не зі старої копії таблиці.

## Google Drive / backup
Drive root: https://drive.google.com/drive/folders/1sPq7Ajmi4uhydjK7Wy1B-vR5IwTFVt8b  
PROJECT mirror існує. GitHub залишається source of truth.
Native Google Docs snapshot не довів byte-exact source restore; повний raw/git backup + fresh restore test залишається відкритим.

## One next action
Відкрити Issue #9, звірити live PR #1 і current CI, після перевірки почати base-first integration. Паралельну роботу не припускати без прямого рішення власника.

For ordinary continuation: `START_HERE.md → PROJECT_RULES.md → PROJECT_STATE.md → WORKLOG.md → HANDOFF_CURRENT.md → live GitHub`. Центральний dev-strategy не потрібний щодня.