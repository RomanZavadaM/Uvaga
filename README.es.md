# ТривогА / Uvaga

**Uvaga (ТривогА)** es un proyecto para informar operativamente a la población de Ucrania sobre alertas aéreas oficiales e información adicional verificada sobre amenazas.

> **Estado actual:** prototipo temprano / solo sintético. No existe despliegue de producción, credenciales de API en vivo ni entrega pública de notificaciones. La aplicación no sustituye los canales oficiales de alerta; la ausencia de un mensaje no significa ausencia de peligro.

**Idioma principal del README: [Українська](README.md).** Traducciones: [English](README.en.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · Español · [한국어](README.ko.md) · [日本語](README.ja.md)

## Descarga de prueba

### Windows 10/11 x64 — build sintética r19

[**Descargar Uvaga-Windows-Synthetic-r19.zip**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41442359441/artifacts/13140138112)

Esto **no es una versión publicada ni una build de producción**. Es la última build de endurecimiento desktop verificada automáticamente. r18 sigue siendo la última build de Windows aceptada manualmente por el propietario; la revisión visual del propietario para r19 sigue pendiente.

- Source head: `cc4992975b73f0211cdd44b05f7ebd52d76a5415`
- SHA256 del ZIP: `7f2dc03cd58686e2555026a544d78673f9fdcf7b452c44913b40013ee34bffad`
- SHA256 de `Uvaga.exe`: `c9afe5873aad563dd6f07b51a6b05cc64fac3057e91f1127d2794b3372b7b593`
- SHA256 de `Cargo.lock`: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`
- Contrato UI / Windows / macOS ARM / macOS Intel: **PASS**
- r19 integrado en `main`: `1af2e5d2c53a00619172056bab9fde746daff988`
- Revisión visual del propietario r19: **PENDING**
- Aceptación manual runtime r18: **PASS — 2026-11-02**

r19 hace el estado crítico más compacto y accesible, añade un bloqueo fail-closed de dependencias y un artefacto de prueba autodescriptivo. Las fuentes en vivo siguen sin conectarse.

## Plataformas previstas

- Windows 10/11 x64
- macOS Apple Silicon
- macOS Intel
- Android
- iOS
- Web
- Telegram — más adelante mediante una salida de integración

Windows 7 no es compatible.

## Dirección de arquitectura

- Rust — lógica central de estado, validación y normalización.
- Tauri 2 — shell de aplicación desktop/mobile.
- Relay/cache ligero y controlado en Rust — límite para tokens API secretos, rate limits y un contrato público normalizado.
- PostgreSQL no es un requisito básico. El estado operativo es de corta duración; el almacenamiento de recuperación se define por separado.
- GitHub es la fuente de verdad; Google Drive es un espejo unidireccional de documentación clave, no un editor alternativo.

Detalles: [REQUIREMENTS.md](REQUIREMENTS.md) · [ARCHITECTURE.md](ARCHITECTURE.md) · [SOURCE_VALIDATION.md](SOURCE_VALIDATION.md).

## Invariantes de seguridad

- Una finalización oficial de alerta solo se acepta como un evento oficial confirmado y más reciente.
- Timeout, expiración TTL, error HTTP, respuesta vacía o indisponibilidad de la fuente **no deben crear una finalización de alerta**.
- Una fuente adicional o agregada no puede sustituir por sí sola a una fuente oficial.
- Una finalización antigua confirmada sin una fuente fresca no se muestra como un estado seguro actual.
- El proyecto no está destinado al seguimiento público en vivo de objetos militares, coordenadas exactas o trayectorias.

## Desarrollo

El punto de entrada para cada nueva sesión es **[START_HERE.md](START_HERE.md)**.

Después:
1. [PROJECT_RULES.md](PROJECT_RULES.md)
2. [PROJECT_STATE.md](PROJECT_STATE.md)
3. [WORKLOG.md](WORKLOG.md)
4. [HANDOFF_CURRENT.md](HANDOFF_CURRENT.md)
5. estado live de GitHub PR/CI

La verificación e integración se registran en [GitHub Issue #9](https://github.com/RomanZavadaM/Uvaga/issues/9).

## Metodología

El proyecto es una **aplicación de prueba de dev-strategy v2 RC**. La continuidad clean-session C13 está en **PASS**. Las reglas locales del proyecto son autosuficientes para el trabajo diario; el repositorio central `dev-strategy` no es una dependencia runtime de cada sesión. La aceptación completa por parte del propietario (C14) aún no ha sido declarada.

## Licencia

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

Uvaga es **software propietario**. La visibilidad pública del repositorio no concede una licencia open source. Consulte [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md) y [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
