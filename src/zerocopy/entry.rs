use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

use super::*;

/// Rust representation of an entry record.
pub enum Record {
    Dir(DirRecord),
    File(FileRecord),
}

#[repr(C)]
#[derive(Clone, Copy, AsBytes, FromBytes, FromZeroes, Unaligned)]
/// An entry in the archive. Represents either a directory or a file.
pub struct ArchiveEntry {
    /// Offset into the node name table. The most significant bit is a flag representing whether
    /// the entry is a file (`true`) or directory (`false`).
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
    /// Creates a new `ArchiveEntry` with the provided name table offset and entry record.
    ///
    /// # Panics
    ///
    /// Panics if the provided offset sets the most significant bit (`0x80000000`), which is
    /// reserved for the file flag. Thus, the value must be within the range `0..=i32::MAX as u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zarchive2::zerocopy::{ArchiveEntry, DirRecord, EntryKind, FileRecord};
    ///
    /// let (dir, file) = (DirRecord::default(), FileRecord::default());
    /// let entry = ArchiveEntry::new(12345, EntryKind::File(file));
    /// assert!(entry.is_file());
    ///
    /// assert_eq!(entry.offset(), 12345); // `.offset()` strips the flag
    /// let with_flag = entry.offset() | ArchiveEntry::FILE_FLAG;
    /// assert_eq!(entry.flag_offset, with_flag); // the flag is set
    ///
    /// let entry = ArchiveEntry::new(54321, EntryKind::Dir(dir));
    /// assert_eq!(entry.flag_offset, 54321);
    ///
    /// // since the flag isn't set, `.offset()` returns the same value
    /// assert_eq!(entry.flag_offset, entry.offset());
    /// ```
    pub const fn new(offset: u32, record: Record) -> Self {
        match record {
            Record::Dir(dir) => Self::new_dir(offset, dir),
            Record::File(file) => Self::new_file(offset, file),
        }
    }

    #[inline]
    /// Creates a new `ArchiveEntry` with the provided name table offset and directory record.
    ///
    /// # Panics
    ///
    /// Panics if the provided offset sets the most significant bit (`0x80000000`), which is
    /// reserved for the file flag. Thus, the value must be within the range `0..=i32::MAX as u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zarchive2::zerocopy::{ArchiveEntry, DirRecord};
    ///
    /// let entry = ArchiveEntry::new_dir(0, DirRecord::default());
    /// assert!(entry.is_dir());
    ///
    /// // since this is a directory, the flag is not set
    /// assert_eq!(entry.flag_offset & ArchiveEntry::FILE_FLAG, 0);
    /// assert_eq!(entry.flag_offset, entry.offset());
    /// ```
    pub const fn new_dir(offset: u32, record: DirRecord) -> Self {
        assert!(offset < Self::FILE_FLAG);

        Self {
            flag_offset: U32::new(offset & !Self::FILE_FLAG),
            record: EntryRecord { dir: record },
        }
    }

    #[inline]
    /// Creates a new `ArchiveEntry` with the provided name table offset and file record.
    ///
    /// # Panics
    ///
    /// Panics if the provided offset sets the most significant bit (`0x80000000`), which is
    /// reserved for the file flag. Thus, the value must be within the range `0..=i32::MAX as u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zarchive2::zerocopy::{ArchiveEntry, FileRecord};
    ///
    /// let entry = ArchiveEntry::new_file(0, FileRecord::default());
    /// assert!(entry.is_file());
    ///
    /// // since this is a file, the flag is set
    /// assert_eq!(entry.flag_offset & ArchiveEntry::FILE_FLAG, ArchiveEntry::FILE_FLAG);
    /// assert_ne!(entry.flag_offset, entry.offset());
    ///
    /// assert_eq!(entry.flag_offset, entry.offset() | ArchiveEntry::FILE_FLAG);
    /// ```
    pub const fn new_file(offset: u32, record: FileRecord) -> Self {
        assert!(offset < Self::FILE_FLAG);

        Self {
            flag_offset: U32::new(offset | Self::FILE_FLAG),
            record: EntryRecord { file: record },
        }
    }

