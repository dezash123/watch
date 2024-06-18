use alloc::borrow::ToOwned;
use embassy_embedded_hal::shared_bus::{asynch::i2c::I2cDevice as I2cContainer, I2cDeviceError};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embedded_hal_async::i2c::I2c;
use esp_hal::{i2c::{Instance, I2C}, Async};

use crate::os::kernel::drivers::{errors::DeviceError, Device};

use super::InterfaceType;

pub trait I2cDevice<BUS: Instance + 'static>: Device {
    const DEVICE_ADDRESS: u8;
    fn get_bus(&mut self) -> &mut I2cContainer<'static, NoopRawMutex, I2C<'static, BUS, Async>>;
    fn register_shifted_by(register: &mut [u8], delta: u8) {
        register[0] += delta;
    }
    async fn write(&mut self, address: &[u8], data: &[u8])-> Result<(), DeviceError> {
        let mut output = address.to_owned();
        output.extend_from_slice(data);
        self.get_bus().write(Self::DEVICE_ADDRESS, &output).await
    }
    async fn write_read(&mut self, data: &[u8], buffer: &mut[u8]) -> Result<(), DeviceError> {
        self.get_bus().write_read(Self::DEVICE_ADDRESS, data, buffer).await
    }
    async fn read<const N: usize>(&mut self, address: &[u8]) -> Result<[u8; N], DeviceError> {
        let mut buffer = [0; N];
        self.write_read(address, &mut buffer).await?;
        Ok(buffer)
    }
}


