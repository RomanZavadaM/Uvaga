# r20 — Windows installer / distribution readiness

## Мета

Підготувати відтворюваний unsigned Windows NSIS **test installer candidate** для synthetic-only desktop застосунку без production/live scope.

## Predeclared acceptance

1. Tauri bundling вмикається лише для штатного Windows NSIS candidate; install mode = currentUser, без admin-вимоги; `allowDowngrades=false`; WebView2 = embedded bootstrapper.
2. Build toolchain pins `tauri-cli 2.12.4`; application dependency graph перед build мусить відтворити exact accepted Cargo.lock SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.
3. Installer build використовує `--locked`; silent dependency drift fail-closed.
4. Artifact містить installer, release executable, README, proprietary LICENSE, THIRD_PARTY_NOTICES, build metadata і SHA256SUMS.
5. Artifact також містить exact Windows full third-party license texts із вже прийнятого evidence для того самого verified lock; coverage має залишатись 222/222 external normal+build, missing 0, extra 0, а embedded Cargo.lock має мати той самий accepted SHA256.
6. MPL-covered source-availability material не губиться: candidate містить source-availability notice і посилання/ідентифікатор перевіреного source bundle; production release потребує non-expiring location.
7. SECURITY.md та Windows distribution document чітко описують unsigned/test status, SmartScreen/signing limits, installation/uninstallation, provenance/checksums і канали повідомлення про уразливості.
8. Installer candidate не містить live API credentials, updater endpoint, public notifications, geolocation, production deployment або signing keys.
9. Windows installer workflow і existing UI/docs/desktop checks — PASS.
10. Після build artifact завантажується назад і SHA256/readback перевіряється; installer існує та checksum відповідає.
11. Owner manual install/uninstall/visual sanity — окремий acceptance gate; automated PASS не дорівнює production release.

## Scope boundary

r20 не підключає live source adapter. Не додає автооновлення. Не публікує GitHub Release. Не підписує executable/installer. Не змінює safety mapping.
