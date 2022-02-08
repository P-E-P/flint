pub mod thr;
pub mod lsr;

pub trait Register {
    type Value;

    fn read(&self) -> Self::Value;

    fn write(&self, value: Self::Value);
}
