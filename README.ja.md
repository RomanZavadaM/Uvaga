# ТривогА / Uvaga

**Uvaga (ТривогА)** は、ウクライナの住民に対して、公式の空襲警報と検証済みの追加脅威情報を迅速に提供するためのプロジェクトです。

> **現在の状態:** 初期プロトタイプ / synthetic-only。production 配備、live API 認証情報、公開通知配信はまだありません。本アプリは公式警報チャネルの代替ではなく、メッセージがないことは危険がないことを意味しません。

**README の主要言語: [Українська](README.md)。** 翻訳: [English](README.en.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [한국어](README.ko.md) · 日本語

## テスト用ダウンロード

### Windows 10/11 x64 — r20 NSIS installer candidate

[**Uvaga-Windows-Installer-r20.zip をダウンロード**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41447023597/artifacts/13142977634)

これは **未署名のテスト installer で、production release ではありません**。CI は current-user silent install、HKCU 登録、インストール済みアプリ起動、silent uninstall を検証済みです。所有者による installer UX / visual sanity はまだ保留中です。r20 は完全な automated install/uninstall evidence を保持する最新の archived installer test candidate であり、r21–r23 は CI/release-process hardening で、新しい owner-accepted release artifact は作成していません。

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

[**Uvaga-Windows-Synthetic-r19.zip をダウンロード**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41442359441/artifacts/13140138112)

これは **正式リリースでも production build でもありません**。検証可能な evidence として保存されている歴史的な r19 synthetic desktop-hardening build です。r18 は所有者が Windows で手動受け入れした最後の build で、r19 の owner visual sanity はまだ保留中です。

- Source head: `cc4992975b73f0211cdd44b05f7ebd52d76a5415`
- ZIP SHA256: `7f2dc03cd58686e2555026a544d78673f9fdcf7b452c44913b40013ee34bffad`
- `Uvaga.exe` SHA256: `c9afe5873aad563dd6f07b51a6b05cc64fac3057e91f1127d2794b3372b7b593`
- `Cargo.lock` SHA256: `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`
- UI contract / Windows / macOS ARM / macOS Intel: **PASS**
- r19 を `main` に統合: `1af2e5d2c53a00619172056bab9fde746daff988`
- r19 owner visual sanity: **PENDING**
- r18 owner manual runtime 受け入れ: **PASS — 2026-11-02**

r19 は重要状態をよりコンパクトかつアクセスしやすくし、fail-closed dependency lock と自己説明型 test artifact を追加します。live source は接続されていません。

## 対応予定プラットフォーム

- Windows 10/11 x64
- macOS Apple Silicon
- macOS Intel
- Android
- iOS
- Web
- Telegram — 後に統合出力経由で対応予定

Windows 7 はサポートしません。

## アーキテクチャ方針

- Rust — 状態、検証、正規化のコアロジック。
- Tauri 2 — desktop/mobile アプリケーション shell。
- 軽量で制御された Rust relay/cache — secret API token、rate limit、正規化された public contract の境界。
- PostgreSQL は基本要件ではありません。運用状態は短命で、recovery storage は別途定義します。
- GitHub が source of truth であり、Google Drive は主要文書の一方向 mirror であって代替エディタではありません。

詳細: [REQUIREMENTS.md](REQUIREMENTS.md) · [ARCHITECTURE.md](ARCHITECTURE.md) · [SOURCE_VALIDATION.md](SOURCE_VALIDATION.md)。

## Safety invariants

- 公式の警報解除は、より新しい確認済み公式イベントとしてのみ受け入れます。
- timeout、TTL 失効、HTTP error、空レスポンス、ソース停止は **警報解除を生成してはなりません**。
- 補助データや集約データは、単独で公式ソースを置き換えることはできません。
- 新鮮なソースがない古い確認済み clear は、現在の安全状態として表示しません。
- 本プロジェクトは軍事対象、正確な座標、軌跡の公開 live tracking を目的としていません。

## 開発

新しいセッションの開始点は **[START_HERE.md](START_HERE.md)** です。

次に読むもの:
1. [PROJECT_RULES.md](PROJECT_RULES.md)
2. [PROJECT_STATE.md](PROJECT_STATE.md)
3. [WORKLOG.md](WORKLOG.md)
4. [HANDOFF_CURRENT.md](HANDOFF_CURRENT.md)
5. live GitHub PR/CI 状態

検証と統合は [GitHub Issue #9](https://github.com/RomanZavadaM/Uvaga/issues/9) で管理します。

## 方法論

このプロジェクトは **dev-strategy v2 RC の試験適用**です。C13 clean-session continuity は **PASS** です。日常作業はプロジェクト内ルールだけで継続でき、中央 `dev-strategy` リポジトリは各セッションの runtime 依存ではありません。オーナーによる完全受け入れ (C14) はまだ宣言されていません。

## ライセンス

**Copyright © 2026 Roman Zavada (Роман Завада). All rights reserved.**

Uvaga は **proprietary software** です。リポジトリが公開されていても open-source ライセンスが付与されるわけではありません。[LICENSE.md](LICENSE.md)、[COPYRIGHT.md](COPYRIGHT.md)、[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) を参照してください。
