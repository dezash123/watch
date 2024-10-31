#![no_main]
#![no_std]

use defmt::*;
use core::sync::atomic::{AtomicUsize, Ordering};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::time::Hertz;
use embassy_stm32::Config;
use embassy_time::{ Timer, Instant };
use {defmt_brtt as _, panic_probe as _};

// Terminates the application and makes `probe-rs` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(24_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV3,
            mul: PllMul::MUL150,
            divp: Some(PllDiv::DIV2),
            divq: None,
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P; // 600 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 300 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 150 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 150 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // 150 Mhz
        config.rcc.apb5_pre = APBPrescaler::DIV2; // 150 Mhz
        config.rcc.voltage_scale = VoltageScale::HIGH;
    }
    let pac = embassy_stm32::init(config);
    info!("Hello World!");

    let led_r = Output::new(pac.PB7, Level::High, Speed::Low);
    let led_g = Output::new(pac.PD10, Level::High, Speed::Low);
    let led_b = Output::new(pac.PD13, Level::High, Speed::Low);

    spawner.spawn(blink_loop(led_r, 100, 300)).unwrap();
    Timer::after_millis(100).await;
    spawner.spawn(blink_loop(led_b, 100, 100)).unwrap();
    Timer::after_millis(100).await;
    spawner.spawn(blink_loop(led_g, 100, 300)).unwrap();
}

#[embassy_executor::task(pool_size = 3)]
async fn blink_loop(mut led: Output<'static>, high_time: u64, low_time: u64) -> ! {
    loop {
        // info!("{}", Instant::now().as_millis());
        led.set_high();
        Timer::after_millis(high_time).await;

        led.set_low();
        Timer::after_millis(low_time).await;
    }
}
