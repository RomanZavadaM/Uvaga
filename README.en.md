# ТривогА / Uvaga

**Uvaga (ТривогА)** is a project for operational public information in Ukraine about official air-raid alerts and verified additional threat information.

> **Current status:** early prototype / synthetic-only. There is no production deployment, no live API credentials, and no public notification delivery. The application does not replace official alerting channels; absence of a message does not mean absence of danger.

**Primary README language: [Українська](README.md).** Translations: English · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

## Test download

### Windows 10/11 x64 — r20 NSIS installer candidate

[**Download Uvaga-Windows-Installer-r20.zip**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41447023597/artifacts/13142977634)

This is an **unsigned test installer, not a production release**. CI has already verified current-user silent install, HKCU registration, installed-app launch, and silent uninstall. Owner installer UX acceptance is still pending. r20 remains the latest archived installer test candidate with full automated install/uninstall evidence; r21–r23 were CI/release-process hardening and did not create a newer owner-accepted release artifact.

- Source head: `96a7953a2388811716c9c13001fcf9ca3d5b79c2`
- r20 merge: `e913edf6bb62249806afc7c19ac8c19e2cde1166`
- ZIP SHA256: `a8a00e0c7f20282d13673ba1fd3de353d4500d9cf36b1d40ed7be3c86ee33d3f`
- Installer SHA256: `dc8f128e73983d28c4093edd0434d4a8eed140a29718cd09faa99c13bf9f8588`
- Release `Uvaga.exe` SHA256: `636aa7d7ec6e5cf485dfda219d107115dba7d3dc79c0b5a995d4e08efc4aab30`
- Exact `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`
- NSIS install/launch/uninstall smoke: **PASS**
- Owner installer/visual sanity: **PENDING**

Drive backup: https://drive.google.com/file/d/1A0fAVnfmgnK6xfVwTlXB-muZxTnGrTuX/view?usp=drivesdk

### Windows 10/11 x64 — r19 synthetic test build

[**Download Uvaga-Windows-Synthetic-r19.zip**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41442359441/artifacts/13140138112)

This is **not a release and not a production build**. It is a historical r19 synthetic desktop-hardening build retained as verifiable evidence. r18 remains the last Windows build manually accepted by the owner; owner visual sanity for r19 is still pending. GitHub may require sign-in to download the Actions artifact.

- Source head: `cc4992975b73f0211cdd44b05f7ebd52d76a5415`
- ZIP SHA256: `7f2dc03cd58686e2555026a544d78673f9fdcf7b452c44913b40013ee34bffad`
- `Uvaga.exe` SHA256: `c9afe5873aad563dd6f07b51a6b05cc64fac3057e91f1127d2794b3372b7b593`
- `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`
- UI contract / Windows / macOS ARM / macOS Intel: **PASS**
- r19 integrated into `main`: `1af2e5d2c53a00619172056bab9fde746daff988`
- Owner visual sanity for r19: **PENDING**
- r18 owner manual runtime acceptance: **PASS — 2026-11-02**

r19 makes the critical state more compact and accessible, adds a fail-closed dependency-lock gate, and produces a self-describing test artifact. Synthetic territories and five fail-safe scenarios remain local; no live source is connected.

## Planned platforms

- Windows 10/11 x64
- macOS Apple Silicon
- macOS Intel
- Android
- iOS
- Web
- Telegram — later through an integration output

Windows 7 is not supported.

## Architecture direction

- Rust — core state, validation, and normalization logic.
- Tauri 2 — desktop/mobile application shell.
- A lightweight controlled Rust relay/cache — boundary for secret API tokens, rate limits, and a normalized public contract.
- PostgreSQL is not a baseline requirement. Operational state is short-lived; recovery storage is defined separately.
- GitHub is the source of truth; Google Drive is a one-way mirror of key documentation, not an alternative editor.

Details: [REQUIREMENTS.md](REQUIREMENTS.md) · [ARCHITECTURE.md](ARCHITECTURE.md) · [SOURCE_VALIDATION.md](SOURCE_VALIDATION.md).

## Safety invariants

- An official all-clear is accepted only as a newer confirmed official event.
- Timeout, TTL expiry, HTTP error, empty response, or source unavailability **must not create an all-clear**.
- Supplemental or aggregated data cannot independently replace an official source.
- An old confirmed clear without a fresh source is not presented as a current safe state.
- The project is not intended for public live tracking of military objects, exact coordinates, or trajectories.

## Development

The entry point for every new session is **[START_HERE.md](START_HERE.md)**.

Then read:
1. [PROJECT_RULES.md](PROJECT_RULES.md)
2. [PROJECT_STATE.md](PROJECT_STATE.md)
3. [WORKLOG.md](WORKLOG.md)
4. [HANDOFF_CURRENT.md](HANDOFF_CURRENT.md)
5. live GitHub PR/CI state

Verification and integration are tracked in [GitHub Issue #9](https://github.com/RomanZavadaM/Uvaga/issues/9).

## Methodology

The project is a **trial application of dev-strategy v2 RC**. C13 clean-session continuity is **PASS**. Project-local rules are self-contained for normal daily work; the central `dev-strategy` repository is not a runtime dependency of every session. Full owner acceptance of v2 RC (C14) has not yet been declared.

## License

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

Uvaga is **proprietary software**. Public repository visibility does not grant an open-source license. See [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md), and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
