# Local relay r5 — synthetic HTTP prototype

## Purpose
Prove the boundary between public clients and future secret-bearing source adapters without using real credentials or third-party APIs.

## Behavior
- Binds only to `127.0.0.1:8787`.
- `GET /health` returns `synthetic-local-only`.
- `GET /v1/demo/status/{region}` returns a synthetic normalized region snapshot.
- Contract exposes three separate concepts:
  - `confirmed_state`;
  - `source_freshness`;
  - `presentation`.
- Presentation is derived by the shared core `presentation_state()`; relay does not maintain a second safety rule.
- Unknown regions return 404 rather than a false clear.
- A stale source can coexist with the last confirmed active state.
- A stale confirmed clear becomes `clear_needs_refresh`, not `cleared_current`.
- No POST endpoints, persistence, API keys, Internet fetches, push notifications, geolocation or production configuration.

## Dependency choice
- axum 0.8.9 pinned for the prototype.
- Tokio 1.53.x LTS line constrained with `~1.53`.
- serde 1.x for JSON output.

## Predeclared acceptance
1. Relay unit tests pass.
2. Root core tests remain green.
3. Relay binds only to loopback in source.
4. Unknown region never becomes clear.
5. Synthetic active + stale remains `confirmed_state=active`, `source_freshness=stale`, `presentation=active_last_known`.
6. Stale clear is `clear_needs_refresh`, not `cleared_current`.
7. Relay uses the shared core presentation model.
8. No third-party endpoint/token exists in code.
9. No release or deployment is produced.

## Next
After integration, replace hard-coded synthetic data only with an offline source-adapter interface + recorded fixtures. Live credentials stay outside GitHub and require a separate owner decision.
