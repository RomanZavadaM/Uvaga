# VERIFICATION — ТривогА

**Оновлено:** 2026-11-01

## Integrated product state
Last integrated product-code base: `bba0bd82351fa06f20dbe144e8ac487b81314e4f` (r16).
Live `main` must always be checked in GitHub.

Integrated prototype verifies:
- official alert state separated from source freshness;
- source loss cannot create clear;
- recovery resets freshness conservatively;
- stale clear is not current clear;
- trusted-source admission and immutable event-version identity;
- provider territory mapping before core identity;
- hierarchical subscription overlap;
- deterministic attention ranking;
- local Tauri UI ↔ Rust core IPC without HTTP/live source.

## Desktop CI
r16 PR #27:
- Windows bridge unit test: PASS;
- macOS ARM64 compile: PASS;
- macOS Intel compile: PASS;
- organizational docs check: PASS.

## r17 build verification
PR #28 source head:
`2a9ef7e1d30c50a42dedcf822a2c8728a6d1d490`

Final workflow run:
`41352681448` — PASS.

GitHub artifact:
- id `13102550016`;
- digest `sha256:c02fab2a009e85ea3b9218f800e00c8e53be24f6e8b5789f3e188344bf281c98`.

Downloaded ZIP was independently unpacked/read back:
- `Uvaga.exe` — 12,201,472 bytes;
  SHA256 `220a15adc02ab03ff2f067f01a3f67a31b97fc88c01eb46a0633d4ee9022acd8`;
- `Cargo.lock` — 104,041 bytes;
  SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- `SHA256SUMS.txt`: both entries PASS;
- `TEST_ARTIFACT.txt`: correct source head + distinct Actions checkout SHA;
- downloaded ZIP SHA256 equals GitHub artifact digest: PASS.

Drive raw backup:
`Uvaga-Windows-Synthetic-r17-2a9ef7e1-20261101.zip`
in `ТривогА/BACKUPS`.

**Manual Windows runtime acceptance: NOT YET DONE.**
Therefore r17 is not accepted/merged yet.

## Source/legal checks
- UkraineAlarm public request page/offer checked without accepting/applying.
- KATOTTG uses cutoff/amendment-chain policy.
- alerts.in.ua remains aggregator/proxy candidate only.
- no live API credential used.

## Backup status
- project continuity mirror: exists;
- raw Tauri lock artifact: preserved;
- raw r17 Windows artifact: preserved and checksum-verified;
- full Git history bundle + clean restore: **NOT DONE**.

## Still open
- r17 manual Windows runtime acceptance;
- full Git bundle/restore (C12);
- full transitive dependency/license gate before release;
- Android/iOS validation;
- live API access/security/latency/schema acceptance;
- push/background delivery;
- signing/installers/release/production.
