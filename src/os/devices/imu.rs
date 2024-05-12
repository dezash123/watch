use nalgebra::{Vector2, Vector3};

pub struct Pose3d {
    pub position: Vector3<f32>,
    pub rotation: Vector2<f32>,
}

pub trait IMU {
    fn get_pose_estimate(&self) -> Pose3d;
}
