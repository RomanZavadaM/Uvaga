# ТривогА / Uvaga

**Uvaga (ТривогА)** ist ein Projekt zur operativen Information der Bevölkerung in der Ukraine über offizielle Luftalarmmeldungen und verifizierte zusätzliche Gefahreninformationen.

> **Aktueller Status:** früher Prototyp / ausschließlich synthetisch. Es gibt kein Production-Deployment, keine Live-API-Zugangsdaten und keine öffentliche Benachrichtigungszustellung. Die Anwendung ersetzt keine offiziellen Warnkanäle; das Ausbleiben einer Meldung bedeutet nicht, dass keine Gefahr besteht.

**Primäre README-Sprache: [Українська](README.md).** Übersetzungen: [English](README.en.md) · [Français](README.fr.md) · Deutsch · [Español](README.es.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

## Test-Download

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
