# PROJECT_STATE — ТривогА

**Оновлено:** 2026-11-01  
**Repository:** `RomanZavadaM/Uvaga`  
**Методологія:** dev-strategy v2 RC — пробне застосування, не прийнято повністю.

## Інтегрований продукт-код
Last integrated product-code base: `bba0bd82351fa06f20dbe144e8ac487b81314e4f` (r16).

До `main` уже інтегровані:
- fail-safe official alert state, freshness, recovery і presentation;
- source trust/normalization та provider lifecycle identity;
- alerts.in.ua offline parser;
- localhost synthetic relay;
- Tauri 2 desktop shell;
- source-neutral territory hierarchy, provider mapping і hierarchical subscriptions;
- mapped source-territory normalization;
- KATOTTG cutoff/amendment policy + staging validator;
- UkraineAlarm public-offer audit;
- deterministic attention ranking;
- r16 local Tauri UI ↔ Rust core bridge.

Live `main` SHA на початку кожної сесії читати з GitHub, а не відновлювати з чату.

## Поточний checkpoint — r17
Open draft PR #28: Windows synthetic runtime artifact.

Final verified source head:
`2a9ef7e1d30c50a42dedcf822a2c8728a6d1d490`

CI run `41352681448`: PASS.
- generate Cargo.lock: PASS;
- `cargo test --locked`: PASS;
- `cargo build --locked`: PASS;
- checksum creation/upload: PASS.

Verified artifact:
- `Uvaga.exe` SHA256 `220a15adc02ab03ff2f067f01a3f67a31b97fc88c01eb46a0633d4ee9022acd8`;
- `Cargo.lock` SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- ZIP SHA256 `c02fab2a009e85ea3b9218f800e00c8e53be24f6e8b5789f3e188344bf281c98`.

**Manual Windows 10/11 x64 runtime acceptance: PENDING.**
PR #28 не зливати до результату ручного запуску власником.

## Safety invariants
- source outage/timeout/TTL/auth/schema error never creates official clear;
- aggregator/supplemental data cannot self-promote to official state;
- stale/unavailable previous clear is not current clear;
- provider territory IDs become core identity only through audited mapping;
- no live military target tracking/trajectories;
- no production deployment/public notifications/live credentials without separate owner decision.

## Виконання
Один активний виконавець за замовчуванням. Паралельність — лише за рішенням власника.
Якщо пам'ять сесії суперечить GitHub: STOP → READ → VERIFY → RECONCILE → CONTINUE.

## Sources
- UkraineAlarm public request page/offer rechecked 2026-11-01; no application/credentials/live call.
- alerts.in.ua public docs researched; token belongs only behind relay.
- KATOTTG production snapshot must use Ministry base + discovered amendment chain to explicit cutoff.

## Drive / recovery
GitHub — source of truth.
Drive PROJECT mirror exists.
BACKUPS currently contains raw Tauri Cargo.lock checkpoint and the verified r17 Windows ZIP.
Full Git-history backup + fresh restore test is still open.

## Відкрите
- manual runtime acceptance of r17;
- full Git bundle + restore test (C12);
- full transitive dependency/license gate before release;
- Android/iOS build/runtime validation;
- live API credentials/schema/latency/webhook/security acceptance only after separate owner decision;
- push/background delivery;
- production packaging/signing/release.

## Одна наступна дія
Власник запускає r17 `Uvaga.exe` на Windows 10/11 x64 і повідомляє фактичний результат. Після PASS — merge PR #28; після FAIL — зафіксувати симптом і виправити до merge.
