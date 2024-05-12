use nalgebra::Vector3;

pub trait Magnetometer {
    fn get_field(&self) -> Vector3<f32>;
}
