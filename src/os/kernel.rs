use self::drivers::{display::TextDisplay, errors::DeviceError, imu::{Accelerometer, Gyroscope, Imu, Magnetometer}, misc::Thermometer, Device};
use thiserror_no_std::Error;

pub mod drivers;
pub mod interfaces;
pub mod custom_kernels;

#[async_trait(?Send)]
pub trait Kernel {
    async fn new() -> Result<Self, KernelError>;
    async fn manage_inputs(&mut self) -> Result<(), KernelError>;
    async fn manage_outputs(&mut self) -> Result<(), KernelError>;
}

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("Kernel setup failed due to {0}")]
    Setup(SetupError),
}

#[derive(Debug, Error)]
pub enum SetupError {
    Device(#[from] DeviceError),
}
