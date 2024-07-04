mod consts;

use embedded_hal_async::i2c::I2c;
use alloc::string::String;
use async_trait::async_trait;
use consts::*;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice as I2cContainer;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Timer;
use esp_hal::{i2c::{Instance, I2C}, Async};
use alloc::boxed::Box;

use crate::os::kernel::{drivers::{imu::lis3mdl::consts::DEVICE_ADDRESS, Device, DeviceError, SetupError}, interfaces::i2c::I2cDevice};

use super::TextDisplay;

pub struct Pcf8574t<N: Instance + 'static> {
    i2c: I2cContainer<'static, NoopRawMutex, I2C<'static, N, Async>>,
    top_string: String,
    bottom_string: String,
}

impl<N: Instance> Pcf8574t<N> {
    pub async fn new(i2c: I2cContainer<'static, NoopRawMutex, I2C<'static, N, Async>>) -> Self {
        let mut new = Self {
            i2c,
            top_string: String::new(),
            bottom_string: String::new(),
        };
        new.init();
        new
    }
    pub async fn init(&mut self) {
        self.lcd_write(0x03, 0x00).await;
        self.lcd_write(0x03, 0x00).await;
        self.lcd_write(0x03, 0x00).await;
        self.lcd_write(0x02, 0x00).await;
        self.lcd_write(
            LCD_FUNCTIONSET | LCD_2LINE | LCD_5X8DOTS | LCD_4BITMODE,
            0x00,
            ).await;
        self.lcd_write(LCD_DISPLAYCONTROL | LCD_DISPLAYON, 0x00).await;
        self.lcd_write(LCD_CLEARDISPLAY, 0x00).await;
        self.lcd_write(LCD_ENTRYMODESET | LCD_ENTRYLEFT, 0x00).await;
        Timer::after_millis(200).await;
    }
    async fn write_cmd(&mut self, data: u8) {
        self.i2c.write(LCD_ADDRESS, &[data]).await.unwrap();
    }
    async fn lcd_strobe(&mut self, data: u8) {
        self.write_cmd(data | EN | LCD_BACKLIGHT).await;
        Timer::after_micros(1500).await;
        self.write_cmd((data & !EN) | LCD_BACKLIGHT).await;
        Timer::after_micros(300).await;
    }
    async fn lcd_write_four_bits(&mut self, data: u8) {
        self.write_cmd(data | LCD_BACKLIGHT).await;
        self.lcd_strobe(data).await;
    }
    async fn lcd_write(&mut self, cmd: u8, mode: u8) {
        self.lcd_write_four_bits(mode | (cmd & 0xF0)).await;
        self.lcd_write_four_bits(mode | ((cmd << 4) & 0xF0)).await;
    }
}

#[async_trait(?Send)]
impl<N: Instance> Device for Pcf8574t<N> {
    async fn enable(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }
    async fn enabled(&self) -> bool { true }
    async fn disable(&mut self) -> Result<(), DeviceError> {
        Err(DeviceError::Setup(SetupError::NoDisable))
    }
    async fn estimated_current_draw(&mut self) -> f64 {0f64}
}

impl<N: Instance> I2cDevice<N> for Pcf8574t<N> {
    const DEVICE_ADDRESS: u8 = DEVICE_ADDRESS;
    fn get_bus(&mut self) -> &mut I2cContainer<'static, NoopRawMutex, I2C<'static, N, Async>> {
        &mut self.i2c
    }
}

#[async_trait(?Send)]
impl<N: Instance> TextDisplay for Pcf8574t<N> {
    async fn display_on_line(&mut self, string: String, line: u8) {
        if line == 1 {
            self.top_string = string.clone();
            self.lcd_write(0x80, 0x00).await;
        } else if line == 2 {
            self.bottom_string = string.clone();
            self.lcd_write(0xC0, 0x00).await;
        } else {
            return;
        }
        for char in string.as_bytes().iter() {
            self.lcd_write(*char, RS).await;
        }
    }
    async fn clear(&mut self) {
        self.top_string = String::new();
        self.bottom_string = String::new();
        self.lcd_write(LCD_CLEARDISPLAY, 0x00).await;
        self.lcd_write(LCD_RETURNHOME, 0x00).await;
    }
    async fn display_text(&mut self, text: String) {
       self.display_on_line(self.bottom_string.clone(), 1).await;
       self.display_on_line(text, 2).await;

    }
}
