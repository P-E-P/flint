pub mod gate;
pub mod segment;
pub mod tss;

#[repr(u8)]
pub enum PrivilegeLevel {
    Kernel = 0,
    Unused1 = 1,
    Unused2 = 2,
    Userland = 3,
}

#[repr(u8)]
pub enum Granularity {
    Byte = 0,
    FourKByte = 1,
}

/// Split a 32 bits address in three parts, first part containing
/// bits from 31 to 24, second part the bits 23 to 16 and last part
/// bits 15 to 0.
pub fn split_address(address: u32) -> (u32, u32, u32) {
    (
        (address & 0xff) >> 16,
        (address & 0xff) >> 8,
        address & 0xff,
    )
}

/// Split a 32 bit limit in two parts, first part contains bits
/// from 19 to 16, second part contains bits from 15 to 0.
pub fn split_limit(limit: u32) -> (u32, u32) {
    (limit >> 16, limit & 0xffff)
}
