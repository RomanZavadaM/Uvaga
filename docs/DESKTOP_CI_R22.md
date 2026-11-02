# r22 — desktop CI consolidation

## Мета

Зменшити дубльовані Windows compilation minutes без послаблення desktop/safety/release gates і закрити coverage-gap для змін root uvaga-core.

## Predeclared acceptance

1. Активний Windows desktop CI має один канонічний pipeline: installer candidate.
2. Окремий active `windows-synthetic-artifact.yml` видаляється; історичні r17/r18/r19/r21 artifacts та hashes лишаються immutable evidence.
3. Installer candidate продовжує: metadata gate, exact lock, cargo test, release build, unsigned verification, install/launch/uninstall smoke, full license packaging, checksums.
4. Installer candidate additionally triggers on root `Cargo.toml` and `src/**`, because desktop app depends on uvaga-core.
5. Tauri platform matrix стає macOS-only: Apple Silicon + Intel. Windows не дублюється там, бо canonical Windows candidate already tests/builds/installs.
6. macOS Tauri check also triggers on root `src/**` and release/lock verifier changes.
7. Root `rust-core.yml` remains Linux core test gate.
8. `relay.yml` remains independent relay gate.
9. UI contract and release metadata checks remain independent cheap gates.
10. Release metadata checker enforces that retired synthetic workflow does not reappear and installer candidate identity stays generic.
11. For a desktop/core code change, expected heavy topology: one Windows candidate + two macOS checks, not three Windows builds + two macOS checks.
12. No product/safety logic, app version, accepted Cargo.lock, signing, updater, live source or production scope changes.

## Evidence boundary

Historical manually accepted r18 synthetic artifact remains valid historical evidence. r22 changes only active CI topology for future source heads.
