from __future__ import annotations

import argparse
import json
import pathlib
import re
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
CARGO = ROOT / 'src-tauri' / 'Cargo.toml'
CONFIG = ROOT / 'src-tauri' / 'tauri.conf.json'

EXPECTED_PRODUCT = 'ТривогА'
EXPECTED_IDENTIFIER = 'ua.zavada.uvaga'
EXPECTED_PUBLISHER = 'Roman Zavada'
EXPECTED_LANGUAGES = {'Ukrainian', 'English', 'French', 'German', 'Spanish', 'Korean', 'Japanese'}
SEMVER = re.compile(r'^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$')


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def load() -> tuple[str, dict]:
    cargo = tomllib.loads(CARGO.read_text(encoding='utf-8'))
    config = json.loads(CONFIG.read_text(encoding='utf-8'))
    cargo_version = cargo['package']['version']
    config_version = config.get('version')

    require(cargo_version == config_version,
            f'app version drift: Cargo.toml={cargo_version!r} tauri.conf.json={config_version!r}')
    require(bool(SEMVER.fullmatch(cargo_version)), f'app version is not semver: {cargo_version!r}')
    require(config.get('productName') == EXPECTED_PRODUCT, 'unexpected productName')
    require(config.get('identifier') == EXPECTED_IDENTIFIER, 'unexpected app identifier')

    bundle = config.get('bundle') or {}
    require(bundle.get('active') is True, 'bundle.active must stay true')
    require('nsis' in (bundle.get('targets') or []), 'NSIS bundle target missing')
    require(bundle.get('publisher') == EXPECTED_PUBLISHER, 'unexpected Windows publisher metadata')

    windows = bundle.get('windows') or {}
    require(windows.get('allowDowngrades') is False, 'Windows downgrades must stay disabled')
    webview = windows.get('webviewInstallMode') or {}
    require(webview.get('type') == 'embedBootstrapper', 'WebView2 mode must stay embedBootstrapper')
    require(webview.get('silent') is True, 'WebView2 bootstrapper must stay silent')

    nsis = windows.get('nsis') or {}
    require(nsis.get('installMode') == 'currentUser', 'NSIS installMode must stay currentUser')
    require(set(nsis.get('languages') or []) == EXPECTED_LANGUAGES, 'NSIS language set drift')

    plugins = config.get('plugins') or {}
    require('updater' not in plugins, 'updater must not be enabled in current test scope')

    return cargo_version, config


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--print-version', action='store_true')
    args = parser.parse_args()
    version, _ = load()
    if args.print_version:
        print(version)
        return
    print('Release metadata contract: PASS')
    print(f'app_version={version}')
    print(f'product={EXPECTED_PRODUCT}')
    print(f'identifier={EXPECTED_IDENTIFIER}')
    print('installer=nsis/currentUser; downgrades=false; updater=absent')


if __name__ == '__main__':
    main()
