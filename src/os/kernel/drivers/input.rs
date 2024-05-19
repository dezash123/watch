pub trait Button {
    async fn get_status(&mut self) -> bool;
    async fn get_time_last_pressed(&mut self) -> f64;
}
