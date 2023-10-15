use std::fs::File;

use zerocopy::{AsBytes, FromZeroes};

use crate::error::Invalid;

use super::*;

pub struct FileInner(Cache<File>);

impl Archive for File {
    type Inner = FileInner;

    fn to_archive(self) -> Result<Self::Inner, CheckErr> {
        Ok(FileInner(Cache { footer: get_footer_file(&self)?, inner: self }))
    }

    fn from_inner(inner: Self::Inner) -> Self {
        inner.0.inner
    }
}

impl ArchiveInner for FileInner {
    fn check(&self) -> Result<(), CheckErr> {
        Ok(())
    }
}

fn get_footer_file(file: &File) -> Result<Footer, CheckErr> {
    let len = file.metadata()?.len();
    let footer_off = len.checked_sub(Footer::LEN as u64).ok_or(Invalid::TooShort(len))?;

    let mut footer = Footer::new_zeroed();
    let mut buf = footer.as_bytes_mut();

    #[cfg(unix)]
    {
        use std::os::unix::fs::FileExt;
        file.read_exact_at(&mut buf, footer_off)?;
    }

    #[cfg(not(unix))]
    {
        file.seek(SeekFrom::Start(footer_off))?;
        file.read_exact(&mut buf)?;
    }

    // footer.validate()?;

    Ok(footer)
}
