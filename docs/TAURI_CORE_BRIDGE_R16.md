# Tauri ↔ core bridge r16 — offline end-to-end slice

## Purpose
Prove that the bundled vanilla-JS UI can read a fail-safe synthetic status produced by the real Rust core through Tauri IPC.

No HTTP server, external API, credential, push notification, geolocation or production source is introduced.

## Flow
`Rust core → private Tauri command → JSON string → window.__TAURI__.core.invoke → textContent UI`

The command constructs one synthetic `UA-46` active event with `SourceFreshness::Stale`. The shared core derives `ActiveLastKnown`. The frontend displays those three distinct facts:
- confirmed state = active;
- source freshness = stale;
- presentation = active last-known.

## Tauri configuration
Vanilla JavaScript uses the supported global Tauri API:
- `app.withGlobalTauri = true`;
- IPC command is invoked through `window.__TAURI__.core.invoke`;
- CSP explicitly allows Tauri IPC via `connect-src ipc: http://ipc.localhost`;
- scripts/styles remain bundled local assets;
- no plugin capabilities are added.

References checked 2026-11-01:
- https://v2.tauri.app/reference/javascript/api/
- https://v2.tauri.app/develop/calling-rust/
- https://v2.tauri.app/security/csp/

## Safety
- The UI never calculates official safety itself.
- Presentation is derived in Rust by the shared core.
- IPC failure displays an error saying that failure does **not** mean clear.
- DOM updates use `textContent`, not dynamic HTML.
- The command accepts no user arguments and performs no I/O.
- The payload is explicitly marked `synthetic=true`.

## Predeclared acceptance
1. Windows runs the Tauri bridge unit test and passes.
2. macOS ARM64 and Intel compile the shell.
3. Tauri config enables global API and IPC-only CSP without remote origins.
4. UI loads the synthetic snapshot through `invoke("demo_snapshot")`.
5. Snapshot is active + stale + active_last_known.
6. Failure path never displays a clear.
7. No network/plugin/push/geolocation capability is introduced.
8. No production bundle/release is produced.

## Not yet in scope
Live source adapters, persistent settings, notification delivery, Android/iOS runtime, signed installers and production UX.
