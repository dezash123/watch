use embedded_hal_async::digital::Wait;

pub enum ButtonState {}

pub trait Button {
    async fn get_state(&mut self);
    async fn wait_for_press(&mut self);
}

pub struct PinButton<P: Wait> {
    pin: P,
}

impl<P: Wait> Button for PinButton<P> {
    async fn get_state(&mut self);
    async fn wait_for_press(&mut self);
}
