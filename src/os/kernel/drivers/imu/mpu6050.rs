use async_trait::async_trait;
use alloc::boxed::Box;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use esp_hal::{i2c::{Instance, I2C}, Async};
use config::Config;
use nalgebra::Vector3;

use crate::{os::kernel::{drivers::misc::{Thermometer, ThermometerError}, peripherals::i2c::I2cMethods}, util::conversions::{be_bytes_to_f64, single_be_to_f64}};
use self::config::{AccelerationRange, AngularVelocityRange};
use consts::*;

use super::{Accelerometer, AccelerometerError, Gyroscope, GyroscopeError};

pub mod config;
pub mod consts;

pub struct Mpu6050<N: Instance + 'static> {
    i2c: I2cDevice<'static, NoopRawMutex, I2C<'static, N, Async>>,
    config: Config,
}

impl<N: Instance> Mpu6050<N> {
    pub async fn new(i2c: I2cDevice<'static, NoopRawMutex, I2C<'static, N, Async>>, config: Config) -> Self {
        Self { i2c, config }
    }
}

impl<N: Instance> I2cMethods<N> for Mpu6050<N> {
    fn get_device_address() -> u8 { DEVICE_ADDRESS }
    fn get_bus(&mut self) -> &mut I2cDevice<'static, NoopRawMutex, I2C<'static, N, Async>> { &mut self.i2c }
}

#[async_trait(?Send)]
impl<N: Instance> Accelerometer for Mpu6050<N> {
    async fn get_acceleration(&mut self) -> Result<Vector3<f64>, AccelerometerError> {
        let byte_string: [u8; 6] = self.read(ACCEL_OUT_START).await?;
        Ok(Vector3::from(be_bytes_to_f64(byte_string, AccelerationRange::conversion_factor(&self.config.accelerometer_range))))
    }
}

#[async_trait(?Send)]
impl<N: Instance> Gyroscope for Mpu6050<N> {
    async fn get_angular_velocity(&mut self) -> Result<Vector3<f64>, GyroscopeError> {
        let byte_string: [u8; 6] = self.read(GYRO_OUT_START).await?;
        Ok(Vector3::from(be_bytes_to_f64(byte_string, AngularVelocityRange::conversion_factor(&self.config.gyro_range))))
    }
}

#[async_trait(?Send)]
impl<N: Instance> Thermometer for Mpu6050<N> {
    async fn get_temp_raw(&mut self) -> Result<f64, ThermometerError> {
        Ok(single_be_to_f64(self.read::<2>(TEMP_OUT_START).await?))
    }
    async fn get_temp_celsius(&mut self) -> Result<f64, ThermometerError> {
        Ok(self.get_temp_raw().await? * (1f64 / 340f64) + 36.53f64)
    }
}
