# Presentation state r9 — fail-safe UI semantics

## Purpose
Convert the confirmed alert state + source freshness into a UI-safe state without letting stale data look safe.

## States
- ActiveCurrent — active and source is fresh.
- ActiveLastKnown — last confirmed state is active, but current source freshness is unknown/stale/unavailable.
- ClearNeedsRefresh — last confirmed state was clear, but source is not currently fresh.
- Unknown — no confirmed official state exists.
- ClearedCurrent — confirmed clear and source is currently fresh.

Only ClearedCurrent may be rendered with a positive/safe visual treatment. This does NOT mean the product guarantees safety; it means only that the latest accepted official state is clear and the source is currently fresh.

## Ordering
Attention-oriented lists rank:
1. ActiveLastKnown
2. ActiveCurrent
3. ClearNeedsRefresh
4. Unknown
5. ClearedCurrent

The ranking is not a military threat prediction. It only prevents uncertain/stale states from being buried below a fresh clear.

## Predeclared acceptance
1. active + source loss stays non-safe;
2. stale/unavailable clear becomes ClearNeedsRefresh, never ClearedCurrent;
3. only fresh confirmed clear may show as safe;
4. unknown is never safe;
5. uncertain states rank above fresh clear;
6. all prior core tests remain green.
