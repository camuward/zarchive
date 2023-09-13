use zerocopy::big_endian::{U16, U32, U64};

pub use self::entry::*;
pub use self::node::*;
pub use self::record::*;

pub const COMPRESSED_BLOCK_SIZE: usize = 64 * 1024; // 64 KiB
pub const ENTRIES_PER_OFFSETRECORD: usize = 16; // must be aligned to 2

mod entry;
mod node;
mod record;

const _: () = {
    use core::mem::size_of;

    assert!(size_of::<ArchiveEntry>() == 16);
    assert!(size_of::<Footer>() == 16 * 6 + 32 + 8 + 4 + 4);
    assert!(size_of::<CompressionOffsetRecord>() == 8 + 16 * 2);
};
