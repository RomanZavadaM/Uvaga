# Synthetic core r2 — multi-region store

Branch: feat/synthetic-region-store-r2. This stage is stacked on r1 and remains fully offline.

## Added
- In-memory registry keyed by region code.
- Per-region recent event-id deduplication with a fixed cap of 64 IDs.
- Explicit apply result: applied / duplicate / rejected.
- Source-health flag remains separate from confirmed alert state.
- No external dependencies, persistence, networking, UI, notification delivery or geolocation.

## Predeclared acceptance
1. Regions do not overwrite one another.
2. Replayed event_id is rejected within its region even if it arrives with a newer timestamp.
3. The same provider event ID may exist independently in different regions.
4. Source failure does not clear a confirmed active alert.
5. An initially unverified event may later be accepted when delivered by an admitted official adapter; rejected input must not poison deduplication.
6. Existing r1 tests must remain green.

## Limits
The 64-ID cache is a bounded replay guard, not a permanent event archive. Restart persistence, provider-specific ID semantics, oblast/district/community hierarchy, event conflict policy, real API authentication and source trust are not implemented. This stage is not safe for public alerts.
