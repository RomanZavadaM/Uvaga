# VERIFICATION — ТривогА

**Оновлено:** 2026-11-01  
Обсяг: organizational checkpoint + offline/synthetic prototype slices. Незалежний reviewer PASS ще відсутній.

## GitHub
- Public repo: `RomanZavadaM/Uvaga`.
- Initial main: `ad8653ebcdf16507be54a929b690c8a1060e035b`.
- PR #1–#8, #10, #11 — draft/open prototype work; не production.
- Independent review/integration gate: Issue #9.

## Confirmed CI
| Slice | Run | Result |
|---|---:|---|
| docs checkpoint | 41338991422 | SUCCESS |
| r1 alert core | 41335891826 | SUCCESS |
| r2 multi-region | 41336873203 | SUCCESS |
| r3 Tauri desktop matrix | 41338709796 | SUCCESS: Windows x64, macOS ARM64, macOS Intel |
| r4 source trust | 41337182862 | SUCCESS |
| r5 localhost relay | 41337322695 | SUCCESS |
| r6 alerts parser | 41337434827 | SUCCESS |
| r7 freshness | 41337648092 | SUCCESS |
| r8 recovery fixed | 41338929570 | SUCCESS |
| r9 presentation | 41338934425 | SUCCESS |

PR #1 docs changed after run 41338991422; newest docs CI must be checked before merge.

## Tauri build issues actually found and fixed
1. Linux compile initially failed because Tauri required `icons/icon.png`.
2. Setting an empty icon list did not eliminate that requirement.
3. Added technical PNG placeholder; Linux compile run 41337554330 passed.
4. Windows matrix then found missing `icons/icon.ico`.
5. Added technical ICO placeholder.
6. Product matrix run 41338709796 passed on all three agreed desktop architecture targets.
7. Direct Tauri dependencies pinned to CI-resolved tauri 2.12.5 / tauri-build 2.7.4.
8. CI-generated Cargo.lock preserved as artifact; lock SHA256 `420880998b64ca0e1a882c781b7d4f1577b9873be8b943ae3f425482bfb514fd`. File not yet committed to repo.

These placeholder icons are compile assets, not product branding.

## Source/document audit
UkraineAlarm public access page and published offer were reviewed. No credentials or live endpoint calls were made. Architecture records API access as fallible, changeable, and requiring a controlled relay/adapter boundary.
alerts.in.ua docs were reviewed for token secrecy, public proxy requirement, limits/caching, compact v2 status semantics and fixed 27-oblast order. PR #7 remains offline parser only.

## License
Project remains proprietary. Preliminary direct dependency audit:
- Tauri: MIT/Apache-2.0 family;
- axum: MIT;
- Tokio: MIT;
- Serde: MIT/Apache-2.0.
Full transitive dependency/license inventory is NOT DONE and remains a release gate.

## Drive mirror
Root: https://drive.google.com/drive/folders/1sPq7Ajmi4uhydjK7Wy1B-vR5IwTFVt8b
Five continuity docs were written to PROJECT and read back. Stored GitHub blob SHA markers matched, and normalized text matched GitHub.
Verification record: https://docs.google.com/document/d/1W7cGX-l0ozK7Se8Q0OEN8WM-WqanVBwv-V53JRqAlgA/edit

Google Docs can collapse blank Markdown paragraphs, therefore this is semantic/text mirror verification, not byte-identical Markdown preservation.

## Backup limitation
Attempt to create a full mirror clone/Git bundle from the container failed before cloning because that environment could not resolve github.com. Therefore:
- Drive key-doc mirror readback: PASS.
- Full Git-history independent backup: NOT DONE.
- Restore branches/tags/commits from independent backup: NOT DONE.
C12 remains partial.

## Explicitly not verified / not done
- Independent other-person/other-AI review.
- Merge/retarget sequence to main.
- Android/iOS build.
- Exact production API contract/credentials/latency/webhook behavior.
- Public notification/background push.
- Full Git backup/restore.
- Full transitive license gate.
- Release artifacts, signing, installers/portable packages.

No production or public alerting was performed.
