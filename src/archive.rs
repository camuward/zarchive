use core2::io::{self, Read, Seek, Write};

use crate::error::Invalid;

use self::{entry::ArchiveEntry, footer::Footer};

pub mod adapter;
mod big_endian;
mod entry;
mod footer;
mod node;

#[cfg(feature = "std")]
mod file;

pub const COMPRESSED_BLOCK_SIZE: usize = 64 * 1024; // 64 KiB

pub trait Archive {
    fn check(&self) -> Result<(), Invalid>;
}

trait SeekRead: Read + Seek {}
impl<R: Read + Seek> SeekRead for R {}

struct ReadArchive(dyn SeekRead);
impl ReadArchive {
    fn footer(&self) -> io::Result<Footer> {
        todo!()
    }
}

struct MemArchive<'a>(&'a [u8]);
impl MemArchive<'_> {
    fn footer(&self) -> zerocopy::Ref<&[u8], Footer> {
        zerocopy::Ref::new_unaligned_from_suffix(self.0).unwrap().1
    }
}

struct WriteArchive(dyn Write);

/// An iterator over the entries of an archive.
pub struct Entries<'a> {
    archive: &'a dyn Archive,
    footer: Footer,
}

impl Iterator for Entries<'_> {
    type Item = io::Result<ArchiveEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

const _: () = {
    use core::mem::size_of;

    assert!(size_of::<entry::ArchiveEntry>() == 16);
    assert!(size_of::<footer::Footer>() == 16 * 6 + 32 + 8 + 4 + 4);
    assert!(size_of::<footer::CompressionOffsetRecord>() == 8 + 16 * 2);
};
