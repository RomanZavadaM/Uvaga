# ТривогА / Uvaga

**Uvaga (ТривогА)** est un projet d’information opérationnelle de la population en Ukraine sur les alertes aériennes officielles et sur des informations complémentaires vérifiées concernant les menaces.

> **État actuel :** prototype précoce / uniquement synthétique. Il n’existe aucun déploiement production, aucun identifiant API live et aucune diffusion publique de notifications. L’application ne remplace pas les canaux officiels d’alerte ; l’absence de message ne signifie pas l’absence de danger.

**Langue principale du README : [Українська](README.md).** Traductions : [English](README.en.md) · Français · [Deutsch](README.de.md) · [Español](README.es.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

## Téléchargement de test

### Windows 10/11 x64 — build synthétique r18

[**Télécharger Uvaga-Windows-Synthetic-r18.zip**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41415006414/artifacts/13127022813)

Ceci n’est **ni une version publiée ni une build de production**. Il s’agit de la build synthétique r18 vérifiée ; la validation manuelle Windows est terminée et les changements sont intégrés dans `main`. GitHub peut demander une connexion pour télécharger un artifact Actions.

- Source head : `a2c945034bfea58b3ff5890ea53862831c1b502a`
- SHA256 ZIP : `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`
- SHA256 `Uvaga.exe` : `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`
- Vérifications automatisées Windows/macOS : **PASS**
- Validation manuelle Windows r18 : **PASS — 2026-11-02**
- r18 intégré dans `main` : `61c0662d941e388858f5214c14860ec92a586905`

L’interface de test permet de choisir un territoire synthétique (`UA-46` ou `UA-30`) et cinq scénarios fail-safe : active/fresh, active/stale, cleared/fresh, cleared/stale et unknown/unavailable.

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
