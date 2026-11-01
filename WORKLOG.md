# WORKLOG — ТривогА

## 2026-11-01 — організаційний старт
- Власник погодив public `RomanZavadaM/Uvaga`, Rust + Tauri 2, proprietary license за моделлю Taxo.
- Заборонено production і публічні сповіщення до окремого рішення.
- Створено project-local continuity docs, requirements/architecture/source/test/license docs.
- Створено Google Drive PROJECT/CHECKPOINTS/BACKUPS/VERIFICATION.

## 2026-11-01 — prototype work
- Створено offline Rust alert core, multi-region store, source contract, localhost relay, alerts.in.ua parser, freshness, recovery/presentation state, Tauri shell та territory/source/attention slices.
- CI виявляв реальні Tauri build/icon defects; виправлення перевірені.
- Product-platform Tauri checks проходили на Windows x64, macOS ARM64, macOS Intel.
- Live API tokens, production deployment і public notifications не використовувались.

## 2026-11-01 — source/recovery
- Досліджено UkraineAlarm public API/application/offer та alerts.in.ua public docs.
- Зафіксовано secret-safe relay boundary.
- KATOTTG policy: Ministry base + applicable amendment chain.
- Drive PROJECT mirror записано/readback-перевірено.
- Native Google Docs source snapshot не довів byte-exact restore й був видалений; BACKUPS зараз порожній. Full raw/git backup + restore лишається відкритим.

## 2026-11-01 — execution policy
- Один активний виконавець за замовчуванням.
- Паралельність можлива тільки за рішенням власника.
- Одна гілка/матеріал не редагуються одночасно двома виконавцями.
- Issue #9 веде verification/integration workflow.

## 2026-11-01 — integration cleanup
- PR #1 очищено, fresh docs/core CI PASS, squash merge: `556ae3f09c63cfefd8c2fba57be16dd782517603`.
- PR #2 після squash-base перебудовано force-with-lease як один чистий r1 commit поверх main; diff = 3 files; fresh docs/core CI PASS; squash merge: `b802ee08377f5deff4922c4014801c2b171745f6`.
- PR #3 так само перебудовано як чистий r2 commit; diff = 3 files; fresh docs/core CI PASS; squash merge: `cb7e245eb9e4d1267b74625e67bd1cf7b3d1d47f`.
- Старі merged branches залишаються на GitHub лише тому, що доступний конектор не має delete-branch operation; вони не є активними.

## NEXT
Очистити PR #8 на current main, виконати fresh CI і інтегрувати freshness slice. Далі #10 → #11.