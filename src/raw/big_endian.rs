//! Endian order types.
use zerocopy::big_endian as be;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
/// A big-endian unsigned 16-bit integer.
pub struct U16(be::U16);

impl U16 {
    pub const fn new(n: u16) -> Self {
        // Self(be::U16::new(n)) is not const
        unsafe { core::mem::transmute(n.to_be()) }
    }

    pub const fn swap(self) -> u16 {
        // self.0.get() is not const
        u16::from_be(unsafe { core::mem::transmute(self) })
    }

    pub fn set(&mut self, n: u16) {
        self.0.set(n);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
/// A big-endian unsigned 32-bit integer.
pub struct U32(be::U32);

impl U32 {
    pub const fn new(n: u32) -> Self {
        // Self(be::U32::new(n)) is not const
        unsafe { core::mem::transmute(n.to_be()) }
    }

    pub const fn swap(self) -> u32 {
        // self.0.get() is not const
        u32::from_be(unsafe { core::mem::transmute(self) })
    }

    pub fn set(&mut self, n: u32) {
        self.0.set(n);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[derive(AsBytes, FromBytes, FromZeroes, Unaligned)]
/// A big-endian unsigned 64-bit integer.
pub struct U64(be::U64);

impl U64 {
    pub const fn new(n: u64) -> Self {
        // Self(be::U64::new(n)) is not const
        unsafe { core::mem::transmute(n.to_be()) }
    }

    pub const fn swap(self) -> u64 {
        // self.0.get() is not const
        u64::from_be(unsafe { core::mem::transmute(self) })
    }

    pub fn set(&mut self, n: u64) {
        self.0.set(n);
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// /// A native-endian unsigned 16-bit integer.
// pub struct U16(u16);

// impl U16 {
//     pub const fn new(n: u16) -> Self {
//         Self(n)
//     }

//     pub const fn swap(self) -> u16 {
//         self.0
//     }

//     pub fn set(&mut self, n: u16) {
//         self.0 = n;
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// /// A native-endian unsigned 32-bit integer.
// pub struct U32(u32);

// impl U32 {
//     pub const fn new(n: u64) -> Self {
//         Self(n)
//     }

//     pub const fn swap(self) -> u64 {
//         self.0
//     }

//     pub fn set(&mut self, n: u64) {
//         self.0 = n;
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// /// A native-endian unsigned 64-bit integer.
// pub struct U64(u64);

// impl U64 {
//     pub const fn new(n: u64) -> Self {
//         Self(n)
//     }

//     pub const fn swap(self) -> u64 {
//         self.0
//     }

//     pub fn set(&mut self, n: u64) {
//         self.0 = n;
//     }
// }
