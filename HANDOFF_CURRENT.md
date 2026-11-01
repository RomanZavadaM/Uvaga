# HANDOFF_CURRENT — ТривогА
Дата: 2026-11-01. Стандарт: dev-strategy v2 RC — пробне застосування, **не прийнято повністю**.

## Джерело істини
GitHub: https://github.com/RomanZavadaM/Uvaga
Поточні live SHA/PR/CI завжди перевіряти перед записом. main досі містить лише початковий README, якщо live GitHub не показує інше.

## Погоджено
Україна/public; офіційна тривога + додаткова перевірена інформація; web/desktop/mobile; Telegram пізніше; Rust + Tauri 2; proprietary Roman Zavada; Windows 7 виключено. Легкий Rust relay/cache замість PostgreSQL; секретні API tokens не в клієнтах.

## Заборонено без окремого рішення
Production, публічні сповіщення, live credentials, реальні third-party API calls, публічне військове стеження/точні траєкторії, секрети в GitHub/Drive.

## Active drafts
- #1 organizational docs
- #2 r1 offline state core
- #3 r2 multi-region store
- #4 Tauri shell
- #5 source trust/normalization
- #6 localhost synthetic relay
- #7 alerts.in.ua v2 parser
- #8 source freshness states
Review gate: Issue #9 https://github.com/RomanZavadaM/Uvaga/issues/9

Verified green CI: docs 41337682369; r1 41335891826; r2 41336873203; Tauri Linux compile checkpoint 41337554330 after icon fix; r4 41337182862; r5 41337322695; r6 41337434827; r7 41337648092. PR #4 later changed to Windows/macOS product-platform matrix — recheck its newest head and CI live before any integration.

## Drive / backup
Drive root: https://drive.google.com/drive/folders/1sPq7Ajmi4uhydjK7Wy1B-vR5IwTFVt8b
PROJECT mirror now contains START/RULES/STATE/WORKLOG/HANDOFF and readback matches normalized GitHub text with recorded blob SHA. Verification: https://docs.google.com/document/d/1W7cGX-l0ozK7Se8Q0OEN8WM-WqanVBwv-V53JRqAlgA/edit
Full Git-history independent backup/restore is still OPEN; container GitHub DNS blocked the attempted git bundle.

## One next action
Open Issue #9 and independently review PR #1 first. Do not merge stacked children out of order. After accepted base merge, retarget child to main and require fresh CI. Separately complete full Git bundle/restore in an environment with GitHub network access.

For ordinary continuation: START_HERE.md → PROJECT_RULES.md → PROJECT_STATE.md → WORKLOG.md → this HANDOFF + live GitHub. Центральний dev-strategy не потрібний щодня.