    #[inline]
    /// Whether the entry is a directory.
    ///
    /// # Examples
    ///
    /// ```
    /// use zarchive2::zerocopy::{ArchiveEntry, DirRecord, EntryKind, FileRecord};
    ///
    /// let mut entry = ArchiveEntry::new_dir(0, DirRecord::default());
    /// assert!(entry.is_dir()); // the entry is a directory
    ///
    /// entry.set_record(EntryKind::File(FileRecord::default()));
    /// assert!(!entry.is_dir()); // the entry is now a file
    /// ```
    pub const fn is_dir(&self) -> bool {
        self.flag_offset.get() & Self::FILE_FLAG == 0
    }

    #[inline]
    /// Whether the entry is a file.
    ///
    /// # Examples
    ///
    /// ```
    /// use zarchive2::zerocopy::{ArchiveEntry, DirRecord, EntryKind, FileRecord};
    ///
    /// let mut entry = ArchiveEntry::new_file(0, FileRecord::default());
    /// assert!(entry.is_file()); // the entry is a file
    ///
    /// entry.set_record(EntryKind::Dir(DirRecord::default()));
    /// assert!(!entry.is_file()); // the entry is now a directory
    /// ```
    pub const fn is_file(&self) -> bool {
        self.flag_offset.get() & Self::FILE_FLAG != 0
    }

    #[inline]
    pub const fn offset(&self) -> u32 {
        self.flag_offset.get() & !Self::FILE_FLAG
    }

    #[inline]
    /// Sets the offset to the given value.
    ///
    /// # Panics
    ///
    /// Panics if the provided offset sets the most significant bit (`0x80000000`), which is
    /// reserved for the file flag. Thus, the value must be within the range `0..=i32::MAX as u32`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zarchive2::zerocopy::ArchiveEntry;
    ///
    /// let mut entry = ArchiveEntry::new_zeroed();
    /// assert_eq!(entry.offset(), 0);
    ///
    /// entry.set_offset(0x7FFFFFFF);
    /// assert_eq!(entry.offset(), 0x7FFFFFFF);
    /// ```
    ///
    /// The function will panic if the offset is too large:
    ///
    /// ```should_panic
    /// use zarchive2::zerocopy::ArchiveEntry;
    ///
    /// let mut entry = ArchiveEntry::new_zeroed();
    /// entry.set_offset(2147483648); // panic!
    /// ```
    pub fn set_offset(&mut self, offset: u32) {
        assert!(offset < Self::FILE_FLAG);

        let flag = self.flag_offset.get() & Self::FILE_FLAG;
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
    ///
    /// # Examples
    ///
    /// ```
    /// use zarchive2::zerocopy::{ArchiveEntry, DirRecord, EntryKind, FileRecord};
    ///
    /// let mut entry = ArchiveEntry::new_zeroed();
    /// assert!(entry.is_dir()); // the flag isn't set
    /// assert_eq!(entry.flag_offset, 0);
    ///
    /// entry.set_record(EntryKind::File(FileRecord::default()));
    /// assert!(entry.is_file());
    /// assert_eq!(entry.flag_offset, entry.offset() | ArchiveEntry::FILE_FLAG);
    ///
    /// entry.set_record(EntryKind::Dir(DirRecord::default()));
    /// assert!(entry.is_dir());
    /// assert_eq!(entry.flag_offset, entry.offset() & !ArchiveEntry::FILE_FLAG);
    /// ```
    pub fn set_record(&mut self, record: Record) {
        let old = self.flag_offset.get();
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

    _reserved: [U32; 1],
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
        let low = self.offset_low.get() as u64;
        let high = self.offset_high.get() as u64;

        low | high << 32
    }

    pub const fn file_size(&self) -> u64 {
        let low = self.size_low.get() as u64;
        let high = self.size_high.get() as u64;

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
