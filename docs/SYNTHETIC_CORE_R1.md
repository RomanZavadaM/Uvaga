# Synthetic core r1 — plan and limits

Branch: feat/synthetic-alert-state-r1, PR intended against docs organizational branch until PR #1 lands.
This is a small Rust library with **zero external dependencies**, no executable service, no API requests, no UI and no notification system. It is intentionally non-operational.

## Predeclared acceptance
- Verified official start activates a region.
- Verified newer official clear clears it.
- Unverified / duplicate / older / equal-time conflicting events cannot overwrite the state.
- Feed loss does not clear a previously active alert; show unknown freshness separately.
- Supplementary advisory expiry does not clear official alert.
- Reject blank IDs and territories.
- Cargo tests and format check pass in CI before integration; failure or skipped checks mean NOT accepted.

## Known limitations
Model covers only one region per instance; no trust verification is performed inside this library (verified_official flag is a placeholder requiring a secure adapter boundary), no stable ordering beyond timestamp, no restart persistence or recovery, no real source integration. This is **not safe for public alerts**. Review design before expanding. Public messaging forbidden.
