#![allow(dead_code, unused_imports)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use self::archive::Entries;

pub mod archive;
pub mod error;

pub struct ZArchive<A: archive::Archive + ?Sized> {
    inner: A,
}

impl<A: archive::Archive> ZArchive<A> {
    pub fn new(archive: A) -> Result<Self, error::Invalid> {
        // the inner archive must always be valid
        archive.check()?;
        Ok(Self { inner: archive })
    }

    pub fn into_inner(self) -> A {
        self.inner
    }
}

impl<A: archive::Archive + ?Sized> ZArchive<A> {
    pub fn entries(&self) -> Entries<'_> {
        todo!()
    }
}

#[cfg(feature = "std")]
pub use self::std::*;

#[cfg(feature = "std")]
mod std {
    use std::fs::File;
    use std::io::{self, Read, Seek, Write};
    use std::path::Path;

    use crate::ZArchive;

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
