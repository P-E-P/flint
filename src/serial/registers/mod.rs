pub mod dlh;
pub mod dll;
pub mod fcr;
pub mod ier;
pub mod iir;
pub mod lcr;
pub mod lsr;
pub mod mcr;
pub mod msr;
pub mod rbr;
pub mod sr;
pub mod thr;

pub trait Register {
    type Value;

    fn read(&self) -> Self::Value;

    fn write(&self, value: Self::Value);
}
