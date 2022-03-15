use core::fmt;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct Gate(u128);

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08X?};{:08X?}", self.0 >> 32, self.0 & 0xffffffff)
    }
}
