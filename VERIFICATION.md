# VERIFICATION — ТривогА

**Оновлено:** 2026-11-01

## Integrated GitHub state
Current confirmed `main`: `1d7281c944ecfded2d70c7d7540bebdba06b0f15`.

Merged after clean reconciliation + fresh CI:
- #1 project foundation;
- #2 r1 alert-state core;
- #3 r2 multi-region store;
- #22 continuity checkpoint;
- #8 source freshness;
- #10 recovery snapshot;
- #11 presentation states.

## Verified safety behavior
- Source freshness: Unknown/Fresh/Stale/Unavailable separated from confirmed alert state.
- Loss/stale source does not clear an active alert.
- Recovery snapshot preserves confirmed state but restores freshness as Unknown.
- Inconsistent/duplicate recovery records fail closed.
- Stale/unavailable previous clear becomes ClearNeedsRefresh, not current clear.
- Unknown state never becomes a safe presentation.

## CI
Fresh Organizational docs checks + Rust core offline tests passed on the final clean heads for #8, #10 and #11 before merge.

## Sources
UkraineAlarm public access/offer, alerts.in.ua docs and KATOTTG source policy researched without credentials/live API calls.

## Drive
PROJECT mirror exists and was read back.
BACKUPS currently empty.
Full raw/git backup + restore: **NOT DONE**.

## Not completed
- Reconciliation/integration of #5/#7/#6/#4 and #12–#21.
- Android/iOS build validation.
- Live API credentials/schema/latency/webhook/security/legal acceptance.
- Public notification/background push.
- Full raw/git backup/restore.
- Full transitive license gate.
- Release artifacts/signing/installers.
- Formal clean-session continuity verification C13.

Issue #9 is the current integration/verification register.