#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;

#[cfg(target_os = "linux")]
mod fallback;
#[cfg(target_os = "linux")]
pub use fallback::*;

#[cfg(target_os = "macos")]
mod macos;
pub use macos::*;
