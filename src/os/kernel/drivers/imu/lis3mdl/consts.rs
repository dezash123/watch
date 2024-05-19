// Registers:
pub const DEVICE_ADDRESS: u8 = 0b0011100; // SA1 connected to GND
pub const WHO_AM_I: u8 = 0x0F;
pub const WHO_AM_I_ID: u8 = 0b00111101;
pub const CTRL_START: u8 = 0x20;
pub const OUT_START: u8 = 0x28;

// Config values
pub enum Control1 {
    TempEn = 1 << 7,
    
}


