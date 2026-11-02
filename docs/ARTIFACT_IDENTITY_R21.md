# r21 — artifact identity and version metadata hardening

## Мета

Прибрати історичні r19/r20 ярлики з активних CI artifacts і зробити app/installer version metadata fail-closed та однозначною.

## Predeclared acceptance

1. Активний synthetic workflow не створює нові artifacts з історичною назвою r19.
2. Активний installer workflow не створює нові artifacts/BUILD_INFO з історичною назвою r20.
3. Майбутні CI artifacts мають generic candidate identity + source head/run metadata; історичні r19/r20 artifact IDs у документації не змінюються.
4. `src-tauri/Cargo.toml` package version і `src-tauri/tauri.conf.json` app version мають збігатись; drift = CI failure.
5. Rust UI payload version походить із `CARGO_PKG_VERSION`, тому consistency gate гарантує відповідність UI ↔ installer metadata.
6. Installer BUILD_INFO містить app version, source PR head, GitHub checkout SHA, run id, Tauri CLI version/hash та accepted Cargo.lock SHA.
7. Synthetic TEST_ARTIFACT metadata містить app version, source head, run id та accepted Cargo.lock SHA.
8. Exact accepted dependency lock SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d` не змінюється.
9. Історичні r19/r20 artifacts і їх hashes лишаються immutable evidence.
10. Existing docs/UI/desktop/installer checks PASS.
11. Немає bump app version, product logic change, live source, signing, updater, public release або production deployment у r21.

## Version policy boundary

Поточна prototype app version лишається 0.1.0. r21 лише гарантує узгодженість. Окремий owner-approved version bump перед public release потребуватиме нового dependency/release checkpoint.
