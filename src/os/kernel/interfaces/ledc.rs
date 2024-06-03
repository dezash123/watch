use esp_hal::gpio::GpioProperties;
use esp_hal::gpio::GpioPin;
use esp_hal::gpio::Output;
use esp_hal::gpio::PushPull;
use esp_hal::ledc::LEDC;
use esp_hal::ledc::LSGlobalClkSource;
use fugit::HertzU32;
use esp_hal::ledc::channel;
use esp_hal::ledc::timer;
use esp_hal::ledc::LowSpeed;
use esp_hal::ledc::timer::LSClockSource;
use esp_hal::gpio::OutputPin;
use esp_hal::prelude::_esp_hal_ledc_timer_TimerIFace;
use esp_hal::prelude::_esp_hal_ledc_channel_ChannelIFace;

use crate::util::math;

pub struct LedDevice {
    ledc: LEDC<'static>,
}

impl LedDevice {
    fn new(mut ledc: LEDC<'static>) -> Self {
        ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);
        Self {
            ledc
        }   
    }
    pub fn configure_timer(&mut self, timer: timer::Number, bits: timer::config::Duty, freq: HertzU32) -> Result<(), esp_hal::ledc::timer::Error> {
        let config = timer::config::Config {
            duty: bits,
            clock_source: LSClockSource::APBClk,
            frequency: freq,
        };
        self.ledc.get_timer::<LowSpeed>(timer).configure(config)
    }
    pub fn configure_channel<const ID: u8>(&mut self, channel: channel::Number, timer: timer::Number, pin: GpioPin<Output<PushPull>, ID>, percent_output: f32) -> Result<(), esp_hal::ledc::channel::Error>
where
    GpioPin<Output<PushPull>, ID>: OutputPin,
    GpioPin<Output<PushPull>, ID>: GpioProperties
    {
        let config = channel::config::Config {
            timer: &self.ledc.get_timer::<LowSpeed>(timer),
            duty_pct: math::clamp_upct(percent_output) as u8,
            pin_config: channel::config::PinConfig::PushPull,

        };
        self.ledc.get_channel(channel, pin).configure(config)
    }
}
