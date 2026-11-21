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
