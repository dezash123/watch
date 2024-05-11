use esp_hal::{clock::{ClockControl, Clocks}, delay::Delay, peripherals::Peripherals, system::SystemParts};

pub struct Kernel {
    pub peripherals: Peripherals, 
    pub system: SystemParts<'static>,
    pub cocks: Clocks<'static>,
    pub delay: Delay,
}

impl Kernel {
    pub fn new() -> Self {
        let peripherals = Peripherals::take();
        let system = peripherals.SYSTEM.split();
        let cocks = ClockControl::max(system.clock_control).freeze();
        let mut delay = Delay::new(&cocks);
        Self {
            peripherals,
            system,
            cocks,
            delay,
        }
    }
}
