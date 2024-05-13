use esp_hal::{clock::ClockControl, gpio::IO, i2c::I2C, peripherals::{Peripherals, I2C0, I2C1}, Async};
use fugit::RateExtU32;
use esp_hal::prelude::_esp_hal_system_SystemExt;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use static_cell::StaticCell;

use super::drivers::pcf8574t::Pcf8574t;

pub static I2C0_BUS: StaticCell<Mutex<NoopRawMutex, I2C<I2C0, Async>>> = StaticCell::new();
pub static I2C1_BUS: StaticCell<Mutex<NoopRawMutex, I2C<I2C1, Async>>> = StaticCell::new();

// pub mod led;
// pub mod spi;
// pub mod i2c;
pub mod i2s;
pub mod sdio;

pub trait HardwareRequest {
    fn service_request(&self);
}

pub struct Kernel {
}

impl Kernel {
    pub fn new() -> Self {
        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();
        let cocks = ClockControl::max(system.clock_control).freeze();
        let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

        let i2c0_sda = io.pins.gpio0;
        let i2c0_scl = io.pins.gpio1;

        let i2c0 = I2C::new_async(
            peripherals.I2C0,
            i2c0_sda,
            i2c0_scl,
            400u32.kHz(),
            &cocks,
        );
        let i2c0_bus = Mutex::new(i2c0);
        let i2c0_bus = I2C0_BUS.init(i2c0_bus);

        let text_display = Pcf8574t::new(I2cDevice::new(i2c0_bus));

        Self {
        }
    }
}
