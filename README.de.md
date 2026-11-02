# ТривогА / Uvaga

**Uvaga (ТривогА)** ist ein Projekt zur operativen Information der Bevölkerung in der Ukraine über offizielle Luftalarmmeldungen und verifizierte zusätzliche Gefahreninformationen.

> **Aktueller Status:** früher Prototyp / ausschließlich synthetisch. Es gibt kein Production-Deployment, keine Live-API-Zugangsdaten und keine öffentliche Benachrichtigungszustellung. Die Anwendung ersetzt keine offiziellen Warnkanäle; das Ausbleiben einer Meldung bedeutet nicht, dass keine Gefahr besteht.

**Primäre README-Sprache: [Українська](README.md).** Übersetzungen: [English](README.en.md) · [Français](README.fr.md) · Deutsch · [Español](README.es.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

## Test-Download

### Windows 10/11 x64 — r20 NSIS-Installer-Kandidat

[**Uvaga-Windows-Installer-r20.zip herunterladen**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41447023597/artifacts/13142977634)

Dies ist ein **unsignierter Test-Installer, kein Production-Release**. CI hat current-user Silent-Installation, HKCU-Registrierung, Start der installierten App und Silent-Deinstallation bereits geprüft. Die Installer-UX-Abnahme durch den Eigentümer steht noch aus.

- Source head: `96a7953a2388811716c9c13001fcf9ca3d5b79c2`
- r20-Merge: `e913edf6bb62249806afc7c19ac8c19e2cde1166`
- ZIP SHA256: `a8a00e0c7f20282d13673ba1fd3de353d4500d9cf36b1d40ed7be3c86ee33d3f`
- Installer SHA256: `dc8f128e73983d28c4093edd0434d4a8eed140a29718cd09faa99c13bf9f8588`
- Release-`Uvaga.exe` SHA256: `636aa7d7ec6e5cf485dfda219d107115dba7d3dc79c0b5a995d4e08efc4aab30`
- Exakte `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`
- NSIS Install/Start/Uninstall Smoke: **PASS**
- Eigentümer-Installer/Visual-Sanity: **PENDING**

Drive-Backup: https://drive.google.com/file/d/1A0fAVnfmgnK6xfVwTlXB-muZxTnGrTuX/view?usp=drivesdk

### Windows 10/11 x64 — r19 Synthetic-Test-Build

[**Uvaga-Windows-Synthetic-r19.zip herunterladen**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41442359441/artifacts/13140138112)

Dies ist **kein Release und kein Production-Build**. Es ist der neueste automatisiert geprüfte Desktop-Hardening-Build. r18 bleibt der letzte vom Eigentümer manuell akzeptierte Windows-Build; die visuelle Eigentümerprüfung für r19 steht noch aus.

- Source head: `cc4992975b73f0211cdd44b05f7ebd52d76a5415`
- ZIP SHA256: `7f2dc03cd58686e2555026a544d78673f9fdcf7b452c44913b40013ee34bffad`
- `Uvaga.exe` SHA256: `c9afe5873aad563dd6f07b51a6b05cc64fac3057e91f1127d2794b3372b7b593`
- `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`
- UI-Vertrag / Windows / macOS ARM / macOS Intel: **PASS**
- r19 in `main` integriert: `1af2e5d2c53a00619172056bab9fde746daff988`
- Visuelle Eigentümerprüfung r19: **PENDING**
- r18 manuelle Runtime-Abnahme: **PASS — 2026-11-02**

r19 macht den kritischen Status kompakter und zugänglicher, ergänzt einen fail-closed Dependency-Lock und erzeugt ein selbstdokumentierendes Test-Artefakt. Live-Quellen bleiben getrennt.

## Geplante Plattformen

- Windows 10/11 x64
- macOS Apple Silicon
- macOS Intel
- Android
- iOS
- Web
- Telegram — später über einen Integrationsausgang

Windows 7 wird nicht unterstützt.

## Architektur-Richtung

- Rust — Kernlogik für Zustand, Prüfung und Normalisierung.
- Tauri 2 — Desktop-/Mobile-Anwendungsshell.
- Leichtgewichtiger kontrollierter Rust-Relay/Cache — Grenze für geheime API-Tokens, Rate Limits und einen normalisierten öffentlichen Vertrag.
- PostgreSQL ist keine Grundanforderung. Operativer Zustand ist kurzlebig; Recovery-Speicher wird separat definiert.
- GitHub ist die Source of Truth; Google Drive ist ein einseitiger Spiegel wichtiger Dokumentation und kein alternativer Editor.

Details: [REQUIREMENTS.md](REQUIREMENTS.md) · [ARCHITECTURE.md](ARCHITECTURE.md) · [SOURCE_VALIDATION.md](SOURCE_VALIDATION.md).

## Sicherheitsinvarianten

- Eine offizielle Entwarnung wird nur als neueres bestätigtes offizielles Ereignis akzeptiert.
- Timeout, TTL-Ablauf, HTTP-Fehler, leere Antwort oder Quellenausfall **dürfen niemals eine Entwarnung erzeugen**.
- Zusätzliche oder aggregierte Daten dürfen eine offizielle Quelle nicht eigenständig ersetzen.
- Eine alte bestätigte Entwarnung ohne frische Quelle wird nicht als aktueller sicherer Zustand dargestellt.
- Das Projekt ist nicht für öffentliches Live-Tracking militärischer Objekte, exakter Koordinaten oder Flugbahnen vorgesehen.

## Entwicklung

Der Einstiegspunkt für jede neue Sitzung ist **[START_HERE.md](START_HERE.md)**.

Danach:
1. [PROJECT_RULES.md](PROJECT_RULES.md)
2. [PROJECT_STATE.md](PROJECT_STATE.md)
3. [WORKLOG.md](WORKLOG.md)
4. [HANDOFF_CURRENT.md](HANDOFF_CURRENT.md)
5. Live-Status von GitHub PR/CI

Verifikation und Integration werden in [GitHub Issue #9](https://github.com/RomanZavadaM/Uvaga/issues/9) geführt.

## Methodik

Das Projekt ist eine **Testanwendung von dev-strategy v2 RC**. C13 Clean-Session-Continuity ist **PASS**. Die lokalen Projektregeln sind für die tägliche Arbeit ausreichend; das zentrale `dev-strategy`-Repository ist keine Runtime-Abhängigkeit jeder Sitzung. Die vollständige Abnahme durch den Eigentümer (C14) wurde noch nicht erklärt.

## Lizenz

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

Uvaga ist **proprietäre Software**. Die öffentliche Sichtbarkeit des Repositorys gewährt keine Open-Source-Lizenz. Siehe [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md) und [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
