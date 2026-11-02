# Dependency / license machine policy — predeclared acceptance

**Date:** 2026-11-02  
**Basis:** exact r18 locked-graph inventory PASS from run `41422471078`.  
**Product behavior:** no change.

## Goal

Turn the reviewed r18 dependency inventory into a machine-enforced license/source gate without broadening approved license scope silently.

## Policy

Globally allowed license identifiers are limited to the permissive/non-copyleft families observed in the exact r18 graph:

- `0BSD`
- `Apache-2.0`
- `Apache-2.0 WITH LLVM-exception`
- `BSD-3-Clause`
- `CC0-1.0`
- `ISC`
- `MIT`
- `MIT-0`
- `Unicode-3.0`
- `Unlicense`
- `Zlib`

`MPL-2.0` is **not** globally allowed. It is accepted only for the five already reviewed exact package versions:

- `cssparser@0.37.0`
- `cssparser-macros@0.7.1`
- `dtoa-short@0.3.5`
- `option-ext@0.2.0`
- `selectors@0.38.0`

Private unpublished Uvaga workspace crates are excluded from third-party license checking.

Only crates.io registry dependencies are accepted. Unknown registries and git dependencies are denied.

## Acceptance criteria

1. The check uses the verified r18 `Cargo.lock` SHA256 `6073fb5d12e3202ed3fdc2e9bf1cbb854b9a7efcfc45063ad5aeba48291b4b7d`.
2. cargo-deny version is fixed by the pinned `cargo-deny-action` commit corresponding to cargo-deny 0.20.2.
3. `cargo deny check licenses sources` passes for the exact r18 graph.
4. No GPL, AGPL or LGPL identifier is added to the global allow-list.
5. MPL-2.0 remains package/version-specific rather than globally allowed.
6. Unknown git or registry sources fail the source policy.
7. The workflow changes no runtime/product source.
8. A PASS closes the machine license/source-policy sub-gate only; distributable third-party license text/source-availability packaging remains a later release gate.

## Safety

No live alert source, credentials, public notifications, production deployment, signing or release are introduced.
