pub mod gate;
pub mod segment;
pub mod tss;

#[repr(u8)]
pub enum Granularity {
    Byte = 0,
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
