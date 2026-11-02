# ТривогА / Uvaga

**Uvaga (ТривогА)** es un proyecto para informar operativamente a la población de Ucrania sobre alertas aéreas oficiales e información adicional verificada sobre amenazas.

> **Estado actual:** prototipo temprano / solo sintético. No existe despliegue de producción, credenciales de API en vivo ni entrega pública de notificaciones. La aplicación no sustituye los canales oficiales de alerta; la ausencia de un mensaje no significa ausencia de peligro.

**Idioma principal del README: [Українська](README.md).** Traducciones: [English](README.en.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · Español · [한국어](README.ko.md) · [日本語](README.ja.md)

## Descarga de prueba

### Windows 10/11 x64 — build sintética r18

[**Descargar Uvaga-Windows-Synthetic-r18.zip**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41415006414/artifacts/13127022813)

Esto **no es una versión publicada ni una build de producción**. El archivo sirve únicamente para la verificación manual de los escenarios sintéticos de la PR #34. GitHub puede requerir iniciar sesión para descargar un artefacto de Actions.

- Source head: `a2c945034bfea58b3ff5890ea53862831c1b502a`
- SHA256 del ZIP: `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`
- SHA256 de `Uvaga.exe`: `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`
- Comprobaciones automáticas Windows/macOS: **PASS**
- Aceptación manual de runtime en Windows para r18: **pendiente**

La interfaz de prueba permite elegir un territorio sintético (`UA-46` o `UA-30`) y cinco escenarios fail-safe: active/fresh, active/stale, cleared/fresh, cleared/stale y unknown/unavailable.

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
