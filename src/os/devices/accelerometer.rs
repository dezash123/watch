use nalgebra::Vector3;

pub trait Accelerometer {
    fn get_acceleration(&self) -> Vector3<f32>;
}
