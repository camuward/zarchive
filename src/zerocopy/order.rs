#![allow(dead_code)]

use zerocopy::big_endian as be;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Unaligned};

#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, AsBytes, FromBytes, FromZeroes, Unaligned
)]
/// A big-endian 16-bit unsigned integer.
pub struct U16(be::U16);

impl U16 {
    #[inline]
    pub const fn new(value: u16) -> Self {
        Self(be::U16::from_bytes(value.to_be_bytes()))
    }

    #[inline]
    pub const fn get(self) -> u16 {
        u16::from_be(unsafe { core::mem::transmute(self) })
    }

    #[inline]
    pub fn set(&mut self, value: u16) {
        *self = Self::new(value);
    }
}

#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, AsBytes, FromBytes, FromZeroes, Unaligned
)]
/// A big-endian 32-bit unsigned integer.
pub struct U32(be::U32);

impl U32 {
    #[inline]
    pub const fn new(value: u32) -> Self {
        Self(be::U32::from_bytes(value.to_be_bytes()))
    }

    #[inline]
    pub const fn get(self) -> u32 {
        u32::from_be(unsafe { core::mem::transmute(self) })
    }

    #[inline]
    pub fn set(&mut self, value: u32) {
        *self = Self::new(value);
    }
}

#[repr(transparent)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Default, AsBytes, FromBytes, FromZeroes, Unaligned
)]
/// A big-endian 64-bit unsigned integer.
pub struct U64(be::U64);

impl U64 {
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(be::U64::from_bytes(value.to_be_bytes()))
    }

    #[inline]
    pub const fn get(self) -> u64 {
        u64::from_be(unsafe { core::mem::transmute(self) })
    }

    #[inline]
    pub fn set(&mut self, value: u64) {
        *self = Self::new(value);
    }
}
