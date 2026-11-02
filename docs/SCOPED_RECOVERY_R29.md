# r29 — acceptance: safe scoped state recovery

Status: implementation candidate; no live sources, keys, network or release.

- Versioned snapshot holds only historical confirmed states keyed by exact territory × threat category.
- Source freshness is deliberately excluded. Every restored clear must require fresh re-confirmation.
- No parent/child implied clear; categories remain independent.
- Reject unsupported snapshot version, uncatalogued territory, duplicate scope and invalid/no-history entries.
- Old/equal-time events are rejected after restore.
- All exact-head core and desktop CI must pass before merge.
- Cross-platform source/transport plan remains a separate design stage: common core + approved adapters and redundant IP transports; no fabricated latency/ETA for moving military threats.
