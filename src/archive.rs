use core::marker::PhantomData;
use core::ptr::NonNull;

use crate::error::CheckErr;
use crate::raw::footer::Footer;

pub const COMPRESSED_BLOCK_SIZE: usize = 64 * 1024; // 64 KiB

pub trait Archive {
    type Inner: ArchiveInner;

    fn to_archive(self) -> Result<Self::Inner, CheckErr>;
    fn from_inner(inner: Self::Inner) -> Self;
}

pub trait ArchiveInner {
    fn check(&self) -> Result<(), CheckErr>;
}

impl<I: ArchiveInner> Archive for I {
    type Inner = I;

    fn to_archive(self) -> Result<Self::Inner, CheckErr> {
        self.check()?;
        Ok(self)
    }

    fn from_inner(inner: Self::Inner) -> Self {
        inner
    }
}

/// An iterator over the entries of an archive.
pub struct Entries<'a> {
    _p: PhantomData<&'a ()>,
    archive: NonNull<()>, // type-erased ptr to ArchiveInner
    check_fn: unsafe fn(NonNull<()>) -> Result<(), CheckErr>,

    index: usize,
}

impl Entries<'_> {
    pub(crate) fn new<A: ArchiveInner>(archive: &A) -> Self {
        Self {
            _p: PhantomData,
            archive: NonNull::from(archive).cast(),
            check_fn: |inner| A::check(unsafe { inner.cast::<A>().as_ref() }),

            index: 0,
        }
    }
}

impl<'a> Iterator for Entries<'a> {
    type Item = (); // TODO

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub(crate) struct Cache<A> {
    pub footer: Footer,
    pub inner: A,
}

pub mod adapter;

#[cfg(feature = "std")]
mod file;
