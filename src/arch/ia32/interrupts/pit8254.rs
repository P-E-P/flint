use crate::arch::out_byte;
use crate::utils::bitfield::*;

const COUNTER_0: u16 = 0x40;
const COUNTER_1: u16 = 0x41;
const COUNTER_2: u16 = 0x42;
const CONTROL_REG: u16 = 0x43;

const INTERNAL_FREQUENCY: u32 = 1193182;
pub const DESIRED_FREQUENCY: u16 = 100; // 100 Hz

#[derive(PartialEq)]
enum CountMode {
    Bcd,
    Binary,
}

impl From<CountMode> for bool {
    fn from(value: CountMode) -> bool {
        value == CountMode::Bcd
    }
}

enum OperatingMode {
    InterruptOnTerminalCount,
    OneShot,
    RateGenerator,
    SquareWaveGenerator,
    SoftwareTriggeredStrobe,
    HardwareTriggeredStrobe,
}

impl From<OperatingMode> for u8 {
    fn from(value: OperatingMode) -> u8 {
        match value {
            OperatingMode::InterruptOnTerminalCount => 0,
            OperatingMode::OneShot => 1,
            OperatingMode::RateGenerator => 2,
            OperatingMode::SquareWaveGenerator => 3,
            OperatingMode::SoftwareTriggeredStrobe => 4,
            OperatingMode::HardwareTriggeredStrobe => 5,
        }
    }
}

enum AccessPolicy {
    LeastSignificant,
    MostSignificant,
    Both, // Least significant first, then most one.
}

impl From<AccessPolicy> for u8 {
    fn from(value: AccessPolicy) -> u8 {
        match value {
            AccessPolicy::LeastSignificant => 1,
            AccessPolicy::MostSignificant => 2,
            AccessPolicy::Both => 3,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Channel {
    Channel0,
    Channel1,
    Channel2,
}

impl Channel {
    pub fn address(&self) -> u16 {
        match self {
            Channel::Channel0 => COUNTER_0,
            Channel::Channel1 => COUNTER_1,
            Channel::Channel2 => COUNTER_2,
        }
    }
}

impl From<Channel> for u8 {
    fn from(value: Channel) -> u8 {
        match value {
            Channel::Channel0 => 0,
            Channel::Channel1 => 1,
            Channel::Channel2 => 2,
        }
    }
}

unsafe fn send_command(
    channel: Channel,
    count_mode: CountMode,
    ope_mode: OperatingMode,
    policy: AccessPolicy,
) {
    let command: u8 = 0
        .set_bit(0, bool::from(count_mode))
        .set_bits(1..=3, u8::from(ope_mode))
        .set_bits(4..=5, u8::from(policy))
        .set_bits(6..=7, u8::from(channel));

    out_byte(CONTROL_REG, command);
}

// Setup one channel as a Rate Generator for a desired frequency
unsafe fn setup_rate_generator(channel: Channel, frequency: u16) {
    send_command(
        channel,
        CountMode::Binary,
        OperatingMode::RateGenerator,
        AccessPolicy::Both,
    );

    let divisor: u16 = (INTERNAL_FREQUENCY / (frequency as u32)) as u16;
    if divisor == 1 {
        panic!("PIT 8284: Illegal divisor of 1 in Mode 2.");
    }

    // Set desired frequency, least significant byte first.
    out_byte(channel.address(), divisor.get_bits(0..=7) as u8);
    out_byte(channel.address(), divisor.get_bits(8..=15) as u8);
}

// Setup a Rate Generator of DESIRED_FREQUENCY on IRQ0.
pub fn setup_pit() {
    unsafe {
        setup_rate_generator(Channel::Channel0, DESIRED_FREQUENCY);
    }
}

pub struct TickCounter {
    ticks: u32,
    frequency: u16,
}

impl TickCounter {
    pub const fn new(frequency: u16) -> Self {
        Self {
            ticks: 0,
            frequency: frequency,
        }
    }

    pub fn increment(&mut self) {
        self.ticks += 1;
    }

    pub fn elasped_ticks(&self) -> u32 {
        self.ticks
    }

    pub fn elasped_seconds(&self) -> u32 {
        self.ticks / (self.frequency as u32)
    }
}

pub static mut TICK_COUNTER: TickCounter = TickCounter::new(DESIRED_FREQUENCY);
