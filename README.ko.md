# ТривогА / Uvaga

**Uvaga (ТривогА)**는 우크라이나 주민에게 공식 공습 경보와 검증된 추가 위협 정보를 신속하게 제공하기 위한 프로젝트입니다.

> **현재 상태:** 초기 프로토타입 / synthetic-only. production 배포, live API 자격 증명, 공개 알림 전송은 아직 없습니다. 이 애플리케이션은 공식 경보 채널을 대체하지 않으며, 메시지가 없다고 해서 위험이 없다는 뜻은 아닙니다.

**README 기본 언어: [Українська](README.md).** 번역: [English](README.en.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · 한국어 · [日本語](README.ja.md)

## 테스트 다운로드

### Windows 10/11 x64 — r20 NSIS installer candidate

[**Uvaga-Windows-Installer-r20.zip 다운로드**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41447023597/artifacts/13142977634)

이 파일은 **서명되지 않은 테스트 설치 프로그램이며 production release가 아닙니다**. CI에서 current-user silent install, HKCU 등록, 설치된 앱 실행, silent uninstall을 이미 검증했습니다. 소유자의 installer UX/visual sanity는 아직 대기 중입니다. r20은 전체 자동 install/uninstall evidence가 보존된 최신 archived installer test candidate이며, r21–r23은 CI/release-process hardening으로 더 새로운 owner-accepted release artifact를 만들지 않았습니다.

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

[**Uvaga-Windows-Synthetic-r19.zip 다운로드**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41442359441/artifacts/13140138112)

이 파일은 **정식 릴리스도 production 빌드도 아닙니다**. 검증 가능한 evidence로 보존된 역사적 r19 synthetic desktop-hardening build입니다. r18은 소유자가 Windows에서 수동 승인한 마지막 빌드이며 r19의 owner visual sanity는 아직 대기 중입니다.

- Source head: `cc4992975b73f0211cdd44b05f7ebd52d76a5415`
- ZIP SHA256: `7f2dc03cd58686e2555026a544d78673f9fdcf7b452c44913b40013ee34bffad`
- `Uvaga.exe` SHA256: `c9afe5873aad563dd6f07b51a6b05cc64fac3057e91f1127d2794b3372b7b593`
- `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`
- UI contract / Windows / macOS ARM / macOS Intel: **PASS**
- r19 `main` 통합: `1af2e5d2c53a00619172056bab9fde746daff988`
- r19 owner visual sanity: **PENDING**
- r18 owner manual runtime 승인: **PASS — 2026-11-02**

r19은 핵심 상태를 더 작고 접근 가능하게 표시하며 fail-closed dependency lock과 자체 설명형 테스트 artifact를 추가합니다. live source는 연결하지 않습니다.

## 계획된 플랫폼

- Windows 10/11 x64
- macOS Apple Silicon
- macOS Intel
- Android
- iOS
- Web
- Telegram — 추후 통합 출력 방식으로 지원 예정

Windows 7은 지원하지 않습니다.

## 아키텍처 방향

- Rust — 상태, 검증, 정규화 핵심 로직.
- Tauri 2 — desktop/mobile 애플리케이션 shell.
- 경량의 통제된 Rust relay/cache — 비밀 API token, rate limit, 정규화된 public contract의 경계.
- PostgreSQL은 기본 요구 사항이 아닙니다. 운영 상태는 단기 상태이며 recovery storage는 별도로 정의합니다.
- GitHub가 source of truth이며 Google Drive는 핵심 문서의 단방향 mirror일 뿐 대체 편집기가 아닙니다.

자세한 내용: [REQUIREMENTS.md](REQUIREMENTS.md) · [ARCHITECTURE.md](ARCHITECTURE.md) · [SOURCE_VALIDATION.md](SOURCE_VALIDATION.md).

## 안전 불변조건

- 공식 해제는 더 최신의 확인된 공식 이벤트로만 받아들입니다.
- timeout, TTL 만료, HTTP 오류, 빈 응답, 소스 중단은 **절대로 해제를 생성하지 않습니다**.
- 보조 또는 집계 데이터는 공식 소스를 독립적으로 대체할 수 없습니다.
- 최신 소스가 없는 과거의 확인된 해제는 현재 안전 상태로 표시하지 않습니다.
- 이 프로젝트는 군사 객체, 정확한 좌표 또는 궤적의 공개 실시간 추적을 목적으로 하지 않습니다.

## 개발

새 세션의 시작점은 **[START_HERE.md](START_HERE.md)** 입니다.

그다음:
1. [PROJECT_RULES.md](PROJECT_RULES.md)
2. [PROJECT_STATE.md](PROJECT_STATE.md)
3. [WORKLOG.md](WORKLOG.md)
4. [HANDOFF_CURRENT.md](HANDOFF_CURRENT.md)
5. live GitHub PR/CI 상태

검증과 통합은 [GitHub Issue #9](https://github.com/RomanZavadaM/Uvaga/issues/9)에서 관리합니다.

## 방법론

이 프로젝트는 **dev-strategy v2 RC의 시험 적용**입니다. C13 clean-session continuity는 **PASS**입니다. 프로젝트 로컬 규칙만으로 일상 작업을 이어갈 수 있으며 중앙 `dev-strategy` 저장소는 매 세션의 runtime 의존성이 아닙니다. 소유자의 전체 승인(C14)은 아직 선언되지 않았습니다.

## 라이선스

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

Uvaga는 **proprietary software**입니다. 공개 저장소라고 해서 open-source 라이선스가 부여되는 것은 아닙니다. [LICENSE.md](LICENSE.md), [COPYRIGHT.md](COPYRIGHT.md), [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)를 참조하십시오.
