use super::{Device, DeviceError};

pub trait Config {}

pub trait Configurable<T: Config>: Device {
    fn default(&mut self) -> Result<(), DeviceError>;
    fn configure(&mut self, config: T) -> Result<(), DeviceError>;
}
