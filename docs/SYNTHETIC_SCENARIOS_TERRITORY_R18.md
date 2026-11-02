# Synthetic scenarios + territory selection r18

## Purpose

Extend the existing offline Tauri UI ↔ Rust core bridge so a tester can choose a small **synthetic territory fixture** and a **synthetic fail-safe scenario**.

This slice remains fully offline. It does not add a live source adapter, live API credentials, public notifications, geolocation, network access, production deployment, signing or release packaging.

## Predeclared acceptance

1. The Rust command accepts only explicitly whitelisted synthetic territory IDs and scenario IDs.
2. Unknown territory or scenario input is rejected; it must never be interpreted as a clear/safe state.
3. The UI offers at least two synthetic territory fixtures.
4. The UI can demonstrate:
   - active + fresh source → active current;
   - active + stale source → active last-known;
   - cleared + fresh source → cleared current;
   - cleared + stale source → clear needs refresh;
   - no confirmed event + unavailable source → unknown.
5. A stale/unavailable previous clear is never presented as current clear.
6. Territory/scenario selection is passed only through local Tauri IPC; no HTTP or external API is introduced.
7. IPC failure remains an explicit error/unknown condition, never a clear.
8. Rust tests cover every scenario plus rejection of unknown territory/scenario.
9. Windows synthetic artifact CI must compile/test the Tauri app and produce a checksumed unsigned test artifact.
10. Existing core safety tests remain green.

## Fixture scope

Territory labels in this slice are deliberate local test fixtures. They are not a complete current administrative directory of Ukraine and must not be presented as production KATOTTG data.

## Manual runtime acceptance

**PASS — 2026-11-02.**

Owner executed the verified Windows r18 artifact and supplied screenshots confirming:
- UA-46 and UA-30 territory selection;
- active + fresh → active current;
- active + stale → active last-known;
- cleared + fresh → cleared current;
- cleared + stale → clear requires refresh/reconfirmation;
- unknown + unavailable → unknown / insufficient data;
- local Tauri IPC success;
- explicit synthetic/not-real-alerting warning.

Critical acceptance criterion #5 is confirmed manually: **a stale previous clear is never presented as current clear**.

After acceptance, PR #34 was reconciled with current main. Fresh checks on reconciled product head `00044595f40b48b19e086e3e8a2a81cfaa48bfb5`:
- docs `41437451193`: PASS;
- Tauri Windows/macOS matrix `41437451223`: PASS;
- Windows synthetic artifact `41437451346`: PASS.

Final acceptance-checkpoint head `cb444fd31bd4f2e574944c7917df5452f154b12e` also passed:
- docs `41438228543`;
- Tauri Windows/macOS matrix `41438228579`;
- Windows synthetic artifact `41438228529`.

PR #34 merged squash to main at `61c0662d941e388858f5214c14860ec92a586905`.
