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

## Source/legal checks
- UkraineAlarm public request page/offer checked without accepting/applying.
- KATOTTG uses cutoff/amendment-chain policy.
- alerts.in.ua remains aggregator/proxy candidate only.
- no live API credential used.

## Still open
- formal clean-session continuity trial (C13);
- full transitive dependency/license gate before release;
- Android/iOS validation;
- live API access/security/latency/schema acceptance;
- push/background delivery;
- signing/installers/release/production.
