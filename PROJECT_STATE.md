# PROJECT_STATE — ТривогА

**Оновлено:** 2026-11-01  
**Repository:** `RomanZavadaM/Uvaga`  
**Методологія:** dev-strategy v2 RC — пробне застосування, не прийнято повністю.

## Інтегрований стан
- `main`: `cb7e245eb9e4d1267b74625e67bd1cf7b3d1d47f`.
- PR #1 — organizational foundation — merged.
- PR #2 — r1 offline alert-state core — merged.
- PR #3 — r2 bounded multi-region store — merged.
- Усі три інтегровані squash після fresh CI.
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
Один активний виконавець за замовчуванням. Паралельність — лише за рішенням власника. Одна гілка/матеріал не редагуються одночасно двома виконавцями.

## Поточна інтеграційна черга
Issue #9 — source of truth для порядку.
Наступна core-лінія: #8 freshness → #10 recovery → #11 presentation, після чого source contract / parser / relay / Tauri та territory slices.

## Перевірено
- Fresh CI для PR #1, #2, #3 перед merge — PASS.
- Tauri product-platform checks раніше проходили на Windows x64, macOS ARM64 та macOS Intel.
- Google Drive PROJECT mirror створено й readback-перевірено.
- KATOTTG та UkraineAlarm source/contract research задокументовано.
- Production/live delivery не запускалась.

## Drive / recovery
GitHub — source of truth.
PROJECT mirror існує.
Невдалий native-Google-Docs source snapshot видалений; `BACKUPS` зараз порожній.
Full raw/git backup + restore test залишається відкритим.

## Відкрите
- Поетапна інтеграція наступних slices із fresh CI.
- Full transitive license audit.
- Full Git/raw backup + restore.
- Android/iOS compile/build validation.
- Live API access/schema/latency/webhook/security/legal acceptance — лише після окремого рішення власника.
- Push/background delivery — не починалась.

## Одна наступна дія
Очистити/ретаргетити PR #8 на current `main`, виконати fresh CI і інтегрувати лише після PASS.