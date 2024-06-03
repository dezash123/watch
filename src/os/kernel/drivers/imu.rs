use alloc::boxed::Box;
use async_trait::async_trait;
use esp_hal::i2c::{self, Error, Instance};
use nalgebra::{Vector2, Vector3};

use crate::{os::kernel::interfaces::CommunicationError, util::conversions::{be_bytes_to_f64, single_be_to_f64}};

use super::{DeviceError, I2cSensor16Bit};

// pub mod mpu6050;
pub mod lis3mdl;
// pub mod lsm6dsv16x;

#[async_trait(?Send)]
pub trait Sensor3Axis {
    fn get_conversion_factor(&self) -> f64;
    async fn get_all_axes(&mut self) -> Result<Vector3<f64>, DeviceError>;
    async fn get_x(&mut self) -> Result<f64, DeviceError>;
    async fn get_y(&mut self) -> Result<f64, DeviceError>;
    async fn get_z(&mut self) -> Result<f64, DeviceError>;
    async fn get_xy(&mut self) -> Result<Vector2<f64>, DeviceError>;
    async fn get_yz(&mut self) -> Result<Vector2<f64>, DeviceError>;
    fn x_std_dev(&self) -> f64;
    fn y_std_dev(&self) -> f64;
    fn z_std_dev(&self) -> f64;
}

#[async_trait(?Send)]
pub trait I2c3AxisSensor16Bit<N: Instance + 'static>: Sensor3Axis + I2cSensor16Bit<N> {
    const DATA_OUT_START: [u8];
    async fn get_all_axes(&mut self) -> Result<Vector3<f64>, DeviceError> {
        let bytes: [u8; 6] = self.read(&Self::DATA_OUT_START).await?;
        Ok(Vector3::from(be_bytes_to_f64(bytes, self.get_conversion_factor())))
    }
    async fn get_two_axes(&mut self, start_address: &[u8]) -> Result<Vector2<f64>, DeviceError> {
        let bytes: [u8; 4] = self.read(&Self::DATA_OUT_START).await?;
        Ok(Vector2::from([single_be_to_f64([bytes[0], bytes[1]]) * self.get_conversion_factor(), single_be_to_f64([bytes[2], bytes[3]]) * self.get_conversion_factor()]))
    }
    async fn get_x(&mut self) -> Result<f64, DeviceError> {
        self.read_single_be(&Self::DATA_OUT_START, self.get_conversion_factor()).await
    }
    async fn get_y(&mut self) -> Result<f64, DeviceError>{
        self.read_single_be(Self::register_shifted_by(&mut [Self::get_data_start()], 2u8), self.get_conversion_factor()).await
    }
    async fn get_z(&mut self) -> Result<f64, DeviceError> {
        self.read_single_be(Self::register_shifted_by(&mut [Self::get_data_start()], 4u8), self.get_conversion_factor()).await
    }
    async fn get_xy(&mut self) -> Result<Vector2<f64>, DeviceError> {
        self.get_two_axes(Self::get_data_start()).await
    }
    async fn get_yz(&mut self) -> Result<Vector2<f64>, DeviceError> {
        self.get_two_axes(Self::get_data_start() + 2u8).await
    }
}

#[async_trait(?Send)]
pub trait Imu {
}

#[async_trait(?Send)]
pub trait Accelerometer: Sensor3Axis {
    async fn get_acceleration(&mut self) -> Result<Vector3<f64>, DeviceError> {
        self.get_all_axes().await
    }
}

#[async_trait(?Send)]
pub trait Gyroscope: Sensor3Axis {
    async fn get_angular_velocity(&mut self) -> Result<Vector3<f64>, DeviceError> {
        self.get_all_axes().await
    }
}

#[async_trait(?Send)]
pub trait Magnetometer: Sensor3Axis {
    async fn get_field(&mut self) -> Result<Vector3<f64>, DeviceError> {
        self.get_all_axes().await
    }
}


