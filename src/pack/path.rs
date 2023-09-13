#[derive(Debug, Hash)]
#[repr(transparent)]
/// Case-insensitive path.
pub struct Path(unix_path::Path);

impl Path {
    pub fn new<'a>(path: impl AsRef<[u8]> + 'a) -> &'a Self {
        todo!()
    }

    pub fn as_str(&self) -> &str {
        todo!()
    }

    pub fn as_unix_path(&self) -> &unix_path::Path {
        &self.0
    }

    pub fn is_canonical(&self) -> bool {
        todo!()
    }

    #[cfg(feature = "alloc")]
    pub fn canonicalize(&mut self) -> alloc::borrow::Cow<'_, unix_path::Path> {
        todo!()
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        todo!("case-insensitive comparison")
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        todo!("case-insensitive comparison")
    }
}

impl Eq for Path {}
