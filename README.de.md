# ТривогА / Uvaga

**Uvaga (ТривогА)** ist ein Projekt zur operativen Information der Bevölkerung in der Ukraine über offizielle Luftalarmmeldungen und verifizierte zusätzliche Gefahreninformationen.

> **Aktueller Status:** früher Prototyp / ausschließlich synthetisch. Es gibt kein Production-Deployment, keine Live-API-Zugangsdaten und keine öffentliche Benachrichtigungszustellung. Die Anwendung ersetzt keine offiziellen Warnkanäle; das Ausbleiben einer Meldung bedeutet nicht, dass keine Gefahr besteht.

**Primäre README-Sprache: [Українська](README.md).** Übersetzungen: [English](README.en.md) · [Français](README.fr.md) · Deutsch · [Español](README.es.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

## Test-Download

### Windows 10/11 x64 — r18 Synthetic-Test-Build

[**Uvaga-Windows-Synthetic-r18.zip herunterladen**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41415006414/artifacts/13127022813)

Dies ist **kein Release und kein Production-Build**. Es handelt sich um den verifizierten r18-Synthetic-Test-Build; die manuelle Windows-Abnahme ist abgeschlossen und die Änderungen sind in `main` integriert. Für den Download eines GitHub-Actions-Artefakts kann eine Anmeldung bei GitHub erforderlich sein.

- Source head: `a2c945034bfea58b3ff5890ea53862831c1b502a`
- ZIP SHA256: `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`
- `Uvaga.exe` SHA256: `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`
- Automatisierte Windows/macOS-Prüfungen: **PASS**
- Manuelle Windows-Runtime-Abnahme für r18: **PASS — 2026-11-02**
- r18 in `main` integriert: `61c0662d941e388858f5214c14860ec92a586905`

Die Testoberfläche erlaubt die Auswahl eines synthetischen Gebiets (`UA-46` oder `UA-30`) und fünf Fail-safe-Szenarien: active/fresh, active/stale, cleared/fresh, cleared/stale und unknown/unavailable.

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
