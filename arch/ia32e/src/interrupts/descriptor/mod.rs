use task::TaskGate;

pub mod task;

#[repr(C, packed)]
pub struct Gate(u64);

impl From<TaskGate> for Gate {
    fn from(gate: TaskGate) -> Self {
        Self(u64::from(gate.upper.0) << 32 | u64::from(gate.lower.0))
    }
}
