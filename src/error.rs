use displaydoc::Display;
use thiserror_core2::Error;

#[derive(Debug, Display, Error)]
/// Error while [checking the archive](crate::archive::Archive::check).
pub enum CheckErr {
    /// Read error: {0}
    Io(#[from] core2::io::Error),
    /// Invalid archive: {0}
    Invalid(#[from] Invalid),
}

#[derive(Debug, Display, Error)]
/// The archive is not valid.
pub enum Invalid {
    /// The archive is shorter than the footer (len: {0}, min: 144).
    TooShort(u64),
}
