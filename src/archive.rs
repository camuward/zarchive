use core2::io::{self, Read, Seek, Write};

use self::footer::Footer;

mod big_endian;
mod entry;
mod footer;

#[cfg(feature = "std")]
mod file;

pub const COMPRESSED_BLOCK_SIZE: usize = 64 * 1024; // 64 KiB

pub trait Archive {}

trait SeekRead: Read + Seek {}
impl<R: Read + Seek> SeekRead for R {}

struct ReadArchive(dyn SeekRead);
impl ReadArchive {
    fn footer(&self) -> io::Result<Footer> {
        unimplemented!()
    }
}

struct MemArchive<'a>(&'a [u8]);
impl MemArchive<'_> {
    fn footer(&self) -> &Footer {
        unimplemented!()
    }
}

struct WriteArchive(dyn Write);

const _: () = {
    use core::mem::size_of;

    assert!(size_of::<entry::ArchiveEntry>() == 16);
    assert!(size_of::<footer::Footer>() == 16 * 6 + 32 + 8 + 4 + 4);
    assert!(size_of::<footer::CompressionOffsetRecord>() == 8 + 16 * 2);
};
