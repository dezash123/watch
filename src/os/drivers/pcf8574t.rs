mod consts;
use core::str::FromStr;

use alloc::string::String;
use consts::*;
use embassy_time::Timer;
use crate::os::{devices::text_display::TextDisplay, Os};

pub struct Pcf8574t {
    os: &'static Os<'static>,
    top_string: String,
    bottom_string: String,
}

impl Pcf8574t {
    pub async fn new(os: &'static Os<'static>) -> Self {
        let new = Self{
            os,
            top_string: String::new(),
            bottom_string: String::new(),
        };
        new.init().await;
        new
    }
    pub async fn init(&self) {
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
    async fn write_cmd(&self, data: u8) {

    }
    async fn lcd_strobe(&self, data: u8) {
        self.write_cmd(data | EN | LCD_BACKLIGHT).await;
        Timer::after_micros(1500).await;
        self.write_cmd((data & !EN) | LCD_BACKLIGHT).await;
        Timer::after_micros(300).await;
    }
    async fn lcd_write_four_bits(&self, data: u8) {
        self.write_cmd(data | LCD_BACKLIGHT).await;
        self.lcd_strobe(data).await;
    }
    async fn lcd_write(&self, cmd: u8, mode: u8) {
        self.lcd_write_four_bits(mode | (cmd & 0xF0)).await;
        self.lcd_write_four_bits(mode | ((cmd << 4) & 0xF0)).await;
    }
}

impl TextDisplay for Pcf8574t {
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
