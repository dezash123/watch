use esp_hal::i2c::Instance;

#[derive(Debug)]
pub struct Chsc6x<N: Instance> {}

pub struct TouchEvent {
    x: u16,
    y: u16,
    flag: TouchFlag,
    id: i32,
}

pub enum TouchFlag {
    Down,
    Up,
    Contact,
}
