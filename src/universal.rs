use core::cell::RefCell;

use core2::io::{Cursor, Read, Seek, Write};

pub use self::edit::*;
pub use self::entry::*;
pub use self::path::*;
pub use self::path_nodes::*;

mod edit;
mod entry;
mod path;
mod path_nodes;

/// A top-level representation of a ZArchive.
pub struct Archive<R: Read + Seek + ?Sized> {
    inner: RefCell<R>,
}

impl<R: Read + Seek> Archive<R> {
    /// Creates a new `ArchiveReader` from a reader.
    pub fn new(reader: R) -> Self {
        Self { inner: RefCell::new(reader) }
    }
}

impl<'a> Archive<Cursor<&'a [u8]>> {
    /// Creates a new `ArchiveReader` from a byte slice.
    pub fn from_bytes(slice: &'a [u8]) -> Self {
        Self::new(Cursor::new(slice))
    }
}

impl<W: Write + Read + Seek> Archive<W> {
    pub fn edit(&mut self) -> EditArchive<W> {
        EditArchive::new(self)
    }
}

#[cfg(feature = "std")]
pub use self::std::*;

#[cfg(feature = "std")]
mod std {
    use std::fs::File;
    use std::io;
    use std::path::Path;

    use super::Archive;

    impl Archive<File> {
        #[inline]
        pub fn open_file(path: impl AsRef<Path>) -> io::Result<Self> {
            Ok(Self::new(File::open(path)?))
        }
    }
}
