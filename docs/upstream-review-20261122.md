# Upstream review, 2026-11-22

Reviewed the 3 commits added to [`rust-windowing/winit:master`](https://github.com/rust-windowing/winit/compare/b31115a6eeb882ac2aaa886ac6371aa2cf642933...6853ca5e524adb22ef91f283d5f464411dc9d585) since the previous original-name fork tip, `b31115a6e`, through `6853ca5e5`.

The two Wayland changes are applied together. The popup example compiles on Linux with only the Wayland backend, and all four existing Wayland library tests pass. Those tests exercise data transfer. Interactive compositor behavior has separate coverage needs.

| Commit | Upstream change | Decision for Wingy |
| --- | --- | --- |
| [`6d0e5419f`](https://github.com/rust-windowing/winit/commit/6d0e5419ff7f6ee2d4c2eab35cd6366b996c1ac0) | wayland: share window setup, handle the parent window (#4819) | Port the shared Wayland window setup, parent validation and destruction ordering. Keep the local transfer and non-Wayland implementations. Include the example and behavior notes. |
| [`f0abec90a`](https://github.com/rust-windowing/winit/commit/f0abec90a3f9916818b70cde12fea40b797309ae) | Fix closing and opening a popup in the same eventloop run (#4834) | Port event dispatch ordering on top of the shared window setup so closing a popup and creating another in the same event loop turn works. |
| [`6853ca5e5`](https://github.com/rust-windowing/winit/commit/6853ca5e524adb22ef91f283d5f464411dc9d585) | ci: drop the Windows 32bit GNU job | No change. Wingy CI has no 32-bit GNU host-toolchain job. |
