# ТривогА / Uvaga

**Uvaga (ТривогА)**는 우크라이나 주민에게 공식 공습 경보와 검증된 추가 위협 정보를 신속하게 제공하기 위한 프로젝트입니다.

> **현재 상태:** 초기 프로토타입 / synthetic-only. production 배포, live API 자격 증명, 공개 알림 전송은 아직 없습니다. 이 애플리케이션은 공식 경보 채널을 대체하지 않으며, 메시지가 없다고 해서 위험이 없다는 뜻은 아닙니다.

**README 기본 언어: [Українська](README.md).** 번역: [English](README.en.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · 한국어 · [日本語](README.ja.md)

## 테스트 다운로드

### Windows 10/11 x64 — r18 synthetic test build

[**Uvaga-Windows-Synthetic-r18.zip 다운로드**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41415006414/artifacts/13127022813)

이 파일은 **정식 릴리스도 production 빌드도 아닙니다**. PR #34의 synthetic 시나리오를 수동 검증하기 위한 테스트 아카이브입니다. GitHub Actions artifact를 다운로드하려면 GitHub 로그인이 필요할 수 있습니다.

- Source head: `a2c945034bfea58b3ff5890ea53862831c1b502a`
- ZIP SHA256: `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`
- `Uvaga.exe` SHA256: `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`
- 자동 Windows/macOS 검사: **PASS**
- r18 Windows 수동 runtime 승인: **대기 중**

테스트 UI에서는 synthetic 지역(`UA-46` 또는 `UA-30`)을 선택하고 active/fresh, active/stale, cleared/fresh, cleared/stale, unknown/unavailable의 다섯 가지 fail-safe 시나리오를 확인할 수 있습니다.

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
