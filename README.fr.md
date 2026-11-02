# ТривогА / Uvaga

**Uvaga (ТривогА)** est un projet d’information opérationnelle de la population en Ukraine sur les alertes aériennes officielles et sur des informations complémentaires vérifiées concernant les menaces.

> **État actuel :** prototype précoce / uniquement synthétique. Il n’existe aucun déploiement production, aucun identifiant API live et aucune diffusion publique de notifications. L’application ne remplace pas les canaux officiels d’alerte ; l’absence de message ne signifie pas l’absence de danger.

**Langue principale du README : [Українська](README.md).** Traductions : [English](README.en.md) · Français · [Deutsch](README.de.md) · [Español](README.es.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

## Téléchargement de test

### Windows 10/11 x64 — build synthétique r19

[**Télécharger Uvaga-Windows-Synthetic-r19.zip**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41442359441/artifacts/13140138112)

Ceci n’est **ni une version publiée ni une build de production**. C’est la dernière build de durcissement desktop validée automatiquement. r18 reste la dernière build Windows acceptée manuellement par le propriétaire ; le contrôle visuel propriétaire de r19 est encore en attente.

- Source head : `cc4992975b73f0211cdd44b05f7ebd52d76a5415`
- SHA256 ZIP : `7f2dc03cd58686e2555026a544d78673f9fdcf7b452c44913b40013ee34bffad`
- SHA256 `Uvaga.exe` : `c9afe5873aad563dd6f07b51a6b05cc64fac3057e91f1127d2794b3372b7b593`
- SHA256 `Cargo.lock` : `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`
- Contrat UI / Windows / macOS ARM / macOS Intel : **PASS**
- r19 intégré dans `main` : `1af2e5d2c53a00619172056bab9fde746daff988`
- Contrôle visuel propriétaire r19 : **PENDING**
- Acceptation manuelle runtime r18 : **PASS — 2026-11-02**

r19 compacte et rend plus accessible l’état critique, ajoute un verrou de dépendances fail-closed et un artifact de test auto-documenté. Les territoires et scénarios restent synthétiques et locaux.

## Plateformes prévues

- Windows 10/11 x64
- macOS Apple Silicon
- macOS Intel
- Android
- iOS
- Web
- Telegram — plus tard via une sortie d’intégration

Windows 7 n’est pas pris en charge.

## Orientation architecturale

- Rust — logique centrale d’état, de validation et de normalisation.
- Tauri 2 — shell d’application desktop/mobile.
- Relay/cache Rust léger et contrôlé — frontière pour les tokens API secrets, les limites de débit et un contrat public normalisé.
- PostgreSQL n’est pas une exigence de base. L’état opérationnel est de courte durée ; le stockage de récupération est défini séparément.
- GitHub est la source de vérité ; Google Drive est un miroir unidirectionnel de la documentation clé, pas un éditeur alternatif.

Détails : [REQUIREMENTS.md](REQUIREMENTS.md) · [ARCHITECTURE.md](ARCHITECTURE.md) · [SOURCE_VALIDATION.md](SOURCE_VALIDATION.md).

## Invariants de sécurité

- Une fin d’alerte officielle n’est acceptée que comme un événement officiel confirmé et plus récent.
- Timeout, TTL expiré, erreur HTTP, réponse vide ou indisponibilité de la source **ne doivent jamais créer une fin d’alerte**.
- Une source supplémentaire ou agrégée ne peut pas remplacer seule une source officielle.
- Une ancienne fin d’alerte confirmée sans source fraîche n’est pas affichée comme un état sûr actuel.
- Le projet n’est pas destiné au suivi public en direct d’objets militaires, de coordonnées précises ou de trajectoires.

## Développement

Le point d’entrée de toute nouvelle session est **[START_HERE.md](START_HERE.md)**.

Puis lire :
1. [PROJECT_RULES.md](PROJECT_RULES.md)
2. [PROJECT_STATE.md](PROJECT_STATE.md)
3. [WORKLOG.md](WORKLOG.md)
4. [HANDOFF_CURRENT.md](HANDOFF_CURRENT.md)
5. l’état live des PR/CI GitHub

La vérification et l’intégration sont suivies dans [GitHub Issue #9](https://github.com/RomanZavadaM/Uvaga/issues/9).

## Méthodologie

Le projet est une **application expérimentale de dev-strategy v2 RC**. La continuité clean-session C13 est **PASS**. Les règles locales du projet sont suffisantes pour le travail quotidien ; le dépôt central `dev-strategy` n’est pas une dépendance runtime de chaque session. L’acceptation complète par le propriétaire (C14) n’a pas encore été déclarée.

## Licence

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

Uvaga est un **logiciel propriétaire**. La visibilité publique du dépôt n’accorde aucune licence open source. Voir [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md) et [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
