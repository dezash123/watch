use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, mutex::Mutex};
use esp_hal::{clock::ClockControl, gpio::IO, i2c::I2C, peripherals::{Peripherals, I2C0, I2C1}, Async};
use fugit::RateExtU32;
use static_cell::StaticCell;
use esp_hal::prelude::_esp_hal_system_SystemExt;

use crate::os::kernel::drivers::display::pcf8574t::Pcf8574t;

pub static I2C0_BUS: StaticCell<Mutex<NoopRawMutex, I2C<I2C0, Async>>> = StaticCell::new();
pub static I2C1_BUS: StaticCell<Mutex<NoopRawMutex, I2C<I2C1, Async>>> = StaticCell::new();

pub struct V1Kernel {
    text_display: Option<Pcf8574t<I2C0>>,
}

impl V1Kernel {
    pub async fn new() -> Self {
        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();
        let cocks = ClockControl::max(system.clock_control).freeze();
        let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

        let i2c0_scl = io.pins.gpio0;
        let i2c0_sda = io.pins.gpio1;
        let i2c1_scl = io.pins.gpio2;
        let i2c1_sda = io.pins.gpio3;

        let i2c0 = I2C::new_async(
            peripherals.I2C0,
            i2c0_sda,
            i2c0_scl,
            400u32.kHz(),
            &cocks,
        );
        let i2c1 = I2C::new_async(
            peripherals.I2C1,
            i2c1_sda,
            i2c1_scl,
            400u32.kHz(),
            &cocks,
        );

        let i2c0_bus = Mutex::new(i2c0);
        let i2c0_bus = I2C0_BUS.init(i2c0_bus);
        let i2c1_bus = Mutex::new(i2c1);
        let i2c1_bus = I2C1_BUS.init(i2c1_bus);

        let text_display = Some(Pcf8574t::new(I2cDevice::new(i2c0_bus)).await);

        Self {
            text_display,
        }
    }
}
