# Source freshness r7

## Problem
A boolean healthy/unhealthy flag cannot distinguish:
- no source result has ever been observed;
- a fresh source;
- a source whose last data is now stale;
- a source that is currently unavailable.

Those cases must be visible to clients without changing the last confirmed alert state.

## Model
SourceFreshness = Unknown | Fresh | Stale | Unavailable.

Compatibility helper set_source_health(bool) remains for early prototype callers:
- true -> Fresh
- false -> Unavailable

## Safety
Freshness never changes AlertState by itself. Active remains active until a newer verified official clear is applied. Stale/unavailable/unknown only affect confidence in current freshness.

## Predeclared acceptance
1. default freshness is Unknown;
2. Fresh is the only state where current_data_available() is true;
3. Stale and Unavailable are distinct;
4. source loss never clears Active;
5. all prior r1/r2 tests remain green.
