# Winit upstream review: 2026-11-09

Reviewed official `rust-windowing/winit` through `b31115a6eeb882ac2aaa886ac6371aa2cf642933`.
The review window is 2026-10-10 through 2026-11-09 by upstream commit date.
The common Git ancestor is `0a59e4b90565ab9d4638165b01749f887ac222ed`; Wingy already
contains many independently ported commits after that ancestor, so Git ancestry alone
does not identify missing fixes.

This batch preserves Wingy crate names, Rust 1.95, Android lifecycle/GameTextInput,
UIKit scenes/text input, and the existing public transfer errors. The removed scheduled
sync is separate from selective local review; `upstream-main` remains read-only.

| Upstream commit | Date | Change | Decision |
| --- | --- | --- | --- |
| [b31115a6](https://github.com/rust-windowing/winit/commit/b31115a6eeb882ac2aaa886ac6371aa2cf642933) | 2026-11-09 | winit-core: fix inverted RGBA size check for icons and cursors (#4826) | Adopt regression tests only: Wingy retains its correct modulo checks and is not affected by the upstream inversion. |
| [7ed7c203](https://github.com/rust-windowing/winit/commit/7ed7c2031c405e37aefd8635506933fdaa51b919) | 2026-11-03 | Fix commented out code in example | Correct the commented redraw example to use surface.window(). |
| [dbecbfba](https://github.com/rust-windowing/winit/commit/dbecbfba4fdadd906cde5d9bbed45ea78cb52b7d) | 2026-11-03 | Fix doc link | Not applicable: concerns FetchError, which Wingy does not expose. |
| [7c4f1bd4](https://github.com/rust-windowing/winit/commit/7c4f1bd43a5da037067cf8d7b033199de39e70af) | 2026-11-03 | Set default docs.rs targets | Set the supported Linux default target explicitly for docs.rs. |
| [e1ad70bf](https://github.com/rust-windowing/winit/commit/e1ad70bfed1936fb615d602ade72a617cb9bff4a) | 2026-11-02 | win32: fetch clipboard data in fn clipboard | Port eager Windows clipboard snapshots while retaining Wingy TransferError and public interfaces. |
| [c0ef7d25](https://github.com/rust-windowing/winit/commit/c0ef7d2558713eb6827a881fac052b721cf50c44) | 2026-11-02 | Fix for AppKit | Not applicable: Wingy Pasteboards::get returns Pasteboard directly and window_id is already copied separately. |
| [8b1b4764](https://github.com/rust-windowing/winit/commit/8b1b4764a6d7909dfa6c3d8321428ba9bfd1cd8d) | 2026-11-02 | Remove variants TransferError::{Failed, UnknownTransfer} | Defer public TransferError variant removals to an explicit API migration. |
| [ea2bc6a5](https://github.com/rust-windowing/winit/commit/ea2bc6a52f239c97da4bcfce719eab44674ce02c) | 2026-11-02 | Add struct FetchError | Defer new public FetchError API and cross-backend signature changes. |
| [e4b926fc](https://github.com/rust-windowing/winit/commit/e4b926fc3f9091119054fcd8a8a30ba2233bb837) | 2026-11-02 | Add error type UnknownTransfer | Defer new public UnknownTransfer error wrapper and cross-backend signature changes. |
| [1c912641](https://github.com/rust-windowing/winit/commit/1c91264133c8e4987a4d1c7a9d6739d56a778518) | 2026-11-01 | Allow multiple dep versions | Retain Wingy dependency policy; no need to relax cargo-deny bans for these fixes. |
| [b38e539a](https://github.com/rust-windowing/winit/commit/b38e539a60f9d3b03e54835c2f1960b540f814f1) | 2026-11-01 | Use unwrap_or_default | Style-only unwrap_or_default rewrite; no behavior fix needed. |
| [de58d95b](https://github.com/rust-windowing/winit/commit/de58d95b7f570804fb60d7ff9b19d8a7b543817f) | 2026-11-01 | Use as_chunks | Style-only slice as_chunks rewrite; no behavior fix needed. |
| [6f95b6a1](https://github.com/rust-windowing/winit/commit/6f95b6a1ea19c98de1228df161a6b3c96a4b8301) | 2026-11-01 | Use if-let-chains | Style-only if-let-chain rewrite; no behavior fix needed. |
| [c896b0a8](https://github.com/rust-windowing/winit/commit/c896b0a829a89d51899404f2f349a27cb2525a11) | 2026-11-01 | Use is_multiple_of | Do not port: this style rewrite introduced the RGBA validation inversion fixed by b31115a6. |
| [6c2596bb](https://github.com/rust-windowing/winit/commit/6c2596bb9f50763954b8e8b0b8e685493a11186d) | 2026-11-01 | Bump MSRV to 1.88 | Not applicable: Wingy already requires Rust 1.95. |
| [db098642](https://github.com/rust-windowing/winit/commit/db098642637c7e073327c298831678f46a1bb5ed) | 2026-10-30 | winit-wayland: fix some text not being copied due to missing mime types | Port Wayland/Xwayland MIME aliases and plaintext newline normalization, preserving legacy spaced MIME hints. |
| [ab2fe0cd](https://github.com/rust-windowing/winit/commit/ab2fe0cd12ff83fc287d9448ef1cdc4074e8bfeb) | 2026-10-30 | macOS: Handle Character Viewer text input | Already ported in Wingy `b8060874`; preserve existing implementation. |
| [0c737280](https://github.com/rust-windowing/winit/commit/0c7372808c59c0d46ae777959e65e773381dc9e9) | 2026-10-24 | X11: fix 32 bit build (no AtomicI64 on ppc) | Already ported in Wingy `bb6b7351`; preserve existing implementation. |
| [09f3ea32](https://github.com/rust-windowing/winit/commit/09f3ea32a0f208795477dcaab9debb1e41f2dbd6) | 2026-10-24 | Bump version on master | Retain Wingy independent package versions. |
| [4cce1fbe](https://github.com/rust-windowing/winit/commit/4cce1fbee8451b41d428c76ef0605b52d58ed2d7) | 2026-10-24 | win32: fix the AND mask of icons created from RGBA | Already ported in Wingy `534aa401`; preserve existing implementation. |
| [d559b056](https://github.com/rust-windowing/winit/commit/d559b0567d85404bc8d949089aa8e60ed46b5dcf) | 2026-10-21 | winit-win32: fix UTF-16 IME cursor offsets around surrogate pairs | Already ported in Wingy `52985f20`; preserve existing implementation. |
| [84fd4f0c](https://github.com/rust-windowing/winit/commit/84fd4f0c921d192da87b343d10e584f510e935e0) | 2026-10-21 | examples: add clipboard example | Already ported in Wingy `d8e18a12`; preserve existing implementation. |
| [808560e4](https://github.com/rust-windowing/winit/commit/808560e43b3222b7ae4b8ce91a1599f1aa6059bf) | 2026-10-21 | primary selection | Already ported in Wingy `ff328693`; preserve existing implementation. |
| [9817a204](https://github.com/rust-windowing/winit/commit/9817a204aa0ea7b49bdb836ea5a3085d3461b819) | 2026-10-21 | x11: implement clipboard support via the data transfer API | Already ported in Wingy `ae0717db`; preserve existing implementation. |
| [d91d1a5e](https://github.com/rust-windowing/winit/commit/d91d1a5e75f818b066b8274299c5d7b62716741a) | 2026-10-21 | win32: implement clipboard support via the data transfer API | Already ported in Wingy `520eb61f`; preserve existing implementation. |
| [7be6fb6b](https://github.com/rust-windowing/winit/commit/7be6fb6b084022f832b3ae45c475b6cf56838281) | 2026-10-21 | appkit: implement clipboard support via the data transfer API | Already ported in Wingy `e250e65b`; preserve existing implementation. |
| [d6d8aae0](https://github.com/rust-windowing/winit/commit/d6d8aae086ea9ce47260127e3ee24024ae6696f7) | 2026-10-21 | appkit: fix data transfer file-reference resolution | Already ported in Wingy `e965c661`; preserve existing implementation. |
| [5c04e52c](https://github.com/rust-windowing/winit/commit/5c04e52c464d0ef762c165cd9c86a354a2ad2802) | 2026-10-21 | wayland: destroy icon surface when drag ends | Already ported in Wingy `c4c67b88`; preserve existing implementation. |
| [30d64e8e](https://github.com/rust-windowing/winit/commit/30d64e8e7a56cb1cb0c39309bd321bdb134cb6b1) | 2026-10-21 | wayland: implement clipboard support via the data transfer API | Already ported in Wingy `33a07b4a`; preserve existing implementation. |
| [0f283312](https://github.com/rust-windowing/winit/commit/0f2833121491e5ca84a24c9db1229509e13de79d) | 2026-10-21 | wayland: rename dnd module to data_transfer | Already ported in Wingy `3e5c8c31`; preserve existing implementation. |
| [aa2f4069](https://github.com/rust-windowing/winit/commit/aa2f4069f06317dfbc589f0c840a903c92ea5574) | 2026-10-21 | wayland: rework dnd receive into drag sessions | Already ported in Wingy `3b32d2d4`; preserve existing implementation. |
| [776e0bf4](https://github.com/rust-windowing/winit/commit/776e0bf4471ecafda3866090a44735b64d5418e7) | 2026-10-21 | wayland: mark dispatched events when a data transfer fetch completes | Already ported in Wingy `5a194791`; preserve existing implementation. |
| [d1784409](https://github.com/rust-windowing/winit/commit/d1784409a2c1f04f7fbd79b2a3e33b33b0a77b51) | 2026-10-21 | add clipboard methods to ActiveEventLoop | Already ported in Wingy `7a0c59d8`; preserve existing implementation. |
| [18935657](https://github.com/rust-windowing/winit/commit/189356571618d7c38680ec4a1168a81ac838ffb0) | 2026-10-20 | Add `ScrollSource` to `WindowEvent::MouseWheel` | Already ported in Wingy `5e57243a`; preserve existing implementation. |
| [a2e81881](https://github.com/rust-windowing/winit/commit/a2e81881cd9981234af335c010a7cd69b46effde) | 2026-10-20 | macOS: discard marked text when IME is disabled | Already ported in Wingy `963b8111`; preserve existing implementation. |
| [501edec2](https://github.com/rust-windowing/winit/commit/501edec2dca951bed4fa0d5d9d2d228a7f390494) | 2026-10-20 | x11: Update `x11rb` to 0.14.0 | Already ported in Wingy `12c626bb`; preserve existing implementation. |
| [943fcf0a](https://github.com/rust-windowing/winit/commit/943fcf0a162f0ae99d832e5ccc49a4c5a8661639) | 2026-10-20 | No longer send an empty `Ime::Preedit` before `Ime::Commit` | Already ported in Wingy `b40d6095`; preserve existing implementation. |
| [a625a1fb](https://github.com/rust-windowing/winit/commit/a625a1fb0f80c0e1f214639e82ea8f0ba0ea3226) | 2026-10-19 | Fix doc links, rustdoc lints, and nested ref test | Already ported in Wingy `b0f11f04`; preserve existing implementation. |
| [758a50e3](https://github.com/rust-windowing/winit/commit/758a50e35a928dbf72bf05ae91f12fee7a540a83) | 2026-10-16 | macOS: forward mouse events to input context | Already ported in Wingy `82adc48d`; preserve existing implementation. |
| [250cd01a](https://github.com/rust-windowing/winit/commit/250cd01ae8d83a8969265a14496fb6957b44a34e) | 2026-10-15 | wayland: ignore queued updates for destroyed windows (#4760) | Already ported in Wingy `0d0d0625`; preserve existing implementation. |
| [f0a83481](https://github.com/rust-windowing/winit/commit/f0a83481c233c714d061356305f89bace0106f39) | 2026-10-12 | wayland: only destroy the text input when the seat is removed | Already ported in Wingy `5596a504`; preserve existing implementation. |


Validation includes RGBA valid/partial pixel tests, Windows clipboard eager snapshot/
cache/failure tests, and Wayland MIME alias/legacy-spacing/newline/charset tests.
Linux CI now executes the Wayland unit tests in addition to compiling the backend.
The complete platform CI covers Windows, Linux, Apple, Android, and Web/Redox.
Actual native clipboard interoperability still requires OS-session acceptance; the
unit tests validate conversion and state behavior without modifying the system clipboard.
