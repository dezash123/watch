use async_trait::async_trait;
use alloc::boxed::Box;

use super::{Device, DeviceError};

pub trait Config {
}

#[async_trait(?Send)]
pub trait ConfigurableDevice<T: Config>: Device {
    async fn configure(&mut self) -> Result<(), DeviceError>;
}
