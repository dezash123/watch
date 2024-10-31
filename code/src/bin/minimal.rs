#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use main as _; // global logger + panicking-behavior + memory layout

// TODO(7) Configure the `rtic::app` macro
#[rtic::app(
    // TODO: Replace `some_hal::pac` with the path to the PAC
    device = embassy_stm32,
    // TODO: Replace the `FreeInterrupt1, ...` with free interrupt vectors if software tasks are used
    // You can usually find the names of the interrupt vectors in the some_hal::pac::interrupt enum.
    dispatchers = [TIM2],
)]

mod app {
    use embassy_stm32::gpio::{Level, Output, Speed};
    use embassy_stm32::time::Hertz;
    use embassy_stm32::Config;
    use defmt::info;
    use embassy_time::Timer;

    #[shared]
    struct Shared {
        
    }

    // Local resources go here
    #[local]
    struct Local {
        led_r: Output<'static>,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        info!("init");

        let mut config = Config::default();
        let sysclk = {
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
            config.rcc.sys
        };

        let pac = embassy_stm32::init(config);
        
        info!("Hello from asdfasdf!");
        // task1::spawn().ok();

        (
            Shared {
                // Initialization of shared resources go here
            },
            Local {
                led_r: Output::new(pac.PD7, Level::High, Speed::Low),
            },
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle]
    fn idle(_: idle::Context) -> ! {
        info!("idle");

        loop {
            continue;
        }
    }

    // #[task(priority = 1, local = [led_r])]
    // async fn task1(_cx: task1::Context) {
    //     loop {
    //         // info!("Hello from task1!{}", Instant::now().as_millis());
    //         Timer::after_millis(100).await;
    //     }
    // }
}
