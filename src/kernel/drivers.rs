use alloc::boxed::Box;
use async_trait::async_trait;
use embassy_embedded_hal::shared_bus::I2cDeviceError;
use errors::*;
use esp_hal::i2c::{Error, Instance};
use uom::si::f64::ElectricCurrent;

use crate::util::conversions::single_be_to_f64;

use super::interfaces::{i2c::I2cDevice, Interface};

pub mod config;
pub mod display;
pub mod errors;
pub mod imu;
pub mod input;
pub mod misc;
pub mod sensor_data;

// anything I/O
#[async_trait(?Send)]
pub trait Device<I: Interface> {
    async fn enable(&mut self) -> Result<(), DeviceError> {
        Ok(())
    }
    async fn enabled(&self) -> bool {
        true
    }
    async fn disable(&mut self) -> Result<(), DeviceError> {
        Err(DeviceError::Setup(SetupError::NoDisable))
    }
    async fn estimated_current_draw(&mut self) -> ElectricCurrent {
        0f64
    } // Amps
}

#[async_trait(?Send)]
#[async_trait(?Send)]
pub trait I2cSensor16Bit<N: Instance + 'static>: Sensor + I2cDevice<N> {
    async fn read_single_be(
        &mut self,
        address: &[u8],
        conversion_factor: f64,
    ) -> Result<f64, DeviceError> {
        let bytes: [u8; 2] = self.read(address).await?;
        Ok(single_be_to_f64(bytes) * conversion_factor)
    }
}
