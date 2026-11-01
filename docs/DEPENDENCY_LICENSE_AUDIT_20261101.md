# DEPENDENCY_LICENSE_AUDIT_20261101

Дата: 2026-11-01. Обсяг: прямі залежності, які вже з'явилися у draft PR; це НЕ повний аудит транзитивного дерева і НЕ release approval.

## Proprietary compatibility — preliminary
| Компонент | Використання | Upstream license | Попередній висновок |
|---|---|---|---|
| Tauri 2.12.5 | app shell | MIT / Apache-2.0 where applicable | permissive; допускає proprietary продукт за умови notices |
| tauri-build 2.7.4 | build dependency | Tauri project MIT / Apache-2.0 | permissive; notices required as applicable |
| axum 0.8.9 | local/future relay HTTP | MIT | permissive |
| Tokio 1.53.x LTS | relay async runtime | MIT | permissive |
| Serde 1.x | relay JSON serialization | MIT OR Apache-2.0 | permissive |

Джерела перевірки:
- https://github.com/tauri-apps/tauri
- https://github.com/tokio-rs/axum
- https://github.com/tokio-rs/tokio
- https://github.com/serde-rs/serde

## Important Tauri branding boundary
Tauri project code is permissively licensed, але Tauri logo/branding має окремі умови. «ТривогА» не використовує Tauri logo як власну іконку чи бренд. Поточні icon.png/icon.ico в PR #4 — технічні placeholder assets, створені для compile checks, не release branding.

## Before first distributed build
Обов'язково:
1. Commit exact Cargo.lock for every distributed Rust application/service.
2. Run a reproducible license inventory over the exact lockfiles, including transitive crates.
3. Reject copyleft/restricted/unknown dependency unless owner explicitly reviews and approves compatibility.
4. Generate/update THIRD_PARTY_NOTICES.md with exact package/version/license and required notices.
5. Check bundled native components/WebView/runtime assets and platform installer obligations separately.
6. Keep upstream notices in distributed package where licenses require them.
7. Repeat the audit when lockfile changes.

## Current status
Direct dependency families above show no immediate proprietary-license conflict. **Full release license gate: NOT DONE.** No public binary release is approved by this document.
