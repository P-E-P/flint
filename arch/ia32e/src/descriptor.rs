pub use ia32::descriptor::segment::*;
pub use ia32::descriptor::split_limit;

pub mod gate;
pub mod tss;

pub fn split_address(address: u64) -> (u32, u32) {
    (
        ((address >> 32) & 0xffffffff).try_into().unwrap(),
        (address & 0xffffffff).try_into().unwrap(),
    )
}
