from __future__ import annotations

import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[1]
SYNTH = ROOT / '.github' / 'workflows' / 'windows-synthetic-artifact.yml'
INSTALLER = ROOT / '.github' / 'workflows' / 'windows-installer-candidate.yml'
CHECKLIST = ROOT / 'RELEASE_CHECKLIST.md'

FORBIDDEN_DOC_PATHS = {
    'README.md',
    'README*.md',
    'LICENSE.md',
    'THIRD_PARTY_NOTICES.md',
    'SECURITY.md',
    'docs/**',
    'docs/WINDOWS_DISTRIBUTION.md',
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise SystemExit(message)


def pull_request_paths(path: pathlib.Path) -> set[str]:
    lines = path.read_text(encoding='utf-8').splitlines()
    in_paths = False
    values: list[str] = []
    for line in lines:
        if line == '    paths:':
            in_paths = True
            continue
        if not in_paths:
            continue
        if line.startswith('      - '):
            raw = line.split('-', 1)[1].strip()
            if len(raw) >= 2 and raw[0] == raw[-1] and raw[0] in {'\'', '"'}:
                raw = raw[1:-1]
            values.append(raw)
            continue
        if line.strip() and len(line) - len(line.lstrip()) <= 4:
            break
    require(bool(values), f'no pull_request paths parsed from {path}')
    return set(values)


def validate_heavy(path: pathlib.Path) -> None:
    text = path.read_text(encoding='utf-8')
    paths = pull_request_paths(path)
    bad = sorted(paths & FORBIDDEN_DOC_PATHS)
    require(not bad, f'{path.name}: documentation-only paths trigger heavy workflow: {bad}')
    require('  workflow_dispatch:' in text, f'{path.name}: workflow_dispatch must stay available for final candidate')


validate_heavy(SYNTH)
validate_heavy(INSTALLER)

checklist = CHECKLIST.read_text(encoding='utf-8')
for phrase in [
    'workflow_dispatch',
    'Exact release source SHA',
    'Drive copy',
    'Owner manual acceptance',
    'Critical cleared-stale',
    'CI PASS сам по собі',
]:
    require(phrase in checklist, f'RELEASE_CHECKLIST missing required gate: {phrase}')

print('CI scope contract: PASS')
print(f'synthetic_pr_paths={sorted(pull_request_paths(SYNTH))}')
print(f'installer_pr_paths={sorted(pull_request_paths(INSTALLER))}')
print('docs_only_heavy_triggers=0')
