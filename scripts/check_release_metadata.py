from __future__ import annotations

import argparse
import json
import pathlib
import re
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
CARGO = ROOT / 'src-tauri' / 'Cargo.toml'
CONFIG = ROOT / 'src-tauri' / 'tauri.conf.json'
SYNTH_WORKFLOW = ROOT / '.github' / 'workflows' / 'windows-synthetic-artifact.yml'
INSTALLER_WORKFLOW = ROOT / '.github' / 'workflows' / 'windows-installer-candidate.yml'
HISTORICAL_INSTALLER_WORKFLOW = ROOT / '.github' / 'workflows' / 'windows-installer-r20.yml'

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

    synth = SYNTH_WORKFLOW.read_text(encoding='utf-8')
    installer = INSTALLER_WORKFLOW.read_text(encoding='utf-8')
    require('r19' not in synth.lower(), 'active synthetic workflow must not emit historical r19 identity')
    require('r20' not in installer.lower(), 'active installer workflow must not emit historical r20 identity')
    require('Synthetic-candidate-${{ github.run_number }}' in synth, 'synthetic candidate artifact identity missing run number')
    require('Installer-candidate-${{ github.run_number }}' in installer, 'installer candidate artifact identity missing run number')
    require('Artifact track: synthetic-candidate' in synth, 'synthetic BUILD_INFO track missing')
    require('Artifact track: installer-candidate' in installer, 'installer BUILD_INFO track missing')
    require(not HISTORICAL_INSTALLER_WORKFLOW.exists(), 'historical r20 installer workflow must stay retired')

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
    print('artifact_identity=generic candidates; historical r19/r20 workflow labels absent')


if __name__ == '__main__':
    main()
