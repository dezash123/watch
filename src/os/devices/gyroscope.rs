use nalgebra::Vector3;

pub trait Gyroscope {
    fn get_angular_velocity(&self) -> Vector3<f32>;
}
