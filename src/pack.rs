use core2::io::{Cursor, Read, Seek, Write};

pub use self::entry::*;
pub use self::path::*;
pub use self::path_nodes::*;

mod entry;
mod path;
mod path_nodes;

pub struct DirEntry {}

/// A top-level representation of a ZArchive.
pub struct ArchiveReader<R: Read + ?Sized> {
    pos: u64,
    inner: R,
}

impl<R: Read> ArchiveReader<R> {
    /// Creates a new `ArchiveReader` from a [`Read`]er.
    pub fn new(inner: R) -> Self {
        Self { inner, pos: 0 }
    }

    /// Returns the underlying reader.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<'a> ArchiveReader<Cursor<&'a [u8]>> {
    /// Creates a new `ArchiveReader` from a byte slice.
    pub fn from_bytes(slice: &'a [u8]) -> Self {
        Self::new(Cursor::new(slice))
    }
}

impl<R: Read + Seek> ArchiveReader<R> {}

impl<W: Write + Read + Seek> ArchiveReader<W> {}

impl<R: Read + Seek + Clone> Clone for ArchiveReader<R> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone(), pos: self.pos }
    }
}

#[cfg(feature = "std")]
pub use self::std::*;

#[cfg(feature = "std")]
mod std {
    use std::fs::File;
    use std::io;
    use std::path::Path;

    use super::ArchiveReader;

    impl ArchiveReader<File> {
        #[inline]
        pub fn open_file(path: impl AsRef<Path>) -> io::Result<Self> {
            Ok(Self::new(File::open(path)?))
        }
    }
}
