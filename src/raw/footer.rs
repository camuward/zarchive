use super::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
pub struct CompressionOffsetRecord {
    // for every Nth entry we store the full 64bit offset, the blocks in between calculate the
    // offset from the size array
    base_offset: U64,
    size: [U16; Self::ENTRIES], // compressed size - 1
}

impl CompressionOffsetRecord {
    /// The number of entries per offset record.
    pub const ENTRIES: usize = 16; // must be aligned to 2
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
pub struct Footer {
    sections: Sections,
    integrity_hash: [u8; 32],
    total_size: U64,
    // #[valid(|m| m == 0x61bf_3a01)]
    version: U32,
    // #[valid(|m| m == 0x169f_52d6)]
    magic: U32,
}

impl Footer {
    pub const MAGIC: u32 = 0x169f_52d6;
    pub const VERSION: u32 = 0x61bf_3a01;
    pub const LEN: usize = core::mem::size_of::<Self>();
}

impl Valid for Footer {
    fn check(&self) -> Result<(), Invalid> {
        let magic = self.magic.swap();
        if magic != Self::MAGIC {
            return Err(Invalid::Magic(magic));
        }

        let version = self.version.swap();
        if version != Self::VERSION {
            return Err(Invalid::Version(version));
        }

        let sections = {
            let s = self.sections;
            [s.compressed_data, s.offset_records, s.names, s.file_tree, s.meta_dir, s.meta_data]
                .into_iter()
                .zip([
                    "compressed_data",
                    "offset_records",
                    "names",
                    "file_tree",
                    "meta_dir",
                    "meta_data",
                ])
        };

        let size = self.total_size.swap();
        for (section, name) in sections {
            if !section.is_in_range(size) {
                return Err(Invalid::Section(name));
            }
        }

        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
pub struct Sections {
    compressed_data: OffsetInfo,
    offset_records: OffsetInfo,
    names: OffsetInfo,
    file_tree: OffsetInfo,
    meta_dir: OffsetInfo,
    meta_data: OffsetInfo,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
pub struct OffsetInfo {
    offset: U64,
    size: U64,
}

impl OffsetInfo {
    pub const fn is_in_range(&self, archive_len: u64) -> bool {
        let offset = self.offset.swap();
        let size = self.size.swap();

        match offset.checked_add(size) {
            Some(end) => end <= archive_len,
            None => false,
        }
    }
}
