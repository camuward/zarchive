use zerocopy::big_endian::{U16, U32, U64};

pub use self::entry::*;
pub use self::node::*;
pub use self::record::*;

pub const COMPRESSED_BLOCK_SIZE: usize = 64 * 1024; // 64 KiB
pub const ENTRIES_PER_OFFSETRECORD: usize = 16; // must be aligned to 2

const fn get_u16(n: U16) -> u16 {
    u16::from_be_bytes(unsafe { core::mem::transmute(n) })
}

const fn get_u32(n: U32) -> u32 {
    u32::from_be_bytes(unsafe { core::mem::transmute(n) })
}

const fn get_u64(n: U64) -> u64 {
    u64::from_be_bytes(unsafe { core::mem::transmute(n) })
}

mod entry;
mod node;
mod record;

const _: () = {
    use core::mem::size_of;

    assert!(size_of::<ArchiveEntry>() == 16);
    assert!(size_of::<Footer>() == 16 * 6 + 32 + 8 + 4 + 4);
    assert!(size_of::<CompressionOffsetRecord>() == 8 + 16 * 2);
};
