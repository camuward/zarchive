#![cfg_attr(not(feature = "std"), no_std)]

pub use self::pack::ArchiveReader;
pub mod pack;

#[cfg(feature = "zerocopy")]
pub mod zerocopy;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
