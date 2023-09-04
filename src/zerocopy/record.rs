use zerocopy::big_endian::{U16, U32, U64};
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

use super::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
pub struct CompressionOffsetRecord {
    // for every Nth entry we store the full 64bit offset, the blocks in between calculate the
    // offset from the size array
    pub base_offset: U64,
    pub size: [U16; ENTRIES_PER_OFFSETRECORD], // compressed size - 1
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
pub struct Footer {
    sections: Sections,
    integrity_hash: [u8; 32],
    total_size: U64,
    version: U32,
    magic: U32,
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
    pub offset: U64,
    pub size: U64,
}

impl OffsetInfo {
    pub const fn is_in_range(&self, archive_len: u64) -> bool {
        let offset = get_u64(self.offset);
        let size = get_u64(self.size);

        match offset.checked_add(size) {
            Some(end) => end <= archive_len,
            None => false,
        }
    }
}
