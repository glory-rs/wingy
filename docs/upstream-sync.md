# Reviewing upstream changes

Wingy is an independently maintained windowing library. Its renamed crates,
Android Activity lifecycle, GameTextInput integration, UIKit scenes and text
input, and stable transfer interfaces must be preserved when importing fixes.

The renamed repository does not run a scheduled mirror push. Treat its existing
`upstream-main` reference as read-only; do not update it when reviewing or porting
changes. The planned original-name `glory-rs/winit` fork will represent the
relationship with `rust-windowing/winit` separately.

Read the official repository or fetch an upstream revision locally for review,
then adapt selected fixes on a Wingy development branch. Link the original
commit in the review record, explain API differences, and validate the affected
platforms. Do not replace Wingy code with upstream wholesale or copy upstream
version bumps, public API changes, and workflows without an explicit need.

The former mirror deploy key and Actions secret are not used by any Wingy
workflow after removal of the scheduled sync. Their retirement belongs to the
separate repository migration plan; this source change does not remove or rotate
credentials.
