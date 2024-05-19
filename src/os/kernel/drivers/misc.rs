use embassy_embedded_hal::shared_bus::I2cDeviceError;
use esp_hal::i2c;
use alloc::boxed::Box;
use async_trait::async_trait;

pub enum ThermometerError {
    I2c(I2cDeviceError<i2c::Error>),
}

impl From<I2cDeviceError<i2c::Error>> for ThermometerError {
    fn from(value: I2cDeviceError<i2c::Error>) -> Self { Self::I2c(value) }
}

#[async_trait(?Send)]
pub trait Thermometer {
    async fn get_temp_raw(&mut self) -> Result<f64, ThermometerError>;
    async fn get_temp_celsius(&mut self) -> Result<f64, ThermometerError>;
    async fn get_temp_fahrenheit(&mut self) -> Result<f64, ThermometerError> {
        Ok(self.get_temp_celsius().await? * 9f64 / 5f64 + 32f64)
    }
}
