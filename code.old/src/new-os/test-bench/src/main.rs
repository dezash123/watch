#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::Io, peripherals::Peripherals, prelude::*, spi::{master::Spi, SpiMode, FullDuplexMode}, system::SystemControl
};
use icna5300::icna5300::{Backend, BlockingSpiBackend, Icna5300};

extern crate alloc;
use core::mem::MaybeUninit;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let mut clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);
    init_heap();

    esp_println::logger::init_logger_from_env();

    let timer = esp_hal::timer::PeriodicTimer::new(
        esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1, &clocks, None)
            .timer0
            .into(),
    );
    let _init = esp_wifi::initialize(
        esp_wifi::EspWifiInitFor::Wifi,
        timer,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
        &clocks,
    )
    .unwrap();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    
    let fspi_cs = io.pins.gpio10;
    let fspi_d0 = io.pins.gpio11;
    let fspi_d1 = io.pins.gpio13;
    let fspi_d2 = io.pins.gpio14;
    let fspi_d3 = io.pins.gpio9;
    let fspi_clk = io.pins.gpio12;

    let disp_rst = io.pins.gpio15;
    let disp_te = io.pins.gpio16;

    let mut fspi = Spi::new_half_duplex(
        peripherals.SPI2,
        100.kHz(),
        SpiMode::Mode0,
        &mut clocks,
    ).with_pins(Some(fspi_clk), Some(fspi_d0), Some(fspi_d1), Some(fspi_d2), Some(fspi_d3), Some(fspi_cs));
    
    let display_backend = BlockingSpiBackend<T: > {
        spi: &mut fspi,
        delay,
    };

    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}



pub struct BlockingSpiBackend<SPI: , D: DelayNs> {
    pub spi: SPI,
    pub delay: D,
}

impl<SPI: , D: DelayNs> Backend for BlockingSpiBackend<SPI, D> {
    fn write(&mut self, data: &[u8]) {
        self.spi.write(data).unwrap();
    }
    fn delay(&mut self, duration: MillisDurationU32) {
        self.delay.delay_ms(duration.to_millis());
    }
}
