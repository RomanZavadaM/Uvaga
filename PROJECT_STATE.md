# PROJECT_STATE — ТривогА
Дата: 2026-11-01. Статус: організаційний docs-only checkpoint, PR очікує перевірки; немає робочого продукту або production.
Репозиторій: RomanZavadaM/Uvaga. Ініціальний main SHA: ad8653ebcdf16507be54a929b690c8a1060e035b. Поточні SHA та PR потрібно щоразу звіряти live.
Обрано: Україна, public, alerts-plus, desktop/mobile/web; Telegram пізніше; Rust + Tauri 2 як попередня архітектура; жодного PostgreSQL. Ліцензія власницька за моделлю Taxo.
Відкрито: перевірка офіційних API, платформи/CI, незалежне тестування, Google Drive дзеркало та backup, вибір конкретного transport/push для mobile; реальна реалізація не починалась.
Методологія: v2 RC — trial / pending owner acceptance.
Одна наступна дія: провести незалежне docs-only review і рішення щодо інтеграції PR, потім окремо погодити перший кодовий етап.

## Active development slices — 2026-11-01
Nothing below is integrated into main yet.
- PR #1 docs organizational start — draft; docs CI 41337588598 SUCCESS.
- PR #2 r1 offline single-region Rust state — draft; core CI 41335891826 SUCCESS on tested code head.
- PR #3 r2 bounded multi-region store — draft; core CI 41336873203 SUCCESS.
- PR #4 r3 Tauri 2 offline shell — draft; first two CI attempts exposed missing icon; icon added in commit 8a6b8cd1cc4ffe1036323c53578a1b5a5d33424b; latest CI pending.
- PR #5 r4 source trust/normalization contract — draft; core CI 41337182862 SUCCESS.
- PR #6 r5 localhost synthetic relay — draft; relay CI 41337322695 SUCCESS.
- PR #7 r6 alerts.in.ua v2 compact parser — draft; core CI 41337434827 SUCCESS.
- PR #8 r7 explicit source freshness — draft; CI pending.
No live API, production deployment or public notification has been introduced.
