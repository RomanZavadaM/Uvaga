from __future__ import annotations

import pathlib
import re
import sys
from html.parser import HTMLParser

ROOT = pathlib.Path(__file__).resolve().parents[1]
HTML = (ROOT / "app" / "index.html").read_text(encoding="utf-8")
JS = (ROOT / "app" / "app.js").read_text(encoding="utf-8")
RUST = (ROOT / "src-tauri" / "src" / "main.rs").read_text(encoding="utf-8")

EXPECTED_TERRITORIES = {"UA-46", "UA-30"}
EXPECTED_SCENARIOS = {
    "active_stale",
    "active_fresh",
    "cleared_fresh",
    "cleared_stale",
    "unknown_unavailable",
}


class ContractParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.select_stack: list[str | None] = []
        self.territories: set[str] = set()
        self.scenarios: set[str] = set()
        self.remote_assets: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        values = dict(attrs)

        if tag == "select":
            if "data-territory" in values:
                self.select_stack.append("territory")
            elif "data-scenario" in values:
                self.select_stack.append("scenario")
            else:
                self.select_stack.append(None)
        elif tag == "option" and self.select_stack:
            value = values.get("value")
            if value:
                if self.select_stack[-1] == "territory":
                    self.territories.add(value)
                elif self.select_stack[-1] == "scenario":
                    self.scenarios.add(value)

        if tag in {"script", "link"}:
            ref = values.get("src") or values.get("href")
            if ref and ref.startswith(("http://", "https://")):
                self.remote_assets.append(ref)

    def handle_endtag(self, tag: str) -> None:
        if tag == "select" and self.select_stack:
            self.select_stack.pop()


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


parser = ContractParser()
parser.feed(HTML)

require(parser.territories == EXPECTED_TERRITORIES, f"territory whitelist drift: {parser.territories}")
require(parser.scenarios == EXPECTED_SCENARIOS, f"scenario whitelist drift: {parser.scenarios}")
require(not parser.remote_assets, f"remote UI assets are forbidden: {parser.remote_assets}")

require("СИНТЕТИЧНИЙ РЕЖИМ" in HTML, "synthetic-mode banner missing")
require("<dt>Останній підтверджений стан</dt>" in HTML, "historical state label must remain explicit")
require("Це не система реального оповіщення." in HTML, "not-real-alerting warning missing")
require('role="status"' in HTML and 'aria-live="polite"' in HTML, "accessible live status missing")
require('aria-busy="true"' in HTML, "initial aria-busy state missing")
require("data-app-version" in HTML, "app version surface missing")
require('data-bridge data-presentation-kind="unknown"' in HTML, "bridge presentation-kind surface missing")
require(HTML.count(" data-presentation>") == 1, "presentation output selector must be unique")

require(
    'clear_needs_refresh: "Відбій потребує повторного підтвердження"' in JS,
    "critical stale-clear label changed",
)
require('setPresentationKind("error")' in JS, "IPC error must force error/unknown presentation")
require("clearState();" in JS, "IPC error must clear previous snapshot")
require('env!("CARGO_PKG_VERSION")' in RUST, "Rust payload must expose local app version")
require("app_version" in RUST, "app_version JSON field missing")

readme_paths = [
    ROOT / "README.md",
    ROOT / "README.en.md",
    ROOT / "README.fr.md",
    ROOT / "README.de.md",
    ROOT / "README.es.md",
    ROOT / "README.ko.md",
    ROOT / "README.ja.md",
]
pending_pattern = re.compile(
    r"^[-*] [^\n]*r18[^\n]*(ще не завершене|pending|en attente|ausstehend|pendiente|대기 중|保留中)",
    re.IGNORECASE | re.MULTILINE,
)
for path in readme_paths:
    text = path.read_text(encoding="utf-8")
    require("PR #34" not in text, f"{path.name}: stale open-PR wording remains")
    require(not pending_pattern.search(text), f"{path.name}: stale r18 manual-pending wording remains")

print("UI contract PASS")
print(f"territories={sorted(parser.territories)}")
print(f"scenarios={sorted(parser.scenarios)}")
print("remote_assets=0")
