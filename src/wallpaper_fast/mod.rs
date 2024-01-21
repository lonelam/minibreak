#[cfg(windows)]
mod windows;
pub use self::windows::*;

#[cfg(not(windows))]
pub use fallback::*;
