# ТривогА / Uvaga

**ТривогА** — проєкт системи оперативного інформування населення України про офіційні повітряні тривоги та перевірену додаткову інформацію про загрози.

> **Поточний статус:** ранній prototype / synthetic-only. Немає production deployment, live API credentials або публічної доставки сповіщень. Програма не замінює офіційні канали оповіщення; відсутність повідомлення не означає відсутність небезпеки.

**Основна мова README: Українська.** Переклади: [English](README.en.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

## Завантаження для тестування

### Windows 10/11 x64 — r20 NSIS installer candidate

[**Завантажити Uvaga-Windows-Installer-r20.zip**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41447023597/artifacts/13142977634)

Це **unsigned test installer, не production release**. CI уже перевірив current-user silent install, HKCU registration, запуск встановленого застосунку та silent uninstall. Ручне owner-приймання installer UX ще не виконане. r20 лишається останнім заархівованим installer test candidate з повним automated install/uninstall evidence; r21–r23 були CI/release-process hardening і не створювали нового owner-accepted release artifact.

- Source head: `96a7953a2388811716c9c13001fcf9ca3d5b79c2`
- r20 merge: `e913edf6bb62249806afc7c19ac8c19e2cde1166`
- ZIP SHA256: `a8a00e0c7f20282d13673ba1fd3de353d4500d9cf36b1d40ed7be3c86ee33d3f`
- Installer SHA256: `dc8f128e73983d28c4093edd0434d4a8eed140a29718cd09faa99c13bf9f8588`
- Release `Uvaga.exe` SHA256: `636aa7d7ec6e5cf485dfda219d107115dba7d3dc79c0b5a995d4e08efc4aab30`
- Exact `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`
- NSIS install/launch/uninstall smoke: **PASS**
- Owner installer/visual sanity: **PENDING**

Drive backup: https://drive.google.com/file/d/1A0fAVnfmgnK6xfVwTlXB-muZxTnGrTuX/view?usp=drivesdk

### Windows 10/11 x64 — r19 synthetic test build

[**Завантажити Uvaga-Windows-Synthetic-r19.zip**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41442359441/artifacts/13140138112)

Це **не реліз і не production-версія**. Це історичний r19 synthetic desktop-hardening build, збережений як перевірюване evidence. r18 залишається останньою версією, яку власник вручну прийняв на Windows; для r19 owner visual sanity ще не виконане. Для завантаження GitHub Actions artifact може знадобитися вхід у GitHub.

- Source head: `cc4992975b73f0211cdd44b05f7ebd52d76a5415`
- ZIP SHA256: `7f2dc03cd58686e2555026a544d78673f9fdcf7b452c44913b40013ee34bffad`
- `Uvaga.exe` SHA256: `c9afe5873aad563dd6f07b51a6b05cc64fac3057e91f1127d2794b3372b7b593`
- `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`
- UI contract / Windows / macOS ARM / macOS Intel: **PASS**
- r19 інтегровано в `main`: `1af2e5d2c53a00619172056bab9fde746daff988`
- Owner visual sanity r19: **PENDING**
- r18 owner manual runtime acceptance: **PASS — 2026-11-02**

r19 робить критичний стан компактнішим і доступнішим, додає fail-closed dependency-lock gate та самодостатній test artifact. Synthetic території й п’ять fail-safe сценаріїв залишаються локальними; live-джерела не підключені.

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
