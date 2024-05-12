use esp_hal::{clock::{ClockControl, Clocks}, delay::Delay, peripherals::Peripherals};
use esp_hal::prelude::_esp_hal_system_SystemExt;

pub struct Kernel {
    pub delay: Delay,
}

impl Kernel {
    pub fn new() -> Self {
        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();
        let cocks = ClockControl::max(system.clock_control).freeze();
        let delay = Delay::new(&cocks);
        Self {
            delay,
        }
    }
}
