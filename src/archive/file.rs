use std::fs::File;

use super::*;

impl Archive for File {
    fn check(&self) -> Result<(), Invalid> {
        todo!()
    }
}
