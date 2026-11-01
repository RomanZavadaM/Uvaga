# PROJECT_STATE — ТривогА

**Оновлено:** 2026-11-01  
**Repository:** `RomanZavadaM/Uvaga`  
**Методологія:** dev-strategy v2 RC — пробне застосування, не прийнято повністю.

## Інтегрований стан
Current `main`: `1d7281c944ecfded2d70c7d7540bebdba06b0f15`.

Merged with squash + fresh CI:
- PR #1 — organizational foundation;
- PR #2 — r1 offline alert-state core;
- PR #3 — r2 bounded multi-region store;
- PR #22 — continuity checkpoint after r1/r2 integration;
- PR #8 — explicit source freshness;
- PR #10 — conservative recovery snapshot;
- PR #11 — fail-safe presentation states.

Release/tag, production deployment, live API credential і публічна доставка сповіщень відсутні.

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
- Source freshness окремий від confirmed alert state.
- Немає публічного live military tracking/точних траєкторій.
- До окремого рішення власника: без production і public notifications.

## Режим роботи
Один активний виконавець за замовчуванням. Паралельність — лише за рішенням власника. Одна гілка/матеріал не редагуються одночасно двома виконавцями.

## Поточна інтеграційна черга
Issue #9 — source of truth для порядку.
Наступні slices:
1. #5 source trust/normalization;
2. #7 alerts.in.ua parser;
3. #6 localhost relay;
4. #4 Tauri shell;
5. territory/source/attention #12–#21 після reconciliation з accepted core.

## Перевірено
- Fresh docs + Rust CI для #8, #10, #11 перед merge — PASS.
- Recovery restore скидає freshness до Unknown.
- Stale/unavailable clear не показується як current clear.
- Tauri product-platform checks раніше проходили на Windows x64, macOS ARM64, macOS Intel.
- Google Drive PROJECT mirror readback-перевірений.
- Production/live delivery не запускалась.

## Drive / recovery
GitHub — source of truth.
PROJECT mirror існує.
BACKUPS зараз порожній.
Full raw/git backup + restore test залишається відкритим.

## Відкрите
- Поетапна інтеграція #5/#7/#6/#4 та territory slices із fresh CI.
- Full transitive license audit.
- Full Git/raw backup + restore.
- Android/iOS compile/build validation.
- Live API access/schema/latency/webhook/security/legal acceptance — лише після окремого рішення власника.
- Push/background delivery — не починалась.

## Одна наступна дія
Очистити/ретаргетити PR #5 на current `main`, виконати fresh CI і інтегрувати лише після PASS.