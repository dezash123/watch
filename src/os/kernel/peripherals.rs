pub mod ledc;
pub mod i2s;
pub mod sdio;
pub mod i2c;

pub trait ConfigRegister {
    fn get_byte(&self) -> u8;
}
