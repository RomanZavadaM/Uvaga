# ТривогА / Uvaga

**Uvaga (ТривогА)** is a project for operational public information in Ukraine about official air-raid alerts and verified additional threat information.

> **Current status:** early prototype / synthetic-only. There is no production deployment, no live API credentials, and no public notification delivery. The application does not replace official alerting channels; absence of a message does not mean absence of danger.

**Primary README language: [Українська](README.md).** Translations: English · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

## Test download

### Windows 10/11 x64 — r18 synthetic test build

[**Download Uvaga-Windows-Synthetic-r18.zip**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41415006414/artifacts/13127022813)

This is **not a release and not a production build**. It is the verified r18 synthetic test build; Windows manual acceptance is complete and the changes are integrated into `main`. GitHub may require you to sign in before downloading an Actions artifact.

- Source head: `a2c945034bfea58b3ff5890ea53862831c1b502a`
- ZIP SHA256: `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`
- `Uvaga.exe` SHA256: `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`
- Automated Windows/macOS checks: **PASS**
- Manual Windows runtime acceptance for r18: **PASS — 2026-11-02**
- r18 integrated into `main`: `61c0662d941e388858f5214c14860ec92a586905`

The test shell lets you choose a synthetic territory (`UA-46` or `UA-30`) and exercise five fail-safe scenarios: active/fresh, active/stale, cleared/fresh, cleared/stale, and unknown/unavailable.

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
