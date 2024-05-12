use alloc::collections::VecDeque;
use esp_hal::{clock::{ClockControl, Clocks}, delay::Delay, peripherals::Peripherals};
use esp_hal::prelude::_esp_hal_system_SystemExt;

// pub mod led;
// pub mod spi;
pub mod i2c;
pub mod i2s;
pub mod sdio;

pub trait HardwareRequest {
    fn service_request(&self);
}

pub struct Kernel<'a> {
    pub request_queue: VecDeque<&'a dyn HardwareRequest>,
}

impl Kernel<'_> {
    pub fn new() -> Self {
        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();
        let cocks = ClockControl::max(system.clock_control).freeze();
        Self {
            request_queue: VecDeque::new(),
        }
    }
}
