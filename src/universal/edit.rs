use core::cell::RefMut;

use core2::io::{Read, Seek, Write};

use super::*;

pub struct EditArchive<'a, W: Write + ?Sized + 'a> {
    inner: RefMut<'a, W>,
}

impl<'a, W: Read + Seek + Write + 'a> EditArchive<'a, W> {
    pub(crate) fn new(archive: &'a mut Archive<W>) -> Self {
        Self { inner: archive.inner.borrow_mut() }
    }
}
