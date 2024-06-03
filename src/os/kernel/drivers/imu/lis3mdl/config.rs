use deku::prelude::*;
use alloc::vec::Vec;
use alloc::format;
use crate::os::kernel::drivers::Config;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Lis3mdlConfig {
    // register 1
    #[deku(bits = "1")]
    pub enable_thermometer: bool,
    #[deku(bits = "2")]
    pub xy_power_level: PowerLevel,
    #[deku(bits = 3)]
    pub slow_data_rate: SlowDataRate,
    #[deku(bits = 1)]
    pub enable_fast_data_rate: bool,
    #[deku(bits = 1)]
    pub self_test: bool,
    // register 2
    #[deku(pad_bits_before = "1", pad_bits_after = "5", bits = 2)]
    pub magnetometer_range: MagneticFieldStrengthRange,
    // register 3
    #[deku(pad_bits_before = "2", bits = 1)]
    pub low_power_mode: bool,
    #[deku(pad_bits_before = "2", bits = 1)]
    pub spi_3_wire: bool,
    #[deku(bits = 1)]
    pub power_down: bool,
    #[deku(bits = 1)]
    pub single_conversion: bool, // false for continuous
    // register 4
    #[deku(pad_bits_before = "4", bits = 2)]
    pub z_power_level: PowerLevel,
    #[deku(pad_bits_after = "1", bits = 1)]
    pub big_endian: bool,
    // register 5
    #[deku(bits = 1)]
    pub fast_read: bool, // only output MS byte
    #[deku(pad_bits_after = "6", bits = 1)]
    pub block_data: bool, // dont update until data has been read?
    // interrupt config register
    #[deku(bits = 1)]
    pub x_interrupt_enable: bool,
    #[deku(bits = 1)]
    pub y_interrupt_enable: bool,
    #[deku(pad_bits_after = "1", bits = 1)]
    pub z_interrupt_enable: bool,
    #[deku(bits = 1)]
    pub always_true: bool,
    #[deku(bits = 1)]
    pub interrupt_is_high: bool,
    #[deku(bits = 1)]
    pub latched_interrupt: bool,
    #[deku(bits = 1)]
    pub interrupt_enable: bool,
    #[deku(bytes = 2)]
    pub interrupt_threshold: u16
}

impl Lis3mdlConfig {
    pub fn new(
    enable_thermometer: bool,
    xy_power_level: PowerLevel,
    slow_data_rate: SlowDataRate,
    enable_fast_data_rate: bool,
    self_test: bool,
    magnetometer_range: MagneticFieldStrengthRange,
    low_power_mode: bool,
    spi_3_wire: bool,
    power_down: bool,
    single_conversion: bool, // false for continuous
    z_power_level: PowerLevel,
    big_endian: bool,
    fast_read: bool, // only output MS byte
    block_data: bool // dont update until data has been read?
        ) {

    }
    // fn enable_interrupts() -> 
}

pub enum DataRate {
    Fast(FastDataRate),
    Slow(SlowDataRate, PowerLevel),
}

pub enum FastDataRate {
    // "fast" rates
    Hz1000, // low-power; 5.3 mG stddev
    Hz560, // low-power; 4.6 mG stddev
    Hz300, // low-power; 4.0 mG stddev
    Hz155, // low-power; 3.5 mG stddev
}

pub struct FastRateContainer {
    xy_rate: FastDataRate,
    z_rate: FastDataRate,  
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little", type = "u8", bits = "3")]
pub enum SlowDataRate {
    #[deku(id = "0b000")]
    Hz0_625,
    #[deku(id = "0b001")]
    Hz1_25,
    #[deku(id = "0b010")]
    Hz2_5,
    #[deku(id = "0b011")]
    Hz5,
    #[deku(id = "0b100")]
    Hz10,
    #[deku(id = "0b101")]
    Hz20,
    #[deku(id = "0b110")]
    Hz40,
    #[deku(id = "0b111")]
    Hz80,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "2")]
pub enum PowerLevel {
    #[deku(id = "0b00")]
    LowPower,
    #[deku(id = "0b01")]
    MediumPerformance,
    #[deku(id = "0b10")]
    HighPerformance,
    #[deku(id = "0b11")]
    UltraHighPerformance,
}

pub struct DataRateAndPowerLevel {
    xy_level: PowerLevel,
    z_level: PowerLevel,
    data_rate: DataRate,
}

// +/- range for magnetic field strength; gauss
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "2")]
pub enum MagneticFieldStrengthRange {
    #[deku(id = "0b00")]
    G4,
    #[deku(id = "0b01")]
    G8,
    #[deku(id = "0b10")]
    G12,
    #[deku(id = "0b11")]
    G16,
}

impl MagneticFieldStrengthRange {
    pub fn conversion_factor(&self) -> f64 {
        match self {
            Self::G4 => 1f64 / 16834f64,
            Self::G8 => 1f64 / 8192f64,
            Self::G12 => 1f64 / 4096f64,
            Self::G16 => 1f64 / 2048f64,
        }
    }
}

impl Config for Lis3mdlConfig {

}
