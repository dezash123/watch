use nalgebra::Vector2;

use crate::os::devices::{accelerometer::{self, Accelerometer}, gyroscope::Gyroscope, imu::{Pose3d, IMU}, magnetometer::Magnetometer};

pub struct NineDOFIMU {
    accelerometer: &'static dyn Accelerometer,
    gyroscope: &'static dyn Gyroscope,
    magnetometer: &'static dyn Magnetometer,
}

impl IMU for NineDOFIMU {
    fn get_pose_estimate(&self) -> Pose3d {
        Pose3d {
            position: self.accelerometer.get_acceleration(),
            rotation: Vector2::new(0.0, 0.0)
        }
    }
}
