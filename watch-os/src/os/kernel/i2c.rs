use alloc::{boxed::Box, collections::VecDeque};
use core::ptr::addr_of;
use esp_hal::{clock::Clocks, gpio::{InputPin, OutputPin}, i2c::{Instance, I2C}, peripheral::Peripheral, peripherals::{I2C0, I2C1}, Async};
use embedded_hal::i2c::{self, Operation};
use fugit::HertzU32;

pub static mut I2C0_Request_Queue: VecDeque<I2CRequest> = VecDeque::new();

pub static mut I2C1_Request_Queue: VecDeque<I2CRequest> = VecDeque::new();

pub struct I2CRequest<'a> {
    device_address: u8,
    memory_address: u8,
    request_type: Operation<'a>, 
    request_status: &'a mut I2CRequestStatus,
}

pub enum I2CRequestStatus {
    Waiting,
    Fulfilled,
    Error(Box<dyn i2c::Error>),
}

pub trait I2CDevice {
    async fn create_request();
}

pub struct I2CBus<'a, N: Instance> {
    i2c: I2C<'a, N, Async>,
    request_buffer: &'a VecDeque<I2CRequest<'a>>,
}

impl<'d> I2CBus<'d, I2C0> {
    pub fn new<SDA: OutputPin + InputPin, SCL: OutputPin + InputPin>(
        i2c: impl Peripheral<P = I2C0> + 'd,
        sda: impl Peripheral<P = SDA> + 'd,
        scl: impl Peripheral<P = SCL> + 'd,
        frequency: HertzU32,
        clocks: &Clocks,
    ) -> Self {
        let i2c_dev = I2C::new_async(
            i2c,
            sda,
            scl,
            frequency,
            clocks,
        );
        let request_buffer = &I2C0_Request_Queue;
        Self {
            i2c: i2c_dev,
            request_buffer,
        }
    }
}

impl<'d> I2CBus<'d, I2C1> {
    pub fn new<SDA: OutputPin + InputPin, SCL: OutputPin + InputPin>(
        i2c: impl Peripheral<P = I2C1> + 'd,
        sda: impl Peripheral<P = SDA> + 'd,
        scl: impl Peripheral<P = SCL> + 'd,
        frequency: HertzU32,
        clocks: &Clocks,
    ) -> Self {
        let i2c_dev = I2C::new_async(
            i2c,
            sda,
            scl,
            frequency,
            clocks,
        );
        let request_buffer = &I2C1_Request_Queue;
        Self {
            i2c: i2c_dev,
            request_buffer,
        }
    }
}

impl<'d, N: Instance> I2CBus<'d, N> {
}
