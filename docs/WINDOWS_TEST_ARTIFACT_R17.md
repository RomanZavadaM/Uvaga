# Windows synthetic test artifact r17

## Purpose
Produce a real Windows x64 executable for **manual synthetic runtime acceptance** of the r16 Tauri UI ↔ Rust core bridge.

This is not a release, installer or production build.

## Build gate
On `windows-2025` CI:
1. generate `src-tauri/Cargo.lock`;
2. run Tauri bridge tests with `--locked`;
3. build an unsigned debug executable with `--locked`;
4. copy `Uwaga.exe`, `Cargo.lock`, `SHA256SUMS.txt` and `TEST_ARTIFACT.txt` into one artifact;
5. retain the artifact for 30 days.

## Expected manual behavior
- application starts on Windows 10/11 x64;
- banner clearly says synthetic mode / not real alerting;
- UI obtains state through local Tauri IPC;
- displayed synthetic state is:
  - confirmed = active;
  - freshness = stale;
  - presentation = active last-known;
- pressing the refresh button repeats the local IPC read;
- no network permission/source is required;
- IPC failure must show an error and must not display a clear.

## Packaging/security
- unsigned debug executable only;
- no MSI/NSIS installer;
- no auto-update;
- no live API credentials;
- no push notification or geolocation permission;
- no production endpoint.

Tauri uses the system WebView2 runtime on Windows. Current Tauri documentation notes WebView2 is normally already present on supported modern Windows versions:
https://v2.tauri.app/start/prerequisites/

## Acceptance
CI artifact creation PASS is only a build checkpoint. Final r17 acceptance requires a human manual run of the artifact and explicit observation of the expected UI behavior.
