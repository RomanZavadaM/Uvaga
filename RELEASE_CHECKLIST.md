# RELEASE_CHECKLIST — ТривогА

> Цей список є release gate, а не дозволом на реліз. Public release, code signing і live-source integration виконуються лише після окремого рішення власника.

## A. Freeze / source identity

- [ ] GitHub `main` звірений безпосередньо перед release candidate.
- [ ] Немає неочікуваних відкритих PR/гілок, які мають увійти в реліз.
- [ ] Exact release source SHA записаний у verification/state.
- [ ] Версія погоджена власником.
- [ ] `src-tauri/Cargo.toml` і `src-tauri/tauri.conf.json` мають однакову версію; Release metadata checks = PASS.
- [ ] Release docs/README/SECURITY/LICENSE/THIRD_PARTY_NOTICES завершені **до** фінального installer candidate.

## B. Product / safety

- [ ] Rust core tests = PASS.
- [ ] UI contract = PASS.
- [ ] Tauri Windows + macOS platform checks = PASS.
- [ ] Critical invariant: stale/unavailable previous clear не відображається як current safe clear.
- [ ] IPC/error path не залишає старий clear як актуальний стан.
- [ ] Synthetic warning залишається очевидним.
- [ ] Live API / credentials / push / geolocation не з'явились без окремо погодженого етапу.

## C. Dependency / legal

- [ ] Exact final Cargo.lock SHA256 зафіксований.
- [ ] Dependency source/license policy = PASS для exact final lock.
- [ ] Target-specific full third-party license coverage = PASS для фінальної Windows збірки.
- [ ] MPL source-availability material відповідає exact final lock.
- [ ] Для production визначено non-expiring location для license/MPL material.
- [ ] Proprietary LICENSE/COPYRIGHT/THIRD_PARTY_NOTICES включені у distribution package.

## D. Final Windows installer candidate

- [ ] Після freeze docs/version запущено `Windows NSIS installer candidate` через `workflow_dispatch` з exact release ref.
- [ ] Installer build = PASS.
- [ ] Authenticode state відповідає затвердженому режиму: unsigned test candidate або signed production candidate.
- [ ] Silent current-user install = PASS.
- [ ] HKCU registration = PASS.
- [ ] Installed app launch smoke = PASS.
- [ ] Silent uninstall = PASS.
- [ ] Uninstall не залишає application EXE / HKCU registration.
- [ ] Artifact містить BUILD_INFO, SHA256SUMS, LICENSE, THIRD_PARTY_NOTICES, SECURITY, Windows distribution notes і third-party license texts.

## E. Artifact readback / backup

- [ ] GitHub artifact завантажено назад.
- [ ] GitHub artifact ZIP SHA256 зафіксований.
- [ ] Усі внутрішні SHA256SUMS = PASS.
- [ ] Installer SHA256 і release EXE SHA256 зафіксовані.
- [ ] Drive BACKUPS copy створено.
- [ ] Drive copy завантажено назад і byte-identical до GitHub artifact.

## F. Owner manual acceptance

- [ ] Owner запускає exact checksum-verified installer на звичайному Windows 10/11 x64.
- [ ] Installer UX: ім'я, іконка, мова, current-user behavior — PASS.
- [ ] Застосунок стартує після install.
- [ ] Усі 5 synthetic fail-safe сценаріїв — PASS.
- [ ] UA-46 / UA-30 selection — PASS.
- [ ] Critical cleared-stale scenario показує повторне підтвердження, не current safe clear.
- [ ] Uninstall UX — PASS.
- [ ] Reinstall/upgrade behavior — PASS, якщо це входить у конкретний реліз.

## G. Production-only gates

- [ ] Owner окремо схвалив public release.
- [ ] Code-signing рішення прийняте; production candidate підписаний і перевірений, якщо signing required.
- [ ] Final public version/tag/release notes погоджені.
- [ ] Non-expiring license/MPL links доступні з release.
- [ ] Backup/restore checkpoint виконаний для exact release source.
- [ ] Після release перевірено, що опублікований asset SHA256 збігається з accepted candidate.

## Stop conditions

Release зупиняється, якщо:
- source SHA/версія/lock не збігаються з evidence;
- будь-який required CI gate FAIL/UNKNOWN;
- stale clear може виглядати як current safe clear;
- artifact/readback hash не збігається;
- owner manual acceptance не завершене;
- signing/legal/source-availability вимоги для обраного distribution mode не закриті.

CI PASS сам по собі **не** є дозволом на public release і не є дозволом підключати live-джерела.
