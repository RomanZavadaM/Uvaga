# ТривогА / Uvaga

**Uvaga (ТривогА)** は、ウクライナの住民に対して、公式の空襲警報と検証済みの追加脅威情報を迅速に提供するためのプロジェクトです。

> **現在の状態:** 初期プロトタイプ / synthetic-only。production 配備、live API 認証情報、公開通知配信はまだありません。本アプリは公式警報チャネルの代替ではなく、メッセージがないことは危険がないことを意味しません。

**README 言語:** [Українська](README.md) · [English](README.en.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [한국어](README.ko.md) · 日本語

## テスト用ダウンロード

### Windows 10/11 x64 — r18 synthetic test build

[**Uvaga-Windows-Synthetic-r18.zip をダウンロード**](https://github.com/RomanZavadaM/Uvaga/actions/runs/41415006414/artifacts/13127022813)

これは **正式リリースでも production build でもありません**。PR #34 の synthetic シナリオを手動確認するためのテスト用アーカイブです。GitHub Actions artifact のダウンロードには GitHub へのサインインが必要な場合があります。

- Source head: `a2c945034bfea58b3ff5890ea53862831c1b502a`
- ZIP SHA256: `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`
- `Uvaga.exe` SHA256: `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`
- Windows/macOS 自動チェック: **PASS**
- r18 Windows 手動 runtime 受け入れ: **保留中**

テスト UI では synthetic な地域 (`UA-46` または `UA-30`) を選び、active/fresh、active/stale、cleared/fresh、cleared/stale、unknown/unavailable の 5 つの fail-safe シナリオを確認できます。

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
