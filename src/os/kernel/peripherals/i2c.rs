use embassy_embedded_hal::shared_bus::{asynch::i2c::I2cDevice, I2cDeviceError};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embedded_hal_async::i2c::I2c;
use esp_hal::{i2c::{Error, Instance, I2C}, Async};

pub trait I2cMethods<BUS: Instance + 'static> {
    fn get_bus(&mut self) -> &mut I2cDevice<'static, NoopRawMutex, I2C<'static, BUS, Async>>;
    fn get_device_address() -> u8;
    async fn write(&mut self, address: u8, data: u8)-> Result<(), I2cDeviceError<Error>> {
        self.get_bus().write(Self::get_device_address(), &[address, data]).await
    }
    async fn write_read(&mut self, data: &[u8], buffer: &mut[u8]) -> Result<(), I2cDeviceError<Error>> {
        self.get_bus().write_read(Self::get_device_address(), data, buffer).await
    }
    async fn read<const N: usize>(&mut self, address: u8) -> Result<[u8; N], I2cDeviceError<Error>> {
        let mut buffer = [0; N];
        self.write_read(&[address], &mut buffer).await?;
        Ok(buffer)
    }
}
