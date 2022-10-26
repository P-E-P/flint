use pit8254::{setup_rate_generator, Channel};

mod pit8254;

/// PIT Channel 0 interrupt frequency.
pub const DESIRED_FREQUENCY: u16 = 100; // 100 Hz

/// Setup the 8254 PIT with a Rate Generator of [`DESIRED_FREQUENCY`] on IRQ0.
pub fn setup() {
    unsafe {
        setup_rate_generator(Channel::Channel0, DESIRED_FREQUENCY);
    }
}

/// A struct representing a tick counter.
pub struct TickCounter {
    /// Elasped ticks.
    ticks: u32,
    /// Counter expected frequency.
    frequency: u16,
}

impl TickCounter {
    /// Initializes a [`TickCounter`].
    ///
    /// # Arguments
    ///
    /// * `frequency` - The frequency at which *increment* will be called.
    pub const fn new(frequency: u16) -> Self {
        Self {
            ticks: 0,
            frequency,
        }
    }

    /// Increments the counter.
    pub fn increment(&mut self) {
        self.ticks += 1;
    }

    /// Returns the elasped ticks.
    pub fn elasped_ticks(&self) -> u32 {
        self.ticks
    }

    /// Returns the elasped seconds.
    pub fn elasped_seconds(&self) -> u32 {
        self.ticks / (self.frequency as u32)
    }
}

/// 8254 PIT's Channel 0 tick counter.
pub static mut TICK_COUNTER: TickCounter = TickCounter::new(DESIRED_FREQUENCY);
