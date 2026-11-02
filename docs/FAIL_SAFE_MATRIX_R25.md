# r25 — fail-safe presentation matrix acceptance

## Scope

Owner-independent automated regression hardening only. Extend tests in `src/presentation.rs`, without changing production code, app version, dependencies, live adapters or notification behavior.

## Acceptance criteria

- Cover all 12 combinations: three official alert states × four source freshness states.
- For every combination verify exact `PresentationState` and that `may_show_as_safe()` is true **only** for a confirmed clear paired with a fresh source.
- Verify that a previously fresh confirmed clear transitions to `ClearNeedsRefresh` when freshness is stale, unavailable or unknown.
- Rust core offline tests pass.
- Because root `src/**` is consumed by Tauri, all r23 desktop CI gates must execute and pass on the exact PR head.
- No public release, code signing, live data, credentials, notifications or version bump.

Safety invariant: a stale/unavailable previous clear must never appear as a current safe clear.
