use alloc::boxed::Box;
use async_trait::async_trait;
use embassy_embedded_hal::shared_bus::I2cDeviceError;
use esp_hal::i2c;

use super::DeviceError;

#[async_trait(?Send)]
pub trait Thermometer: Sensor {
    async fn get_temp_raw(&mut self) -> Result<f64, DeviceError>;
    async fn get_temp_celsius(&mut self) -> Result<f64, DeviceError>;
    async fn get_temp_fahrenheit(&mut self) -> Result<f64, DeviceError> {
        Ok(self.get_temp_celsius().await? * 9f64 / 5f64 + 32f64)
    }
}
