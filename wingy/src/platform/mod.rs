//! # Platform implementations
//!
//! This module re-exports the platform-specific implementation crates that are used by default in
//! Wingy.
//!
//! Only the crates / modules corresponding to the platform you're compiling to will be available.
//!
//! | Platform | Crate | Module |
//! | -------- | ----- | ------ |
#![doc = concat!("| Android | [`wingy-android`](https://docs.rs/wingy-android/", env!("CARGO_PKG_VERSION"), "/) | `wingy::platform::android` |")]
#![doc = concat!("| macOS | [`wingy-appkit`](https://docs.rs/wingy-appkit/", env!("CARGO_PKG_VERSION"), "/) | `wingy::platform::macos` |")]
#![doc = concat!("| Redox | [`wingy-orbital`](https://docs.rs/wingy-orbital/", env!("CARGO_PKG_VERSION"), "/) | `wingy::platform::orbital` |")]
#![doc = concat!("| iOS/visionOS/tvOS/Mac Catalyst | [`wingy-uikit`](https://docs.rs/wingy-uikit/", env!("CARGO_PKG_VERSION"), "/) | `wingy::platform::ios` |")]
#![doc = concat!("| Wayland | [`wingy-wayland`](https://docs.rs/wingy-wayland/", env!("CARGO_PKG_VERSION"), "/) | `wingy::platform::wayland` |")]
#![doc = concat!("| Web | [`wingy-web`](https://docs.rs/wingy-web/", env!("CARGO_PKG_VERSION"), "/) | `wingy::platform::web` |")]
#![doc = concat!("| Windows | [`wingy-win32`](https://docs.rs/wingy-win32/", env!("CARGO_PKG_VERSION"), "/) | `wingy::platform::windows` |")]
#![doc = concat!("| X11 | [`wingy-x11`](https://docs.rs/wingy-x11/", env!("CARGO_PKG_VERSION"), "/) | `wingy::platform::x11` |")]
//! ## Common modules
//!
//! Furthermore, we provide two modules for common functionality:
//! - `scancode`, available on Windows, macOS, Wayland and X11.
//! - `startup_notify`, available on Wayland and X11.

#[cfg(android_platform)]
pub use wingy_android as android;
#[cfg(macos_platform)]
pub use wingy_appkit as macos;
#[cfg(orbital_platform)]
pub use wingy_orbital as orbital;
#[cfg(ios_platform)]
pub use wingy_uikit as ios;
#[cfg(wayland_platform)]
pub use wingy_wayland as wayland;
#[cfg(web_platform)]
pub use wingy_web as web;
#[cfg(windows_platform)]
pub use wingy_win32 as windows;
#[cfg(x11_platform)]
pub use wingy_x11 as x11;

#[cfg(any(windows_platform, macos_platform, x11_platform, wayland_platform, docsrs))]
pub mod scancode;
#[cfg(any(x11_platform, wayland_platform, docsrs))]
pub mod startup_notify;
