use core2::io::Read;
use unix_path::Path;

pub struct PathNodes<'a> {
    pos: u64,
    reader: &'a dyn Read,
}

impl<'a> Iterator for PathNodes<'a> {
    type Item = &'a Path;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
