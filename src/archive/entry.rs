use zarchive_derive::Valid;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

use super::big_endian::{U16, U32};

/// Rust representation of an entry record.
pub enum Record {
    Dir(DirRecord),
    File(FileRecord),
}

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes, FromZeroes, Unaligned)]
/// An entry in the archive. Represents either a directory or a file.
pub struct ArchiveEntry {
    /// Offset into the node name table. The most significant bit is a flag
    /// representing whether the entry is a file (`true`) or directory
    /// (`false`).
    flag_offset: U32,
    record: EntryRecord,
}

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes, FromZeroes, Unaligned)]
union EntryRecord {
    dir: DirRecord,
    file: FileRecord,
}

impl ArchiveEntry {
    /// The flag for the file bit in the offset.
    pub const FILE_FLAG: u32 = 0x80000000;

    #[inline]
    /// Creates a new entry with the provided name table offset.
    ///
    /// # Panics
    ///
    /// Panics if the provided offset sets the most significant bit
    /// (`0x80000000`), which is reserved for the file flag. Thus, the value
    /// must be within the range `0..=i32::MAX as u32`.
    pub const fn new(offset: u32, record: Record) -> Self {
        match record {
            Record::Dir(dir) => Self::new_dir(offset, dir),
            Record::File(file) => Self::new_file(offset, file),
        }
    }

    #[inline]
    /// Creates a directory record with the provided name table offset.
    ///
    /// # Panics
    ///
    /// Panics if the provided offset sets the most significant bit
    /// (`0x80000000`), which is reserved for the file flag. Thus, the value
    /// must be within the range `0..=i32::MAX as u32`.
    pub const fn new_dir(offset: u32, record: DirRecord) -> Self {
        assert!(offset < Self::FILE_FLAG);

        Self {
            flag_offset: U32::new(offset & !Self::FILE_FLAG),
            record: EntryRecord { dir: record },
        }
    }

    #[inline]
    /// Creates a file record with the provided name table offset.
    ///
    /// # Panics
    ///
    /// Panics if the provided offset sets the most significant bit
    /// (`0x80000000`), which is reserved for the file flag. Thus, the value
    /// must be within the range `0..=i32::MAX as u32`.
    pub const fn new_file(offset: u32, record: FileRecord) -> Self {
        assert!(offset < Self::FILE_FLAG);

        Self {
            flag_offset: U32::new(offset | Self::FILE_FLAG),
            record: EntryRecord { file: record },
        }
    }

    #[inline]
    /// Whether the entry is a directory.
    pub const fn is_dir(&self) -> bool {
        self.flag_offset.swap() & Self::FILE_FLAG == 0
    }

    #[inline]
    /// Whether the entry is a file.
    pub const fn is_file(&self) -> bool {
        self.flag_offset.swap() & Self::FILE_FLAG != 0
    }

    #[inline]
    pub const fn offset(&self) -> u32 {
        self.flag_offset.swap() & !Self::FILE_FLAG
    }

    #[inline]
    /// Sets the offset to the given value.
    ///
    /// # Panics
    ///
    /// Panics if the provided offset sets the most significant bit
    /// (`0x80000000`), which is reserved for the file flag. Thus, the value
    /// must be within the range `0..=i32::MAX as u32`.
    pub fn set_offset(&mut self, offset: u32) {
        assert!(offset < Self::FILE_FLAG);

        let flag = self.flag_offset.swap() & Self::FILE_FLAG;
        self.flag_offset.set(offset | flag);
    }

    pub const fn record(&self) -> Record {
        if self.is_dir() {
            Record::Dir(unsafe { self.record.dir })
        } else {
            Record::File(unsafe { self.record.file })
        }
    }

    #[inline]
    /// Sets the archive entry record to the provided file or directory record.
    pub fn set_record(&mut self, record: Record) {
        let old = self.flag_offset.swap();
        let new = match record {
            Record::Dir(dir) => {
                self.record.dir = dir;
                old & !Self::FILE_FLAG
            }
            Record::File(file) => {
                self.record.file = file;
                old | Self::FILE_FLAG
            }
        };

        self.flag_offset.set(new);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
pub struct DirRecord {
    pub node_start_ind: U32,
    pub count: U32,
    _reserved: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
pub struct FileRecord {
    pub offset_low: U32,
    pub size_low: U32,

    pub offset_high: U16,
    pub size_high: U16,
}

impl FileRecord {
    pub const fn offset(&self) -> u64 {
        let low = self.offset_low.swap() as u64;
        let high = self.offset_high.swap() as u64;

        low | high << 32
    }

    pub const fn file_size(&self) -> u64 {
        let low = self.size_low.swap() as u64;
        let high = self.size_high.swap() as u64;

        low | high << 32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_offset_and_kind() {
        let mut entry = ArchiveEntry::new_zeroed();

        for offset in [0, 1, 2, 3, 4, 5, 6, 7, 0x7FFFFFFE, 0x7FFFFFFF] {
            entry.set_offset(offset);
            assert_eq!(entry.offset(), offset);

            entry.set_record(Record::File(FileRecord::default()));
            assert!(entry.is_file());
            assert_eq!(entry.offset(), offset);

            entry.set_record(Record::Dir(DirRecord::default()));
            assert!(entry.is_dir());
            assert_eq!(entry.offset(), offset);
        }
    }
}
