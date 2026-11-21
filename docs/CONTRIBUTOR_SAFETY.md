# Contributor safety verification

## Keyboard and async status coverage (#504)

Run `cargo test --manifest-path scripts/contributor-safety/Cargo.toml`.
This isolated crate imports the production accessibility, storage and error
modules directly, avoiding unrelated workspace compilation defects. It checks
all 49 async transition pairs, keyboard modes, modal focus targets, broadcasting
dismissal, reduced-motion announcements, invalid-input recovery and actual
Soroban signature rejection. It does not claim browser or screen-reader testing:
this repository supplies metadata; consumers must implement Tab/Shift+Tab focus
containment, Escape handling, and restore focus to an available DOM element.

Retry recoverable failures through Validating; reset terminal states to Idle
before starting again. After an ambiguous broadcast, query transaction settlement
before retrying to avoid duplicate financial operations. Invalid audit input or
missing authorization leaves the previous audit record intact. Audit writers
require their own signature, not an admin role; the record is advisory and must
never authorize a transaction. No deployed ABI, event or storage layout changes
are needed. Rollback is reverting these verification-only additions.

## Local diagnostics (#505)

Run `node scripts/contributor-diagnostics.cjs --json` from any directory, or
`node scripts/contributor-diagnostics.cjs --self-test` for deterministic tests.
Node 18+ is required. The existing JSON health/count/result fields and exit codes
are preserved: 0 healthy, 1 failed; unknown options now return 2. `--strict`
remains accepted (all prerequisite failures are blocking, so there are no warning
overrides). Checks are read-only and offline, with five-second subprocess limits.
No wallet, signing key, network permission or admin access is required. Permission
denial is reported without echoing sensitive subprocess/file error details.

Install missing prerequisites or restore denied read/execute permissions as the
current user, then rerun the same command. The command never elevates privileges,
installs dependencies, alters files, or repairs configuration automatically.
Offline Cargo metadata is only manifest resolution, not a successful build;
an empty dependency cache may need a separate authorized Cargo fetch. The
toolchain manifest remains the authority for the WASM target. No contract
migration is involved; revert the CLI change to restore the previous command.
