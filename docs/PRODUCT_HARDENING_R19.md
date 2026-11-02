# Product hardening r19

## Мета

Зробити synthetic desktop build зрозумілішим, компактнішим і стійкішим до UI-регресій без розширення live/production scope.

## Predeclared acceptance

1. Критичний стан, територія і свіжість джерела видимі у верхній частині робочої області без необхідності читати технічні пояснення.
2. active/current, active/stale, clear/current, clear/stale і unknown мають семантичне представлення, яке не залежить лише від кольору.
3. `cleared_stale` текстово лишається «Відбій потребує повторного підтвердження»; жоден UI-hardening не може перетворити його на актуальний відбій.
4. IPC error очищає попередній стан і показує unknown/error, не clear.
5. Form controls мають явні labels/ids; status updates доступні через live region; loading state позначається `aria-busy`.
6. Технічні пояснення не займають основну висоту екрана за замовчуванням.
7. UI показує локальну версію застосунку з Rust payload; це не production version claim.
8. Static UI contract test перевіряє whitelist двох synthetic territories, усі п’ять scenario IDs, critical stale-clear label, synthetic warning і відсутність remote scripts/styles.
9. Windows Tauri tests/build, Windows synthetic artifact, macOS ARM64/Intel checks і docs/UI-contract CI — PASS.
10. README UA/EN/FR/DE/ES/KO/JA не повинні називати r18 manual acceptance pending або PR #34 відкритим.
11. Live source adapter, credentials, public notifications, geolocation, production deployment, signing і production release не додаються в r19.

## Manual acceptance

r19 не змінює Rust safety mapping. Після CI потрібна візуальна sanity-перевірка synthetic build перед оголошенням UI hardening прийнятим власником; якщо власника немає, PR може бути інтегрований лише як automated hardening checkpoint, без заяви про новий production release.
