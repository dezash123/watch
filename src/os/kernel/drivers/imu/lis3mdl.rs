pub mod config;
pub mod consts;

use async_trait::async_trait;
use alloc::boxed::Box;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice as I2cContainer;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use esp_hal::{i2c::{Instance, I2C}, Async};
use nalgebra::Vector3;
use deku::DekuContainerWrite;
use consts::*;
use core::result::Result;

use crate::os::kernel::{drivers::{ConfigurableDevice, Device, DeviceError, I2cSensor16Bit, Sensor}, interfaces::i2c::I2cDevice};

use self::config::Lis3mdlConfig;

use super::{I2c3AxisSensor16Bit, Magnetometer, Sensor3Axis};

pub struct Lis3mdl<N: Instance + 'static> {
    i2c: I2cContainer<'static, NoopRawMutex, I2C<'static, N, Async>>,
    config: Lis3mdlConfig,
}

impl<N: Instance> Lis3mdl<N> {
    pub async fn new(i2c: I2cContainer<'static, NoopRawMutex, I2C<'static, N, Async>>, config: Lis3mdlConfig) -> Self {
        Self { i2c, config }
    }
}

#[async_trait(?Send)]
impl<N: Instance> ConfigurableDevice<Lis3mdlConfig> for Lis3mdl<N> {
    async fn configure(&mut self) -> Result<(), DeviceError> {
        self.write(CONFIG_START, &self.config.to_bytes().expect("aAa")[0..=4])
    }
}

#[async_trait(?Send)]
impl<N: Instance> Device for Lis3mdl<N> { 
    async fn enable(&mut self) -> Result<(), DeviceError> {
        self.config.power_down = false;
        self.write(CONFIG_START, self.config[2])
    }
    async fn enabled(&self) -> bool {
        !self.config.power_down
    }
    async fn disable(&mut self) -> Result<(), DeviceError> {
        self.config.power_down = true;
        self.write(CONFIG_START, self.config[2])
    }
    async fn estimated_current_draw(&mut self) -> f64 {
        0.0
    }
}

impl<N: Instance> I2cDevice<N> for Lis3mdl<N> {
    const DEVICE_ADDRESS: u8 = DEVICE_ADDRESS;
    fn get_bus(&mut self) -> &mut I2cContainer<'static, NoopRawMutex, I2C<'static, N, Async>> { &mut self.i2c }
}

impl<N: Instance> Sensor for Lis3mdl<N> {}

#[async_trait(?Send)]
impl<N: Instance> Sensor3Axis for Lis3mdl<N> {
    fn get_conversion_factor(&self) -> f64 {
        self.config.magnetometer_range.conversion_factor()
    }
}

impl<N: Instance> I2cSensor16Bit<N> for Lis3mdl<N> {}

#[async_trait(?Send)]
impl<N: Instance> I2c3AxisSensor16Bit<N> for Lis3mdl<N> {
    const DATA_OUT_START: [u8] = [OUT_START];
}

#[async_trait(?Send)]
impl <N: Instance> Magnetometer for Lis3mdl<N> {}
