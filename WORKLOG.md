# WORKLOG — ТривогА

## 2026-11-01 — foundation and offline core
- Created public proprietary `RomanZavadaM/Uvaga`.
- Established project-local continuity, safety rules, source/backup policies and dev-strategy v2 RC trial.
- Integrated offline Rust alert state, multi-region store, source freshness, conservative recovery and fail-safe presentation.
- Integrated source trust, provider event-version identity, alerts.in.ua offline parser and localhost synthetic relay.
- No live credentials, production deployment or public notifications used.

## 2026-11-01 — territory/source model
- Integrated source-neutral `Country → Region → Raion → Hromada → Locality` hierarchy.
- Integrated provider→core mapping and fail-closed hierarchical subscription matching.
- Integrated mapped official source normalization.
- KATOTTG policy now requires Ministry base + amendment discovery to explicit cutoff; staging validator accepts the current reviewed source shape fail-closed.
- UkraineAlarm request page/public offer audited; application/offer acceptance remains owner-only.
- Integrated deterministic attention ranking anchored to shared PresentationState.

## 2026-11-01 — desktop prototype
- Integrated minimal Tauri 2 shell, Windows 10/11 x64 + macOS ARM64/Intel CI.
- Direct pins: tauri 2.12.5; tauri-build 2.7.4.
- Clean Cargo.lock checkpoint SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.
- CI optimized so pure root-core changes do not trigger full desktop matrix.
- r16 integrated: local UI ↔ Rust core bridge through Tauri IPC, no HTTP/live API/plugins.
- r16 Windows bridge unit test and both macOS compile targets PASS.
- r16 squash main: `bba0bd82351fa06f20dbe144e8ac487b81314e4f`.

## 2026-11-01 — r17 Windows synthetic artifact
- Draft PR #28 created for build/test artifact only.
- Corrected executable name to `Uvaga.exe`.
- Corrected artifact metadata to distinguish PR head from Actions merge-ref SHA.
- Final source head: `2a9ef7e1d30c50a42dedcf822a2c8728a6d1d490`.
- Workflow run `41352681448`: PASS.
- `cargo test --locked`: PASS.
- `cargo build --locked`: PASS.
- Final artifact downloaded/unpacked/read back; SHA256SUMS verified OK.
- `Uvaga.exe`: `220a15adc02ab03ff2f067f01a3f67a31b97fc88c01eb46a0633d4ee9022acd8`.
- `Cargo.lock`: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.
- ZIP: `c02fab2a009e85ea3b9218f800e00c8e53be24f6e8b5789f3e188344bf281c98`.
- Raw ZIP mirrored to Drive BACKUPS; verification document added to VERIFICATION.
- Runtime acceptance remains pending; PR #28 stays draft.

## Execution continuity
- One active executor by default; parallel work only by owner.
- A real state-divergence incident was recovered by reading live GitHub rather than assuming another executor.
- Issue #9 is the live integration register.

## NEXT
Owner manual Windows test of r17 artifact. PASS → merge #28 and proceed to Git bundle + restore checkpoint. FAIL → fix and rebuild before merge.
