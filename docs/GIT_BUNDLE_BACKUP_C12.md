# Git bundle backup + restore checkpoint — C12

## Purpose
Prove that the project can be restored from a real Git-history backup artifact rather than from a text mirror or chat memory.

## Backup scope
The workflow creates a Git bundle containing all refs visible to the runner after fetching all branches and tags. Before bundling it also creates an ephemeral local ref `c12-main` pointing to the exact current `origin/main` SHA so restore can target a deterministic ref.

Artifact contents:
- `Uvaga-all-refs.bundle` — full Git bundle;
- `SHA256SUMS.txt`;
- `MANIFEST.txt` with current main SHA, bundle SHA256, workflow SHA and event type.

## Restore test
In a clean directory the workflow:
1. initializes a new Git repository;
2. fetches `refs/heads/c12-main` from the bundle into local `main`;
3. checks out restored `main`;
4. compares restored HEAD byte-for-byte by SHA to the source `origin/main`;
5. runs `git fsck --full`;
6. checks required continuity/source files exist;
7. runs the restored root Rust test suite.

The restore uses only the generated bundle for Git object/history recovery.

## Trigger model
- Pull request: proves the workflow logic before merge.
- Push to `main` that introduces/changes this checkpoint: produces the accepted post-merge backup artifact.
- Manual dispatch remains available for future periodic checkpoints.

## Acceptance
C12 backup/restore may be marked PASS for this checkpoint only when:
- the post-merge `main` workflow run succeeds;
- bundle verification succeeds;
- clean restore reproduces the exact source main SHA;
- `git fsck --full` succeeds;
- restored Rust tests pass;
- the generated artifact is downloaded/read back or preserved in Drive BACKUPS with its checksum;
- the exact run, main SHA and artifact SHA are recorded in VERIFICATION.

## Limits
- This backs up Git repository history/refs, not production databases, external secrets, provider credentials or user data.
- Drive remains a backup destination, not source of truth.
- GitHub remains authoritative for active development.
