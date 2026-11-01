# Attention ranking r15 — verified information on top

Purpose: deterministic display ordering **after** upstream source verification.

## Single source of truth for official state

Official attention priority is derived from the already integrated `PresentationState::attention_rank()`; this module does not maintain a second independent ordering for official alert/freshness combinations.

Current official order:
1. active last-known / source not current;
2. active current;
3. clear needs refresh;
4. official state unknown;
5. fresh confirmed clear.

A verified supplemental threat is placed between active states and `ClearNeedsRefresh`. Informational/service content remains below all official states.

## Safety rules
- Official active states always outrank supplemental information.
- Losing freshness while the last confirmed state is active does not reduce attention.
- Supplemental data cannot mutate or clear official state.
- A stale/unavailable previous clear remains above a fresh confirmed clear.
- There is deliberately no “unverified rumor” class. Admission/verification happens before ranking.
- Ranking controls display attention only; it cannot change official alert state.

## Determinism
Within one attention class:
1. newer `issued_at` first;
2. exact timestamp ties use lexical ID.

## Predeclared acceptance
1. official ordering exactly preserves shared `PresentationState::attention_rank()`;
2. both active official states outrank supplemental threat;
3. supplemental threat outranks clear-needs-refresh/current-clear but cannot alter them;
4. stale/unknown official state outranks current clear;
5. same-class items sort newest first;
6. exact ties are deterministic;
7. all previous core/source/territory/relay tests remain green.

No live source, notification transport, user profiling or production behavior is introduced.
