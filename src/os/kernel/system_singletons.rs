// use esp_hal::{clock::{ClockControl, Clocks}, peripheral::{self, Peripheral}, peripherals::{self, Peripherals}, prelude::*, system::{self, SystemParts}, xtensa_lx::timer::delay, Delay};
// 
// pub struct Singletons {
//     pub peripherals: Peripherals, 
//     pub system: SystemParts<'static>,
//     pub cocks: Clocks<'static>,
//     pub delay: Delay,
// }
// 
// impl Singletons {
//     pub fn new() -> Self {
//         let peripherals = Peripherals::take();
//         let system = peripherals.SYSTEM.split();
//         let cocks = ClockControl::max(system.clock_control).freeze();
//         let mut delay = Delay::new(&cocks);
//         Self {
//             peripherals,
//             system,
//             cocks,
//             delay,
//         }
//     }
// }
