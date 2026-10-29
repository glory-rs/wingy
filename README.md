# Wingy

A cross-platform window and event-loop library, with the maintained Android Activity recreation and Android/UIKit IME implementations used by Glory.

Wingy is maintained by Glory and is based on [Winit](https://github.com/rust-windowing/winit).
Wingy 0.31.0 retains the existing maintenance patches from the Winit 0.31.0-beta.4 baseline. It also corrects packaging links from the initial beta dependency uploads.
Original authors, licenses and copyright notices are retained. Upstream projects using the
original library are not represented as users of this renamed fork.

[![CI](https://github.com/glory-rs/wingy/actions/workflows/ci.yml/badge.svg)](https://github.com/glory-rs/wingy/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/wingy.svg)](https://crates.io/crates/wingy)
[![docs.rs](https://img.shields.io/docsrs/wingy)](https://docs.rs/wingy)

## Dependency

```toml
[dependencies]
wingy = "=0.31.0"
```

Platform packages use the `wingy-*` namespace. The shared geometry package is `wingy-dpi`; applications can keep using the `wingy::dpi` re-export.

## Branches and releases

- `main` contains this project's maintained implementation, namespace and release tooling.
- `upstream-main` mirrors the upstream default branch exactly. Its scheduled/manual sync never updates `main`.
- Upstream changes are reviewed before integration into maintained `main`.
- Release tags use `wingy-v0.31.0`, preserving inherited upstream tags.

See [UPSTREAM.md](UPSTREAM.md) for provenance and [RELEASING.md](RELEASING.md) for
the explicit dependency order, checks and publication procedure. Supported platform
checks are compilation/test evidence and do not imply unperformed device or OS IME acceptance.
