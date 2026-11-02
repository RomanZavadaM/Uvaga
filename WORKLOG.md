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
- KATOTTG policy requires Ministry base + amendment discovery to explicit cutoff; staging validator accepts current reviewed source shape fail-closed.
- UkraineAlarm request page/public offer audited; application/offer acceptance remains owner-only.
- Integrated deterministic attention ranking anchored to shared PresentationState.

## 2026-11-01 — desktop prototype
- Integrated minimal Tauri 2 shell, Windows 10/11 x64 + macOS ARM64/Intel CI.
- Direct pins: tauri 2.12.5; tauri-build 2.7.4.
- Cargo.lock checkpoint SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.
- CI optimized so pure root-core changes do not trigger full desktop matrix.
- r16 integrated: local UI ↔ Rust core bridge through Tauri IPC, no HTTP/live API/plugins.
- r16 Windows bridge unit test and both macOS compile targets PASS.
- r16 squash main: `bba0bd82351fa06f20dbe144e8ac487b81314e4f`.

## 2026-11-01 — r17 Windows synthetic artifact
- PR #28 created for build/test artifact only.
- Final source head: `2a9ef7e1d30c50a42dedcf822a2c8728a6d1d490`.
- Workflow run `41352681448`: PASS.
- `cargo test --locked`: PASS.
- `cargo build --locked`: PASS.
- Artifact downloaded/unpacked/read back; checksums verified.
- Raw ZIP mirrored to Drive BACKUPS.

## 2026-11-02 — r17 manual runtime acceptance
- Owner launched verified `Uvaga.exe` on Windows and supplied screenshots.
- Application opened successfully.
- Synthetic/not-real-alerting warning visible.
- Local Tauri IPC success visible.
- UI showed UA-46, active confirmed state, stale source, active-last-known presentation.
- No false clear displayed.
- Manual runtime acceptance: **PASS**.
- PR #28 merged squash: `0ea256b3cc1a72250a8fa06e3098d13fe780b770`.

## 2026-11-02 — C12 Git backup / restore
- Added Git bundle backup/restore workflow and acceptance doc.
- PR #31 pre-merge run: bundle verify + clean restore + fsck + restored Rust tests PASS.
- PR #31 merged squash: `81c3855112006d9350077b2a5b38f62e3701f73d`.
- Post-merge `main` run `41405252149`: PASS.
- GitHub artifact id `13123246036`.
- Artifact ZIP SHA256 `d236ae8b26179f44e64365b20f08484a65e1730633e7319386d2a14ecbe4de06`.
- Inner `Uvaga-all-refs.bundle` SHA256 `943ee4667b69871d8f851afb2695396a6a5a5ea6c820433c5f3083bcfdcf7a53`.
- Clean restore reproduced exact main SHA `81c3855112006d9350077b2a5b38f62e3701f73d`.
- Additional bundle download/readback/checksum/restore/fsck verification PASS.
- Raw backup copied to Drive BACKUPS.
- C12: **PASS for Git repository source/history**.
- Future production databases/secrets/user state remain outside this checkpoint.

## 2026-11-02 — C13 clean-session continuity
- Formal clean-session continuity trial completed from project-local GitHub docs + live GitHub only.
- Start main independently verified as `3a254e4a6edb3bcd2294cb0905ab7ddae4f1ca61`.
- Open PR count: 0.
- r17 Actions evidence re-read at job level: run `41352681448` PASS.
- C12 post-merge Actions evidence re-read at job/artifact level: run `41405252149` PASS; artifact `13123246036` present and not expired.
- Safety/permission limits recovered correctly.
- C13: **PASS**.
- Detailed record: `C13_CLEAN_SESSION.md`.

## 2026-11-02 — r18 synthetic scenarios / territory selection
- Predeclared acceptance created before implementation.
- PR #34 opened: `feat(r18): add synthetic territory and fail-safe scenarios`.
- Verified product source head: `a2c945034bfea58b3ff5890ea53862831c1b502a`.
- Added two whitelisted synthetic territories: `UA-46`, `UA-30`.
- Added five fail-safe scenarios: active/fresh, active/stale, cleared/fresh, cleared/stale, unknown/unavailable.
- Unknown territory/scenario rejected; local Tauri IPC only; IPC failure never creates clear.
- Automated CI/artifact verification: PASS.
- Verified artifact id `13127022813`; ZIP SHA256 `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`; `Uvaga.exe` SHA256 `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`.
- Subsequent PR-head changes through `cccce86708e77e6a1cd670179ebf9547d0a5f671` are README-only; comparison confirms no r18 product files changed after the verified artifact build.
- Latest PR-head workflows also PASS: docs `41417199329`, Tauri matrix `41417199299`, Windows artifact `41417199372`.
- Manual Windows runtime acceptance: **PENDING**. PR #34 must remain open until owner acceptance.

## 2026-11-02 — README localization
- PR #35 merged to main: `e3d11e7ac47ee72a9d22ab5205115dac1d8a8956`.
- Ukrainian README remains primary/canonical.
- Added English, French, German, Spanish, Korean and Japanese README variants.
- Every README includes the verified r18 synthetic Windows download link, hashes and non-release/non-production warning.
- Organizational docs/link checks: PASS.

## Execution continuity
- One active executor by default; parallel work only by owner.
- State divergence is recovered from live GitHub rather than assumed parallel work.
- Issue #9 is the live integration register.

## NEXT
Owner manual Windows 10/11 x64 runtime acceptance of r18 artifact `13127022813`. Do not merge PR #34 or start a live source adapter before PASS.
