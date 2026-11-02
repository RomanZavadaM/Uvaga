from __future__ import annotations

import hashlib
import pathlib
import sys

LOCK = pathlib.Path(__file__).resolve().parents[1] / "src-tauri" / "Cargo.lock"
EXPECTED_SHA256 = "6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d"

if not LOCK.is_file():
    raise SystemExit(f"missing lock file: {LOCK}")

actual = hashlib.sha256(LOCK.read_bytes()).hexdigest()
print(f"Cargo.lock SHA256: {actual}")
if actual != EXPECTED_SHA256:
    raise SystemExit(
        "Cargo.lock drift detected; dependency graph changed and must be reviewed "
        f"before build. expected={EXPECTED_SHA256} actual={actual}"
    )

text = LOCK.read_text(encoding="utf-8")
if 'name = "cc"\nversion = "1.10.0"' not in text:
    raise SystemExit("verified lock must contain cc 1.10.0")

print("Verified Tauri dependency lock: PASS")
