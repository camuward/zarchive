use core2::io::{self, ErrorKind, Read, Seek, Write};
use thiserror_core2::Error;

pub use self::edit::*;
pub use self::entry::*;
pub use self::node::*;
pub use self::order::*;
pub use self::record::*;

mod edit;
mod entry;
mod node;
mod order;
mod record;

pub const COMPRESSED_BLOCK_SIZE: usize = 64 * 1024; // 64 KiB
pub const ENTRIES_PER_OFFSETRECORD: usize = 16; // must be aligned to 2

#[repr(transparent)]
pub struct Archive<T: AsRef<[u8]> + ?Sized> {
    buf: T,
}

impl<T: AsRef<[u8]>> Archive<T> {
    pub fn new(buf: T) -> Self {
        Self { buf }
    }

    #[inline(always)]
    pub fn check(&self) -> Result<(), InvalidError> {
        fn inner(_buf: &[u8]) -> Result<(), InvalidError> {
            Ok(())
        }

        inner(self.buf.as_ref())
    }
}

impl<T: AsRef<[u8]> + Read + Seek + Write> Archive<T> {
    pub fn edit(&mut self) -> EditArchive<T> {
        EditArchive::new(self)
    }
}

#[derive(Debug, Error)]
pub enum InvalidError {}

impl From<InvalidError> for io::Error {
    fn from(e: InvalidError) -> Self {
        io::Error::new(ErrorKind::InvalidData, e)
    }
}

const _: () = {
    use core::mem::size_of;

    assert!(size_of::<ArchiveEntry>() == 16);
    assert!(size_of::<Footer>() == 16 * 6 + 32 + 8 + 4 + 4);
    assert!(size_of::<CompressionOffsetRecord>() == 8 + 16 * 2);
};
