#![allow(dead_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core2::io::{Read, Seek};

use self::archive::{adapter, Archive, ArchiveInner, Entries};
use self::error::CheckErr;

pub struct ZArchive<A: archive::Archive + ?Sized> {
    inner: A::Inner,
}

impl<A: Archive + ?Sized> ZArchive<A> {
    pub fn new(archive: A) -> Result<Self, CheckErr>
    where
        A: Sized,
    {
        let inner = archive.to_archive()?;
        ArchiveInner::check(&inner)?;
        Ok(Self { inner })
    }

    pub fn new_reader<R: Read + Seek>(
        reader: &mut R,
    ) -> Result<ZArchive<adapter::Reader<'_>>, CheckErr> {
        ZArchive::new(adapter::Reader::new(reader))
    }

    pub fn into_inner(self) -> A
    where
        A: Sized,
    {
        A::from_inner(self.inner)
    }

    pub fn entries(&self) -> Entries<'_> {
        Entries::new(&self.inner)
    }
}

pub mod archive;
pub mod error;

mod raw {
    pub mod big_endian;
    pub mod entry;
    pub mod footer;

    const _: () = {
        use core::mem::size_of;

        assert!(size_of::<entry::ArchiveEntry>() == 16);
        assert!(size_of::<footer::Footer>() == 16 * 6 + 32 + 8 + 4 + 4);
        assert!(size_of::<footer::CompressionOffsetRecord>() == 8 + 16 * 2);
    };
}

#[cfg(feature = "std")]
pub use self::std::*;

#[cfg(feature = "std")]
mod std {

    // pub fn open_file(path: &Path) -> io::Result<ZArchive<Mmap>> {
    //     let mmap = unsafe { Mmap::map(&File::open(path)?)? };
    //     let archive = ZArchive::new(mmap);
    //     archive.check()?;

    //     Ok(archive)
    // }

    // #[cfg(all(feature = "memmap", feature = "zerocopy"))]
    // pub fn open_file_mut(path: &Path) ->
    // io::Result<zerocopy::Archive<MmapMut>> {     let mmap = unsafe {
    // MmapMut(&File::open(path)?)? };     let archive =
    // zerocopy::Archive::new(mmap);     archive.check()?;

    //     Ok(archive)
    // }

    // #[cfg(feature = "universal")]
    // pub fn open_reader(
    //     reader: impl Read + Seek,
    // ) -> io::Result<universal::Archive<impl Read + Seek>> { todo!()
    // }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
