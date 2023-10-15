use core::cell::RefCell;
use core2::io::{Read, Seek, SeekFrom};

use zerocopy::{AsBytes, FromZeroes};

use crate::error::{CheckErr, Invalid};
use crate::raw::footer::Footer;

use super::*;

trait SeekRead: Read + Seek {}
impl<R: Read + Seek> SeekRead for R {}

/// Generic adapter wrapping a `Read + Seek`er.
pub struct Reader<'a>(&'a mut dyn SeekRead);

impl<'a> Reader<'a> {
    pub fn new<R: Read + Seek>(reader: &'a mut R) -> Self {
        Self(reader)
    }
}

pub struct ReaderInner<'a>(Cache<RefCell<&'a mut dyn SeekRead>>);

impl<'a> Archive for Reader<'a> {
    type Inner = ReaderInner<'a>;

    fn to_archive(self) -> Result<Self::Inner, CheckErr> {
        Ok(ReaderInner(Cache {
            footer: get_footer_seek_read(self.0)?,
            inner: RefCell::new(self.0),
        }))
    }

    fn from_inner(inner: Self::Inner) -> Self {
        Self(RefCell::into_inner(inner.0.inner))
    }
}

impl ArchiveInner for ReaderInner<'_> {
    /// A cache is always valid.
    fn check(&self) -> Result<(), CheckErr> {
        Ok(())
    }
}

fn get_footer_seek_read(archive: &mut dyn SeekRead) -> Result<Footer, CheckErr> {
    let len = archive.seek(SeekFrom::End(0))?;
    let footer_off = len.checked_sub(Footer::LEN as u64).ok_or(Invalid::TooShort(len))?;

    // read the footer
    let mut footer = Footer::new_zeroed();
    let mut buf = footer.as_bytes_mut();
    archive.seek(SeekFrom::Start(footer_off))?;
    archive.read_exact(&mut buf)?;

    // footer.validate()?;

    Ok(footer)
}

impl ArchiveInner for &[u8] {
    fn check(&self) -> Result<(), CheckErr> {
        let _footer = get_footer_mem(self)?;
        // footer.validate()?;
        Ok(())
    }
}

fn get_footer_mem(archive: &[u8]) -> Result<zerocopy::Ref<&[u8], Footer>, CheckErr> {
    match zerocopy::Ref::new_unaligned_from_suffix(archive) {
        Some((_, footer)) => Ok(footer),
        None => Err(Invalid::TooShort(archive.len() as u64).into()),
    }
}
