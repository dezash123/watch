pub mod config;
pub mod consts;

use async_trait::async_trait;
use alloc::boxed::Box;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use esp_hal::{i2c::{Instance, I2C}, Async};
use config::Config;
use nalgebra::Vector3;
use consts::*;

pub struct Lis3mdl<N: Instance + 'static> {
    i2c: I2cDevice<'static, NoopRawMutex, I2C<'static, N, Async>>,
    config: Config,
}

impl<N: Instance> Lis3mdl<N> {
    pub async fn new(i2c: I2cDevice<'static, NoopRawMutex, I2C<'static, N, Async>>, config: Config) -> Self {
        Self { i2c, config }
    }
}

impl<N: Instance> I2cMethods<N> for Lis3mdl<N> {
    fn get_device_address() -> u8 { DEVICE_ADDRESS }
    fn get_bus(&mut self) -> &mut I2cDevice<'static, NoopRawMutex, I2C<'static, N, Async>> { &mut self.i2c }
}
