//! A module containing the different descriptors structures.
pub mod gate;
pub mod segment;
pub mod tss;

/// The scaling of a descriptor's limit field.
#[repr(u8)]
pub enum Granularity {
    /// Byte unit granularity.
    Byte = 0,
    /// 4-KByte unit granularity.
    FourKByte = 1,
}

impl From<Granularity> for bool {
    fn from(value: Granularity) -> Self {
        match value {
            Granularity::Byte => false,
            Granularity::FourKByte => true,
        }
    }
}
