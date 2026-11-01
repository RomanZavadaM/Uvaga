# REPOSITORY_GOVERNANCE — GitHub settings and merge discipline

**Snapshot checked:** 2026-11-01.

## Factually observed GitHub settings
- Repository: public `RomanZavadaM/Uvaga`.
- Default branch: `main`.
- GitHub rulesets API returned an empty list: no repository ruleset is currently visible.
- Branch-protection endpoint could not be read by the installed GitHub integration (403: resource not accessible by integration), therefore branch protection status is **unknown**, not claimed.
- Repository currently allows `squash`, `merge commit` and `rebase` merge methods.
- Auto-delete merged branches: disabled.
- Auto-merge: disabled.
- Update-branch button: disabled.
- Issues enabled; wiki enabled; discussions/pages disabled.

## Project merge policy
Until GitHub settings are technically enforced:
1. No direct product/code writes to `main`; bootstrap README was the one initialization exception.
2. All substantive work goes through a PR.
3. A PR is merged only after applicable CI is green and project acceptance/review requirements are satisfied.
4. **Preferred integration method: squash merge** for small verified slices; do not mix merge strategies in the initial development line without a documented owner decision.
5. Stacked PRs are integrated base-first; child PR is retargeted to `main` and must receive fresh CI after its base lands.
6. Do not reuse an issued/reviewed revision identity for different content.
7. Delete obsolete merged work branches after recovery/checkpoint needs are satisfied; never delete unique work before confirming recovery.

## Desired technical enforcement before first stable release
Recommended repository configuration:
- protect or ruleset `main`;
- require PR before merge;
- require applicable status checks;
- block force-push and branch deletion on `main`;
- prefer squash-only if owner keeps the above project policy;
- enable delete-branch-on-merge after verification;
- keep secrets outside repository and public Actions logs.

The current connector can read the repository/rulesets but cannot write these administration settings. Therefore this document is policy + tracked configuration debt, not proof that GitHub enforcement exists.

## Evidence
Review gate: GitHub Issue #9. Current state: PROJECT_STATE.md / VERIFICATION.md.
