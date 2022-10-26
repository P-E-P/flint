//! A module for the 8254 PIT.
use crate::arch::out_byte;
use crate::utils::bitfield::*;

const COUNTER_0: u16 = 0x40;
const COUNTER_1: u16 = 0x41;
const COUNTER_2: u16 = 0x42;
const CONTROL_REG: u16 = 0x43;

const INTERNAL_FREQUENCY: u32 = 1193182;

/// An enum representing the counter representation mode of a [`Channel`].
#[derive(PartialEq)]
enum CountMode {
    /// BCD counting.
    Bcd,
    /// Binary counting.
    Binary,
}

impl From<CountMode> for bool {
    fn from(value: CountMode) -> bool {
        value == CountMode::Bcd
    }
}

/// An enum representing the operating mode of a [`Channel`].
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

/// An enum representing the access policy of a [`Channel`].
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

/// An enum representing the three channels present on a 8254 PIT.
#[derive(PartialEq, Clone, Copy, Eq)]
pub enum Channel {
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

/// Sends a command on the Control Register of the PIT.
///
/// # Arguments
///
/// * `channel` - The [`Channel`] to setup.
/// * `count_mode` - The desired [`CountMode`] of the channel.
/// * `ope_mode` - The desired [`OperatingMode`] of the channel.
/// * `policy` - The desired [`AccessPolicy`] of the channel.
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

/// Setup a given channel as a Rate Generator on a desired frequency.
///
/// # Arguments
///
/// * `channel` - The [`Channel`] to setup.
/// * `frequency` - The desired interrupt frequency.
///
/// # Safety
///
/// The selected channel and frequency must be consistent.
pub unsafe fn setup_rate_generator(channel: Channel, frequency: u16) {
    send_command(
        channel,
        CountMode::Binary,
        OperatingMode::RateGenerator,
        AccessPolicy::Both,
    );

    let divisor: u16 = (INTERNAL_FREQUENCY / (frequency as u32)) as u16;
    if divisor == 1 {
        panic!("PIT 8254: Illegal divisor of 1 in Mode 2.");
    }

    // Set desired frequency, least significant byte first.
    out_byte(channel.address(), divisor.get_bits(0..=7) as u8);
    out_byte(channel.address(), divisor.get_bits(8..=15) as u8);
}
