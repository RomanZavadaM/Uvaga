# PROJECT_STATE — ТривогА

**Оновлено:** 2026-11-01  
**Repository:** `RomanZavadaM/Uvaga`  
**Методологія:** dev-strategy v2 RC — пробне застосування, не прийнято повністю.

## Підтверджений інтегрований стан
- `main` містить початковий README; базовий commit: `ad8653ebcdf16507be54a929b690c8a1060e035b`.
- Prototype/docs work ведеться через stacked PR; перед будь-якою інтеграцією звіряти live GitHub.
- Release/tag, production deployment, live API credential і публічна доставка сповіщень відсутні.

## Погоджений продукт
- Public, вся Україна, alerts-plus.
- Офіційні тривоги + перевірена додаткова інформація.
- Desktop + mobile + web; Telegram — пізніше через інтеграційний вихід.
- Rust + Tauri 2; Windows 7 виключено.
- Легкий Rust relay/cache; PostgreSQL не є вимогою.
- Secret third-party tokens не зберігаються у public clients.
- Proprietary: Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.

## Safety invariants
- Timeout, TTL, empty response, 401/403/429 чи втрата джерела не створюють official clear.
- Додаткове/агреговане джерело не може самостійно стати official source.
- Старий clear без свіжого джерела не показується як поточна безпека.
- Немає публічного live military tracking/точних траєкторій.
- До окремого рішення власника: без production і public notifications.

## Режим роботи
Один активний виконавець за замовчуванням. Паралельність — лише за рішенням власника. Перед записом звіряти live branch, історію попередньої сесії та GitHub.

## Поточні slices
Базова лінія: PR #1, #2, #3; далі source/Tauri/relay/freshness/recovery/presentation та territory/source slices. Повний порядок і поточні live heads — Issue #9. Не зберігати тут рухомі SHA як остаточну правду.

## Перевірено
- Docs/core/Tauri/relay/parser/freshness/recovery/presentation та territory slices мають успішні CI checkpoints; кожен current head треба звіряти live перед merge.
- Tauri product-platform checks успішно виконувались на Windows x64, macOS ARM64 та macOS Intel.
- Google Drive PROJECT mirror створено й readback-перевірено.
- KATOTTG та UkraineAlarm source/contract research задокументовано.
- Production/live delivery не перевірялась і не запускалась.

## Build/dependency
Tauri CI-generated Cargo.lock artifact існує; direct dependency pins та попередній license audit зафіксовані. Повний transitive license gate і точний release lock ще залишаються до релізу.

## Drive / recovery
Google Drive PROJECT mirror існує. GitHub — source of truth. Текстовий checkpoint у Drive не дорівнює byte-exact Git backup. Повний raw/git backup + restore test залишається відкритим.

## Відкрите
- Поетапна інтеграція stacked PR із fresh CI.
- Full transitive license audit.
- Full Git/raw backup + restore.
- Android/iOS compile/build validation.
- Live API access/schema/latency/webhook/security/legal acceptance — лише після окремого рішення власника.
- Push/background delivery — не починалась.

## Одна наступна дія
Звірити Issue #9 і live GitHub, перевірити базовий PR #1 та почати base-first integration з fresh CI. Не bulk-merge.