pub mod imu;
pub mod input;
pub mod display;
pub mod misc;

pub trait Config {
    fn default(&self) -> Self;
}

pub enum DeviceError {

}

pub trait Device {
    async fn enable(&mut self) -> Result<(), DeviceError>;
    async fn enabled(&self) -> bool;
    async fn disable(&mut self) -> Result<(), DeviceError>;
}
