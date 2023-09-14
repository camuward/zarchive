use super::*;

pub struct EditArchive<'a, W: ?Sized + 'a> {
    inner: &'a mut W,
}

impl<'a, W: AsRef<[u8]> + Read + Seek + Write + 'a> EditArchive<'a, W> {
    pub(crate) fn new(archive: &'a mut Archive<W>) -> Self {
        Self { inner: &mut archive.buf }
    }
}
