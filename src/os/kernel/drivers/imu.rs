use alloc::boxed::Box;
use async_trait::async_trait;
use embassy_embedded_hal::shared_bus::I2cDeviceError;
use esp_hal::i2c;
use nalgebra::Vector3;

pub mod mpu6050;
// pub mod lis3mdl;
// pub mod lsm6dsv16x;

pub enum ImuError {
    Accelerometer(AccelerometerError),
    Gyroscope(GyroscopeError),
    Magnetometer(MagnetometerError),
}

pub enum AccelerometerError {
    I2c(I2cDeviceError<i2c::Error>),
}

pub enum GyroscopeError {
    I2c(I2cDeviceError<i2c::Error>),
}

pub enum MagnetometerError {
    I2c(I2cDeviceError<i2c::Error>),
}

pub enum GpsError {
    I2c(I2cDeviceError<i2c::Error>),
}

impl From<I2cDeviceError<i2c::Error>> for AccelerometerError {
    fn from(value: I2cDeviceError<i2c::Error>) -> Self { Self::I2c(value) }
}

impl From<I2cDeviceError<i2c::Error>> for GyroscopeError {
    fn from(value: I2cDeviceError<i2c::Error>) -> Self { Self::I2c(value) }
}

impl From<I2cDeviceError<i2c::Error>> for MagnetometerError {
    fn from(value: I2cDeviceError<i2c::Error>) -> Self { Self::I2c(value) }
}

impl From<I2cDeviceError<i2c::Error>> for GpsError {
    fn from(value: I2cDeviceError<i2c::Error>) -> Self { Self::I2c(value) }
}

#[async_trait(?Send)]
pub trait Imu {
}

#[async_trait(?Send)]
pub trait Accelerometer {
    async fn get_acceleration(&mut self) -> Result<Vector3<f64>, AccelerometerError>;
}

#[async_trait(?Send)]
pub trait Gyroscope {
    async fn get_angular_velocity(&mut self) -> Result<Vector3<f64>, GyroscopeError>;
}

#[async_trait(?Send)]
pub trait Magnetometer {
    async fn get_field(&self) -> Result<Vector3<f64>, MagnetometerError>;
}
