use displaydoc::Display;
use thiserror_core2::Error;

use crate::raw::footer::Footer;

#[derive(Debug, Error)]
/// Error while [checking the archive](crate::archive::ArchiveInner::check).
pub enum CheckErr {
    #[error("read error: {0}")]
    Io(#[from] core2::io::Error),
    #[error("invalid archive: {0}")]
    Invalid(#[from] Invalid),
}

#[derive(Debug, Error)]
/// The archive is not valid.
pub enum Invalid {
    /// The archive is shorter than the footer.
    #[error("archive too short: {0} < {}", Footer::LEN)]
    TooShort(u64),
    /// The magic number is invalid.
    #[error("expected magic {:#X}, found {0:#X}", Footer::MAGIC)]
    Magic(u32),
    /// Unexpected version.
    #[error("expected version {:#X}, found {0:#X}", Footer::VERSION)]
    Version(u32),
}
