# r23 — root core → desktop CI coverage

## Мета

Закрити trigger coverage-gap: desktop/Tauri package `uvaga-app` має пряму path dependency `uvaga-core = { path = ".." }`, тому зміна root Rust core повинна перевірятися не лише Linux core CI, а й desktop/Tauri gates.

## Predeclared acceptance

1. `Cargo.toml` і `src/**` запускають Windows synthetic workflow.
2. `Cargo.toml` і `src/**` запускають Windows installer candidate workflow.
3. `Cargo.toml` і `src/**` запускають Tauri Windows + macOS platform matrix.
4. `scripts/verify_tauri_lock.py` запускає Tauri platform matrix.
5. Cheap `CI scope checks` fail-closed, якщо root core coverage з будь-якого з трьох desktop workflows буде випадково прибрано.
6. Docs-only paths лишаються виключеними з heavy Windows workflows.
7. Existing `workflow_dispatch` лишається доступним.
8. Standalone synthetic workflow і Windows Tauri matrix не видаляються; r23 не повертає aggressive consolidation з superseded PR #51.
9. No product/safety logic, Cargo.lock, app version, signing, updater, live source, credentials, notifications or production deployment change.

## Evidence boundary

Цей етап змінює лише CI trigger contract/guard. Safety behavior продукту не змінюється.
