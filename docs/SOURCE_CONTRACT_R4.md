# Source contract r4 — offline normalization boundary

## Purpose
Define what a future source adapter is allowed to turn into an official alert event before any HTTP client, token or production feed exists.

## Rules
- Only SourceClass::Official plus trusted_adapter=true can normalize into OfficialEvent.
- Aggregator/supplemental sources cannot self-promote to official state.
- Advisories cannot become official start/clear signals.
- Blank provenance identifiers and impossible observation-time ordering are rejected.
- The normalized event ID is namespaced by source_id and includes provider record id + issued_at + signal kind.
- A provider may reuse one record id across lifecycle updates; start and clear of that record must normalize to distinct event-version IDs so dedup does not suppress a legitimate clear.
- No credentials, URLs, HTTP implementation, storage or notifications exist in this slice.

## Predeclared acceptance
1. Trusted official start normalizes.
2. Aggregator cannot create an official clear.
3. Untrusted adapter cannot create official state.
4. Advisory cannot normalize as official signal.
5. observed_at earlier than issued_at is rejected.
6. All prior r1/r2 tests remain green.

## Known limit
Clock skew policy is deliberately strict for now. Before live ingestion, define provider timestamp semantics and allowable skew instead of silently relaxing this invariant.
