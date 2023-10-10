#[cfg(target_os = "windows")]
pub use self::win::*;
#[cfg(target_os = "windows")]
mod win {}

#[cfg(target_os = "linux")]
pub use self::nix::*;
#[cfg(target_os = "linux")]
mod nix {}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub use self::other::*;
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
mod other {}
