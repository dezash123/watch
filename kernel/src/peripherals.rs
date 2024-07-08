pub mod gpio;
pub mod i2c;
pub mod i2s;
pub mod spi;

pub trait DataBus {
    type BusParameters;
    type BusData;
}

pub trait SingleDeviceBus: DataBus {
    fn read(&mut self, paramaters: Self::BusParameters) -> Self::BusData;
    fn write(&mut self, parameters: Self::BusParameters, data: Self::BusData);
}

pub trait MultiDeviceBus: DataBus {
    fn read(&self, paramaters: Self::BusParameters) -> Self::BusData;
    fn write(&self, parameters: Self::BusParameters, data: Self::BusData);
}
