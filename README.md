# ТривогА / Uvaga

**ТривогА** — проєкт системи оперативного інформування населення України про офіційні повітряні тривоги та перевірену додаткову інформацію про загрози.

> **Поточний статус:** ранній prototype / synthetic-only. Немає production deployment, live API credentials або публічної доставки сповіщень. Програма не замінює офіційні канали оповіщення; відсутність повідомлення не означає відсутність небезпеки.

**Основна мова README: Українська.** Переклади: [English](README.en.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

## Завантаження для тестування

### Windows 10/11 x64 — r18 synthetic test build

[**Завантажити Uvaga-Windows-Synthetic-r18.zip**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41415006414/artifacts/13127022813)

Це **не реліз і не production-версія**. Це перевірений synthetic test build r18; ручне Windows-приймання завершене, а зміни інтегровані в `main`. Для завантаження GitHub Actions artifact може знадобитися вхід у GitHub.

- Source head: `a2c945034bfea58b3ff5890ea53862831c1b502a`
- ZIP SHA256: `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`
- `Uvaga.exe` SHA256: `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`
- Автоматичні Windows/macOS перевірки: **PASS**
- Ручне Windows runtime-приймання r18: **PASS — 2026-11-02**
- r18 інтегровано в `main`: `61c0662d941e388858f5214c14860ec92a586905`

У тестовій оболонці можна вибрати синтетичну територію (`UA-46` або `UA-30`) і перевірити п’ять fail-safe сценаріїв: active/fresh, active/stale, cleared/fresh, cleared/stale та unknown/unavailable.

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

Незалежне приймання та інтеграція контролюються через [GitHub Issue #9](https://github.com/RomanZavadaM/Uvaga/issues/9).

## Методологія

Проєкт проходить **пробне застосування dev-strategy v2 RC**. C13 clean-session continuity — **PASS**. Локальні правила самодостатні для щоденної роботи; центральний `dev-strategy` не є runtime-залежністю кожної сесії. Повне прийняття v2 RC власником (C14) ще не оголошено.

## Ліцензія

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

ТривогА — **proprietary software**. Публічна видимість репозиторію не надає open-source ліцензію. Див. [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md) та [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
