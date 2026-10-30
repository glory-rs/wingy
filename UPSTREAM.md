# Upstream provenance

Upstream: https://github.com/rust-windowing/winit

Maintenance repository: https://github.com/glory-rs/wingy

Wingy 0.31.0 is based on upstream 0.31.0-beta.4 and retains all previous Glory maintenance fixes. The release version was explicitly selected by the maintainer after correcting archive links in the initial beta dependency uploads. Original licensing and copyright notices remain in the repository. Names in preserved upstream changelogs describe the historical upstream releases.

Only `upstream-main` is synchronized automatically. Never reset or auto-merge the maintained `main` branch to upstream.

Backported upstream master fixes `0c737280` (32-bit X11 data-transfer IDs) and `ab2fe0cd` (macOS Character Viewer IME commits) on 2026-10-30. Android lifecycle, bounded GameTextInput and UIKit composition maintenance remain in place.
