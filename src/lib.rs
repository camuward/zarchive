#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "universal")]
/// Generic archive manipulation using [`Read`] and [`Write`] traits
pub mod universal;

#[cfg(feature = "zerocopy")]
/// Zero-copy archive manipulation
pub mod zerocopy;

#[cfg(feature = "std")]
pub use self::std::*;

#[cfg(feature = "std")]
mod std {
    use std::fs::File;
    use std::io::{self, Read, Seek, Write};
    use std::path::Path;

    #[cfg(feature = "memmap")]
    use memmap2::{Mmap, MmapMut};

    #[cfg(feature = "universal")]
    use super::universal;

    #[cfg(feature = "zerocopy")]
    use super::zerocopy;

    #[cfg(all(feature = "memmap", feature = "zerocopy"))]
    pub fn open_file(path: &Path) -> io::Result<zerocopy::Archive<Mmap>> {
        let mmap = unsafe { Mmap::map(&File::open(path)?)? };
        let archive = zerocopy::Archive::new(mmap);
        archive.check()?;

        Ok(archive)
    }

    // #[cfg(all(feature = "memmap", feature = "zerocopy"))]
    // pub fn open_file_mut(path: &Path) -> io::Result<zerocopy::Archive<MmapMut>> {
    //     let mmap = unsafe { MmapMut(&File::open(path)?)? };
    //     let archive = zerocopy::Archive::new(mmap);
    //     archive.check()?;

    //     Ok(archive)
    // }

    // #[cfg(feature = "universal")]
    // pub fn open_reader(
    //     reader: impl Read + Seek,
    // ) -> io::Result<universal::Archive<impl Read + Seek>> {
    //     todo!()
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
