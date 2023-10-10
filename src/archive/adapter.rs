use super::*;

/// Generic adapter for reading archives.
pub struct ReadAdapter<R>(R);

impl<R> ReadAdapter<R> {
    pub fn into_inner(self) -> R {
        self.0
    }
}

impl<R: Read + Seek> Archive for ReadAdapter<R> {
    fn check(&self) -> Result<(), Invalid> {
        todo!()
    }
}

pub struct MemAdapter<T>(T);

impl<T> MemAdapter<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: AsRef<[u8]>> Archive for MemAdapter<T> {
    fn check(&self) -> Result<(), Invalid> {
        todo!()
    }
}
