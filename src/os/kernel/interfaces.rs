use embassy_embedded_hal::shared_bus::I2cDeviceError;
use esp_hal::{i2c::{Error as I2cError, Instance as I2cInstance}, spi::master::Instance as SpiMasterInstance};
use thiserror_no_std::Error;

pub mod ledc;
pub mod i2s;
pub mod sdio;
pub mod i2c;

#[derive(Error, Debug)]
pub enum CommunicationError {
    #[error("I2C Error: {0:?}")]
    I2c(#[from] I2cDeviceError<I2cError>),
}

pub trait Interface {}

impl Interface for I2cInstance {}
impl Interface for SpiMasterInstance {}
