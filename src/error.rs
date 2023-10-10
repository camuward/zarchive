use core2::io::{self, ErrorKind, Write};
use displaydoc::Display;
use thiserror_core2::Error;

#[derive(Debug, Display, Error)]
/// The archive is not valid.
pub enum Invalid {
    /// The archive is shorter than the footer (len: {0}, min: 144).
    TooShort(u64),
}

impl From<Invalid> for io::Error {
    #[cfg(feature = "std")]
    fn from(error: Invalid) -> Self {
        io::Error::new(ErrorKind::InvalidData, error)
    }

    #[cfg(not(feature = "std"))]
    fn from(_: Invalid) -> Self {
        io::Error::new(ErrorKind::InvalidData, "invalid archive")
    }
}
