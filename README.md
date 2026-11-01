# ТривогА / Uvaga

**ТривогА** — проєкт системи оперативного інформування населення України про офіційні повітряні тривоги та перевірену додаткову інформацію про загрози.

> **Поточний статус:** ранній prototype / synthetic-only. Немає production deployment, live API credentials або публічної доставки сповіщень. Програма не замінює офіційні канали оповіщення; відсутність повідомлення не означає відсутність небезпеки.

## Заплановані платформи

- Windows 10/11 x64
- macOS Apple Silicon
- macOS Intel
- Android
- iOS
- Web
- Telegram — пізніше через інтеграційний вихід

Windows 7 не підтримується.

## Архітектурний напрям

- Rust — core логіка стану, перевірки та нормалізації.
- Tauri 2 — desktop/mobile application shell.
- Легкий контрольований Rust relay/cache — межа для secret API tokens, rate limits і нормалізованого public contract.
- PostgreSQL не є базовою вимогою. Оперативний стан — короткоживучий; recovery storage визначається окремо.
- GitHub — source of truth; Google Drive — одностороннє mirror ключової документації, не альтернативний редактор.

Деталі: [REQUIREMENTS.md](REQUIREMENTS.md) · [ARCHITECTURE.md](ARCHITECTURE.md) · [SOURCE_VALIDATION.md](SOURCE_VALIDATION.md).

## Safety invariants

- Офіційний відбій приймається тільки як новіша підтверджена офіційна подія.
- Timeout, TTL, HTTP error, порожня відповідь або недоступність джерела **не створюють відбій**.
- Додаткове/агреговане джерело не може самостійно підмінити official source.
- Старий підтверджений clear без свіжого джерела не показується як поточний safe state.
- Проєкт не призначений для публічного live-відстеження військових об'єктів, точних координат чи траєкторій.

## Розробка

Початкова точка для будь-якої нової сесії: **[START_HERE.md](START_HERE.md)**.

Далі:
1. [PROJECT_RULES.md](PROJECT_RULES.md)
2. [PROJECT_STATE.md](PROJECT_STATE.md)
3. [WORKLOG.md](WORKLOG.md)
4. [HANDOFF_CURRENT.md](HANDOFF_CURRENT.md)
5. live GitHub PR/CI

Незалежне приймання активної стартової серії контролюється через GitHub Issue #9.

## Методологія

Проєкт проходить **пробне застосування dev-strategy v2 RC**. Локальні правила самодостатні для щоденної роботи; центральний `dev-strategy` не є runtime-залежністю кожної сесії. Повне прийняття v2 RC ще не оголошено.

## Ліцензія

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

ТривогА — **proprietary software**. Публічна видимість репозиторію не надає open-source ліцензію. Див. [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md) та [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
