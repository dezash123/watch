use super::{DataBus, SingleDeviceBus};

pub trait SpiType {}

pub trait DualSpi: SpiType {}

pub trait QuadSpi: SpiType {}

pub trait OctalSpi: SpiType {}

pub trait SpiBus<T: SpiType + ?Sized> {}

pub struct SingleDeviceSpiBus {}
