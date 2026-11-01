# Synthetic core r1 — plan and limits

PR #2 is the first code slice on top of the accepted organizational `main`.

This is a small Rust library with **zero external dependencies**, no executable service, no API requests, no UI and no notification system. It is intentionally non-operational.

## Predeclared acceptance
- Verified official start activates a region.
- Verified newer official clear clears it.
- Unverified / duplicate / older / equal-time conflicting events cannot overwrite the state.
- Feed loss does not clear a previously active alert; source availability is separate from confirmed alert state.
- Supplementary advisory expiry does not clear official alert.
- Reject blank IDs and territories.
- Current-head Rust CI must pass before integration.

## Event identity
`OfficialEvent.event_id` is an immutable normalized event-version identity. A mutable provider record id must be versioned by a source adapter (for example with timestamp + signal kind) before entering core deduplication.

## Known limitations
The model covers one region per instance; `verified_official` is only a placeholder for a later trusted adapter boundary. There is no persistence, source adapter, network, UI, notification delivery or production integration. This slice is **not safe for public alerts**.
