# Tauri shell r3 — non-operational application shell

## Decision
Use Tauri 2 with a static vanilla HTML/CSS frontend for the first shell. This keeps the UI dependency surface small and avoids requiring Node for the static prototype. Root Rust core remains a separate local crate.

## Target matrix
Product-platform compile checks cover:
- Windows x64 (`windows-2025`);
- macOS Apple Silicon (`macos-26`);
- macOS Intel (`macos-26-intel`).

Planned but **not yet validated**:
- Android;
- iOS.

Web remains a separate static/client delivery target rather than a Tauri bundle. Windows 7 is out of scope.

## Security scope
The r3 shell requests no notification, geolocation, filesystem, autostart, network or other plugin permissions. It contains no live API endpoint and no user tracking. UI text states that it is synthetic/test-only.

## Dependency checkpoint
Direct dependencies are pinned:
- `tauri = 2.12.5`;
- `tauri-build = 2.7.4`.

A successful earlier Windows CI produced a `src-tauri/Cargo.lock` artifact:
- Cargo.lock SHA256: `420880998b64ca0e1a882c781b7d4f1577b9873be8b943ae3f425482bfb514fd`;
- resolved tauri: 2.12.5;
- resolved tauri-build: 2.7.4;
- resolved tauri-runtime: 2.12.2;
- resolved wry: 0.57.1.

The artifact was downloaded and its SHA256 rechecked on 2026-11-01. The current GitHub write connector cannot upload that local raw file directly back into the repository, so `Cargo.lock` is **not yet committed**. Dependency reproducibility must not be called complete until the lock file is committed and checked with `--locked`.

## Predeclared acceptance
- Existing Rust core compiles as the local Tauri dependency.
- Tauri crate compiles on all three desktop matrix targets.
- Static UI assets are bundled from `app/`.
- Real PNG/ICO icon assets are present; build.rs does not generate icons.
- No network/push/geolocation capability is introduced.
- UI never describes missing data as safety.
- No production/release bundle is produced.

## Not yet verified
Android/iOS builds, signing, installer/portable packaging, WebView behavior beyond compile-time checks, accessibility beyond static markup, notification APIs, background execution and actual source integration.
