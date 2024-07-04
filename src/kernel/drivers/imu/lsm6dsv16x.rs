pub mod config;
pub mod consts;

use async_trait::async_trait;
use embassy_embedded_hal::shared_bus::{asynch::i2c::I2cDevice, I2cDeviceError};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embedded_hal_async::i2c::I2c;
use esp_hal::{i2c::{Instance, I2C, Error}, Async};
use config::Config;
use nalgebra::Vector3;
use alloc::{boxed::Box, vec::Vec};
use consts::*;

use crate::os::kernel::peripherals::i2c::I2cMethods;

use super::{Accelerometer, AccelerometerError, Gyroscope, GyroscopeError};

pub struct Lsm6dsv16x<N: Instance + 'static> {
    i2c: I2cDevice<'static, NoopRawMutex, I2C<'static, N, Async>>,
    config: Config,
}

impl<N: Instance> Lsm6dsv16x<N> {
    async fn new(i2c: I2cDevice<'static, NoopRawMutex, I2C<'static, N, Async>>, config: Config) -> Self {
        Self { i2c, config }
    }
}

impl<N: Instance> I2cMethods<N> for Lsm6dsv16x<N> {
    fn get_device_address() -> u8 { DEVICE_ADDRESS }
    fn get_bus(&mut self) -> &mut I2cDevice<'static, NoopRawMutex, I2C<'static, N, Async>> { &mut self.i2c }
}

#[async_trait(?Send)]
impl<N: Instance> Accelerometer for Lsm6dsv16x<N> {
    async fn get_acceleration(&mut self) -> Result<Vector3<f64>, AccelerometerError> {
    }
}

#[async_trait(?Send)]
impl<N: Instance> Gyroscope for Lsm6dsv16x<N> {
    async fn get_angular_velocity(&mut self) -> Result<Vector3<f64>, GyroscopeError> {
    }
}
