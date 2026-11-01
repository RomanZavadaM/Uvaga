# Recovery snapshot r8 — conservative restart state

## Goal
Preserve only the minimum confirmed alert state needed to survive a process restart without inventing a clear.

## Model
RecoverySnapshot v1 stores per region:
- region id;
- confirmed AlertState;
- last_event_at;
- last_event_id.

Source freshness is deliberately NOT persisted. Every restored region starts with SourceFreshness::Unknown until a source is contacted again. This prevents a restart from falsely presenting old data as fresh.

The last event id is restored into the bounded dedup guard so an immediate replay of the last provider event is ignored.

## Validation
Restore fails closed on:
- unsupported snapshot version;
- blank region;
- duplicate region;
- last_event_at without last_event_id or vice versa;
- Active/Cleared without a last event, or Unknown with a persisted last event;
- blank persisted last_event_id.

## Predeclared acceptance
1. Active state survives restore.
2. Fresh source becomes Unknown after restore.
3. Last event remains deduplicated after restore.
4. Inconsistent, state/event-contradictory and duplicate records are rejected.
5. Unsupported versions are rejected.
6. Existing r1/r2/r7 tests remain green.

## Deliberate limits
This slice defines an in-memory versioned snapshot structure only. It does not choose disk format, SQLite, encryption, atomic file replacement, retention, or production recovery. Those require a separate slice and crash/restore tests. No network or notifications.
