# PROJECT_STATE — ТривогА

**Оновлено:** 2026-11-01  
**Repository:** `RomanZavadaM/Uvaga`  
**Методологія:** dev-strategy v2 RC — пробне застосування, не прийнято повністю.

## Підтверджений інтегрований стан
- `main` створений початковим docs-only README; базовий commit: `ad8653ebcdf16507be54a929b690c8a1060e035b`.
- Жоден prototype PR #1–#11 не злитий у `main`.
- Немає release/tag, production deployment, live API credential або публічної доставки сповіщень.

## Погоджений продукт
- Аудиторія: public, територія: вся Україна.
- Обсяг: офіційні тривоги + перевірена додаткова інформація.
- Канали: desktop, mobile, web; Telegram — пізніше через інтеграційний вихід.
- Попередній стек: Rust + Tauri 2; Windows 7 виключено.
- Легкий Rust relay/cache; PostgreSQL не є вимогою. Секретні third-party tokens не в public clients.
- Proprietary: Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.

## Незмінні safety rules
- Timeout, TTL, empty response, 401/403/429 чи втрата джерела не створюють official clear.
- Додаткове/агреговане джерело не може самостійно стати official source.
- Старий clear без свіжого джерела не показується як поточна безпека.
- Немає публічного live military tracking/точних траєкторій.
- До окремого рішення власника: без production і public notifications.

## Активні draft PR
- #1 — organizational docs.
- #2 — r1 offline alert-state core.
- #3 — r2 bounded multi-region store.
- #4 — r3 Tauri 2 offline shell.
- #5 — r4 source trust/normalization.
- #6 — r5 localhost synthetic relay.
- #7 — r6 alerts.in.ua v2 compact parser.
- #8 — r7 explicit source freshness.
- #10 — r8 conservative recovery snapshot.
- #11 — r9 fail-safe presentation state.
- Issue #9 — independent review/integration gate.

## Перевірені CI checkpoints
- Docs: run `41338991422` — SUCCESS.
- r1 core: `41335891826` — SUCCESS.
- r2 multi-region: `41336873203` — SUCCESS.
- Tauri product-platform matrix: `41338709796` — SUCCESS на Windows 2025 x64, macOS 26 ARM64, macOS 26 Intel.
- r4 source contract: `41337182862` — SUCCESS.
- r5 local relay: `41337322695` — SUCCESS.
- r6 alerts parser: `41337434827` — SUCCESS.
- r7 freshness: `41337648092` — SUCCESS.
- r8 recovery після consistency fix: `41338929570` — SUCCESS.
- r9 presentation: `41338934425` — SUCCESS.

## Build/dependency state
PR #4 згенерував CI Cargo.lock artifact:
- Cargo.lock SHA256 `420880998b64ca0e1a882c781b7d4f1577b9873be8b943ae3f425482bfb514fd`.
- artifact ZIP SHA256 `f4ab56495655c1e0a6b1c8d312940c4776697d06d7491961d8ff6bd0931d464e`.
- direct pins: tauri 2.12.5, tauri-build 2.7.4.
Повний Cargo.lock ще не закомічений; dependency reproducibility не вважати завершеною.
Direct-license audit записаний у `docs/DEPENDENCY_LICENSE_AUDIT_20261101.md`; full transitive license gate ще не виконаний.

## Drive / recovery
Google Drive PROJECT mirror для START/RULES/STATE/WORKLOG/HANDOFF створений і readback-перевірений. GitHub — source of truth.
Повний незалежний Git-history bundle + restore test НЕ виконаний: контейнерний Git не мав DNS-доступу до github.com.

## Відкрите
- Незалежне review іншою особою/ШІ за Issue #9.
- Правильна поетапна інтеграція stacked PR з fresh CI після retarget.
- Commit точних Cargo.lock перед release.
- Full transitive license audit.
- Full Git backup/restore.
- Android/iOS compile/build validation.
- Отримання/перевірка live API доступу, latency, schema, webhook/security та legal acceptance.
- Push/background delivery — не починалась.

## Одна наступна дія
Незалежно перевірити PR #1 за Issue #9. Лише після приймання бази — merge/retarget наступного slice та fresh CI; не зливати stacked PR поза порядком.
