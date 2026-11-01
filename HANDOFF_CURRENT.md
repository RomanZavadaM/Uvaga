# HANDOFF_CURRENT — ТривогА
Дата: 2026-11-01. Стандарт: dev-strategy v2 RC — пробне застосування, **не прийнято повністю**.
Головне джерело істини: https://github.com/RomanZavadaM/Uvaga. Перед діями звірити живі SHA, PR, CI.
Ініціальний main: ad8653ebcdf16507be54a929b690c8a1060e035b. Draft PR #1: https://github.com/RomanZavadaM/Uvaga/pull/1, гілка docs/organizational-start-v2-rc-20261101. Точний HEAD звірити при старті.
Погоджено: загальноукраїнська система для населення; офіційна тривога + додаткова перевірена інформація; web/desktop/mobile, Telegram пізніше; Rust + Tauri 2 попередньо; proprietary Roman Zavada.
Заборонено: production, публічні сповіщення, живе військове стеження та секрети в репозиторії; наразі лише docs-only.
Перевірено: у GitHub PR перераховує 15 docs-файлів, чотири ключові прочитані повторно. Доказ VERIFICATION.md. НЕ перевірено: незалежне рев'ю, CI, код, API, функціональна доставка, restore.
Google Drive: https://drive.google.com/drive/folders/1sPq7Ajmi4uhydjK7Wy1B-vR5IwTFVt8b — папки створені, файли **ще не синхронізовані**; BACKUPS порожня.
Доповнено правила SOURCE_VALIDATION.md і тестові сценарії TEST_PLAN.md. Незалежне рев'ю іншою особою/ШІ поки не проведено.
Наступна одна дія: незалежне рев'ю PR #1, виправлення та рішення власника про merge. Після цього підтверджувана одностороння Drive синхронізація та окремий backup/restore test.
Для продовження: START_HERE.md → PROJECT_RULES.md → PROJECT_STATE.md → WORKLOG.md + цей HANDOFF + live GitHub. Центральний dev-strategy не потрібний щодня.
Кодовий етап: відкрито stacked draft PR #2 https://github.com/RomanZavadaM/Uvaga/pull/2 (base — гілка PR #1). Rust core лише для синтетичних подій. CI 41335891826 SUCCESS на попередньому HEAD 0a434aaeee8681a85db6d3752ea5a3e4f37f0b38. Після запису цього HANDOFF HEAD змінився, перевірити останню CI. Незалежне рев'ю ще не проведено. Порядок: прийняти PR #1, потім перенаправити PR #2 на main і перевірити повторно; без production.
