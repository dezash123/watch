use embedded_hal::digital::{ InputPin, OutputPin };
use embedded_hal_async::{ i2c::I2c, delay::DelayNs };
use thiserror::Error;
use core::error;

pub const I2C_ADR: u8 = 0x5c; //8bit 

// #define CHSC6X_MAX_POINTS_NUM     (1)
pub const MAX_X: u16 = 370;
pub const MAX_Y: u16 = 370;

// /*MACRO SWITCH for driver update TP FW */
// #define CHSC6X_AUTO_UPGRADE           (0)
// 
// /*MACRO SWITCH for multi TP_VENDOR Compatible update TP FW */
// #define CHSC6X_MUL_VENDOR_UPGRADE     (0)
// 
// #define MAX_IIC_WR_LEN          (8)
// #define MAX_IIC_RD_LEN          (16)

#[derive(Error, Debug)]
enum Chsc6417Error<I2C: I2c, INT: OutputPin + InputPin, RST: OutputPin> {
    I2c(#[from] I2C::Error),
    InterruptOutput(#[from] INT::Error),
    ResetPin(#[from] RST::Error),
}

pub struct Chsc6417<I2C: I2c, INT: OutputPin + InputPin, RST: OutputPin, TMR: DelayNs> {
    i2c: I2C,
    interrupt_pin: INT,
    reset_pin: RST,
    delay: TMR,
    suspend: bool,
}

impl<I2C: I2c, INT: OutputPin + InputPin, RST: OutputPin, TMR: DelayNs> Chsc6417<I2C, INT, RST, TMR> {

}

