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

The clean-rebuild Windows CI (run `41347909709`, PR #4 head `f8a0f66c95be4b8ca311aad31cea9053ba735dbf`) produced a `src-tauri/Cargo.lock` artifact:
- Cargo.lock SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- resolved tauri: 2.12.5;
- resolved tauri-build: 2.7.4;
- resolved tauri-runtime: 2.12.2;
- resolved wry: 0.57.1.

The artifact was downloaded and its SHA256 rechecked on 2026-11-01. A raw ZIP copy is stored in Google Drive `BACKUPS` as `Uvaga-Tauri-Cargo-lock-f8a0f66c-20261101.zip`. The current GitHub write connector cannot upload that local raw file directly back into the repository, so `Cargo.lock` is **not yet committed**. Dependency reproducibility must not be called complete until the lock file is committed and checked with `--locked`.

The earlier `42088099…` lock is historical; the clean rebuild resolved `crc32fast` from 1.5.3 to 1.5.4, which changed the lock hash while direct Tauri pins remained unchanged.

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


## Clean-rebuild gate
This PR was rebuilt onto the accepted current `main`. The historical matrix results are evidence only; integration requires a new Windows/macOS matrix PASS for the current PR head.


## CI trigger policy
The full Windows/macOS product-platform matrix runs when the Tauri shell, static app assets, root Cargo manifest, or this workflow changes. Pure root `src/**` core changes are covered by Rust core tests and relay compatibility and do **not** run the expensive three-platform Tauri matrix by default. If core later gains platform-specific code, this trigger policy must be revisited explicitly.
