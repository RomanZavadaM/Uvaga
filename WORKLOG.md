# WORKLOG — ТривогА

## 2026-11-01 — організаційний старт
- Власник погодив public `RomanZavadaM/Uvaga`, Rust + Tauri 2, proprietary license за моделлю Taxo.
- Заборонено production і публічні сповіщення до окремого рішення.
- Створено initial README/main, docs branch, PR #1, START/RULES/STATE/WORKLOG/HANDOFF, requirements/architecture/source/test/license docs.
- Створено структуру Google Drive PROJECT/CHECKPOINTS/BACKUPS/VERIFICATION.

## 2026-11-01 — prototype slices
- Створено малими PR offline Rust alert core, multi-region store, source contract, localhost synthetic relay, alerts.in.ua parser, freshness, recovery/presentation state, Tauri shell та подальші territory/source/attention slices.
- CI неодноразово запускався на GitHub; фактичні помилки Tauri icon/build були знайдені й виправлені.
- Product-platform Tauri checks успішно проходили на Windows x64, macOS ARM64, macOS Intel.
- Жодного live API token, production deployment чи public notification не використовувалось.

## 2026-11-01 — source/recovery work
- Досліджено UkraineAlarm public API/application/offer та alerts.in.ua public docs.
- Зафіксовано secret-safe relay boundary.
- Визначено політику актуального KATOTTG: Ministry base + applicable amendment chain.
- Drive PROJECT mirror записано/readback-перевірено.
- Створено текстовий Drive snapshot із blob-SHA markers; readback показав, що native Google Docs не доводить byte-exact restore. Full raw/git backup + restore лишається відкритим.

## 2026-11-01 — режим роботи
- Один активний виконавець за замовчуванням.
- Паралельність можлива тільки за рішенням власника; одна гілка/матеріал не редагуються одночасно двома виконавцями.
- Issue #9 веде звичайний verification/integration workflow.

## NEXT
Звірити current PR #1 / Issue #9 / CI і почати поетапну base-first integration. Після кожного base merge — retarget наступного slice, fresh CI, перевірка, лише потім наступний merge.