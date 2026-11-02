# Windows distribution — ТривогА

## r20 test-installer policy

r20 готує **unsigned NSIS test installer candidate** для Windows 10/11 x64. Це не production release і не заміна офіційних каналів оповіщення.

### Installer profile

- format: NSIS `*-setup.exe`;
- install mode: `currentUser` — без вимоги Administrator; штатно встановлюється у user scope;
- installer languages: Ukrainian, English, French, German, Spanish, Korean, Japanese;
- app identifier: `ua.zavada.uvaga`;
- publisher metadata: `Roman Zavada`;
- target: Windows 10/11 x64;
- code signing: відсутній у r20 test candidate;
- downgrade: blocked (`allowDowngrades=false`);
- WebView2: embedded bootstrapper; якщо runtime відсутній, bootstrapper може потребувати мережі;
- updater: відсутній;
- live API credentials / notifications / geolocation: відсутні.

Official Tauri references:
- https://v2.tauri.app/distribute/windows-installer/
- https://v2.tauri.app/reference/config/
- https://v2.tauri.app/distribute/

## Active CI candidate identity

Historical accepted artifacts keep their original r19/r20 names, IDs and hashes in project evidence. Active workflows must not reuse those revision labels for newer source heads.

Current active Windows CI naming:
- installer/release-executable candidate: `Uvaga-Windows-Installer-candidate-<run>`.

The standalone active Windows synthetic workflow was retired in r22 to avoid duplicate Windows compilation. Historical r17/r18/r19/r21 synthetic artifacts remain immutable evidence. The canonical Windows candidate still runs exact-lock tests, builds the release executable and NSIS installer, verifies unsigned provenance, performs install/launch/uninstall smoke, and packages checksums/licenses.

Each candidate metadata file records app version, source head, GitHub checkout SHA and run ID.

## Version consistency

The Windows app/installer version is currently `0.1.0`. r21 does not bump it.

`src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json` must declare the same app version; CI fails on drift. A public-release version bump is a separate owner-approved checkpoint because it changes release identity and the generated Cargo.lock checkpoint.

## Provenance

Installer candidate приймається тільки якщо:

1. source head зафіксований у GitHub PR;
2. exact accepted Cargo.lock SHA256 = `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
3. application test/build працює з `--locked`;
4. `tauri-cli` pin = `2.12.4`, official Windows archive SHA256 `3e8687fb184bd7f10ebd8642f7715f79f4a605c0694c142a21507e8a24dc0f8b`;
5. installer/executable та accompanying files мають SHA256SUMS;
6. Windows exact-lock third-party license text coverage = 222/222, missing 0, extra 0;
7. artifact після upload завантажено назад і checksum перевірено.

## Manual install acceptance

Перед будь-яким public release власник має на чистому/звичайному Windows 10/11 x64 перевірити:

- запуск installer без admin elevation у current-user mode;
- коректне ім’я/іконку/мову;
- запуск застосунку після install;
- synthetic warning залишається очевидним;
- усі 5 synthetic fail-safe сценаріїв працюють як у r18/r19;
- stale clear не виглядає як current safe clear;
- uninstall видаляє застосунок і shortcut без пошкодження інших даних;
- повторне встановлення/оновлення не погіршує стан;
- installer hash збігається з опублікованим evidence.

## Production release gate

До production distribution окремо потрібні: code signing, стабільне non-expiring розміщення license/MPL source-availability material, release provenance, owner acceptance, і повторний exact-lock audit для фінального release artifact.
