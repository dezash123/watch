use self::drivers::{display::TextDisplay, imu::{Accelerometer, Gyroscope, Imu, Magnetometer}, misc::Thermometer};


pub mod drivers;
pub mod peripherals;
pub mod custom_kernels;

pub trait Kernel {
    fn get_text_display(&self) -> Option<&dyn TextDisplay>;
    fn get_thermometer(&self) -> Option<&dyn Thermometer>;
    fn get_accelerometer(&self) -> Option<&dyn Accelerometer>;
    fn get_gyroscope(&self) -> Option<&dyn Gyroscope>;
    fn get_magnetometer(&self) -> Option<&dyn Magnetometer>;
    fn get_imu(&self) -> Option<&dyn Imu>;
}
