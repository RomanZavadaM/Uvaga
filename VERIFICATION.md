# VERIFICATION — ТривогА

**Оновлено:** 2026-11-02

## Integrated product state
r16 product-code base:
`bba0bd82351fa06f20dbe144e8ac487b81314e4f`.

r17 artifact/manual acceptance:
`0ea256b3cc1a72250a8fa06e3098d13fe780b770`.

C12 source/history backup checkpoint:
`81c3855112006d9350077b2a5b38f62e3701f73d`.

Live `main` must always be checked in GitHub.

## r17 manual runtime
**PASS — 2026-11-02.**

Owner screenshot evidence confirms:
- executable starts on Windows;
- explicit synthetic/not-real-alerting warning;
- local Tauri IPC success;
- UA-46;
- confirmed state active;
- source freshness stale;
- presentation active-last-known;
- no false clear.

## C12 Git backup/restore
**PASS — 2026-11-02 for Git repository source/history.**

Post-merge run:
`41405252149` — PASS.

Accepted source main:
`81c3855112006d9350077b2a5b38f62e3701f73d`.

GitHub artifact:
- id `13123246036`;
- ZIP digest `sha256:d236ae8b26179f44e64365b20f08484a65e1730633e7319386d2a14ecbe4de06`.

Bundle:
- `Uvaga-all-refs.bundle`;
- SHA256 `943ee4667b69871d8f851afb2695396a6a5a5ea6c820433c5f3083bcfdcf7a53`.

CI evidence:
- fetch all branches/tags: PASS;
- bundle create/verify: PASS;
- clean restore from bundle: PASS;
- restored HEAD equals source main: PASS;
- `git fsck --full`: PASS;
- restored Rust tests: PASS;
- artifact upload: PASS.

Additional readback:
- downloaded artifact ZIP SHA equals GitHub digest: PASS;
- inner SHA256SUMS: PASS;
- bundle verify: PASS;
- bundle reports complete history: PASS;
- independent clean restore reproduces exact source main SHA: PASS;
- fsck: PASS.

Drive raw backup:
https://drive.google.com/file/d/1ybaj5qBhXWNprZ7v4GLvIZeDXTr4xobp/view?usp=drivesdk

Drive verification document:
`ТривогА — verification — C12 Git bundle restore — 2026-11-02`.

Scope limit:
This C12 evidence does not cover future production databases, secrets, provider credentials, device tokens or user data.

## C13 clean-session continuity
**PASS — 2026-11-02.**

A clean session reconstructed state from project-local documents and live GitHub only.

Verified:
- live start main `3a254e4a6edb3bcd2294cb0905ab7ddae4f1ca61`;
- no open pull requests;
- r17 Windows manual runtime PASS plus successful Actions run `41352681448`;
- C12 clean Git restore PASS plus successful post-merge run `41405252149`;
- current production/live-credential/public-notification restrictions;
- correct next product slice: richer synthetic scenarios / territory selection, without live source adapter.

Detailed evidence: `C13_CLEAN_SESSION.md`.

## r18 synthetic territory/scenario verification
**AUTOMATED PASS / MANUAL WINDOWS PENDING — 2026-11-02.**

PR #34 verified product source head:
`a2c945034bfea58b3ff5890ea53862831c1b502a`.

Current PR head:
`cccce86708e77e6a1cd670179ebf9547d0a5f671`.

A direct compare from `a2c945...` to `cccce867...` shows only:
- `README.md`;
- `README.en.md`;
- `README.fr.md`;
- `README.de.md`;
- `README.es.md`;
- `README.ko.md`;
- `README.ja.md`.

Therefore no r18 product/source file changed after the verified manual-test artifact was built.

Current-head automated evidence:
- Organizational docs checks `41417199329`: PASS;
- Tauri shell product-platform checks `41417199299`: PASS;
- Windows synthetic test artifact `41417199372`: PASS.

Manual-test artifact evidence from verified product head `a2c945...`:
- artifact id `13127022813`;
- ZIP SHA256 `b9f3f2c47d1333720568e949d957005aa09b8138bf4844e7b9e422ad31cfb444`;
- `Uvaga.exe` SHA256 `f660f2e42a3f6a921230feb1e847bccf6b8e6513beea47bbd156a7714d93fafa`;
- `Cargo.lock` SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.

Automated acceptance covers build/tests/artifact/checksums only. Visual/runtime UX still requires owner execution on Windows 10/11 x64. Until that PASS, PR #34 is not accepted and must not be merged.

## README localization verification
PR #35 merged to main at `e3d11e7ac47ee72a9d22ab5205115dac1d8a8956`.
Ukrainian primary README plus EN/FR/DE/ES/KO/JA translations are present; local Markdown link validation PASS.

## Dependency / license verification
**Inventory PASS / machine policy PASS — 2026-11-02.**

Exact locked inventory:
- source r18 Cargo.lock SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`;
- run `41422471078`: PASS;
- artifact `13130467223`;
- artifact ZIP SHA256 `afc0c0a75986fc68ea836a2cf7778353735dfdcb8bb24e5b99368e23ca351f7f`;
- package count 408;
- 406 crates.io + 2 local/path;
- 0 unknown external sources;
- 0 missing Cargo license expressions.

Machine policy:
- run `41423140592`: PASS;
- `cargo deny check licenses sources`: PASS;
- cargo-deny 0.20.2 via pinned action commit `3c6349835b2b7b196a839186cb8b78e02f7b5f25`;
- MPL-2.0 is exception-scoped to 5 exact reviewed crates, not globally allowed;
- unknown registry/git sources are denied;
- GPL/AGPL/LGPL identifiers are not in the global license allow-list.

Remaining release obligation: generate and read back distributable third-party license texts plus exact MPL source-availability notice for the release lock.

## Source/legal checks- UkraineAlarm public request page/offer checked without accepting/applying.
- KATOTTG uses cutoff/amendment-chain policy.
- alerts.in.ua remains aggregator/proxy candidate only.
- no live API credential used.

## Still open
- full transitive dependency/license gate before release;
- Android/iOS validation;
- live API access/security/latency/schema acceptance;
- push/background delivery;
- signing/installers/release/production.
