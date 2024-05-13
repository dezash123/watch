use alloc::{boxed::{self, Box}, collections::VecDeque};
use embedded_hal::i2c;
use esp_hal::{clock::Clocks, gpio::{InputPin, OutputPin}, i2c::{Instance, I2C}, peripheral::Peripheral, Async};
use fugit::HertzU32;

pub struct I2cRequest<'a> {
    device_address: u8,
    request_type: I2cRequestType<'a>,
    status: I2cRequestStatus,
}

pub enum I2cRequestType<'a> {
    Read(&'a [u8], &'a mut [u8]),
    Write(&'a [u8]),
}

pub enum I2cRequestStatus {
    Waiting,
    Fulfilled,
    Error(Box<dyn i2c::Error>),
}

pub struct I2cContainer<'a, N: Instance> {
    i2c: I2C<'a, N, Async>,
    request_buffer: VecDeque<I2cRequest<'a>>,
}

impl<'d, N: Instance> I2cContainer<'d, N> {
    pub fn new<SDA: OutputPin+ InputPin, SCL: OutputPin + InputPin>(
        i2c: impl Peripheral<P = N> + 'd,
        sda: impl Peripheral<P = SDA> + 'd,
        scl: impl Peripheral<P = SCL> + 'd,
        frequency: HertzU32,
        clocks: &Clocks,
    ) -> Self {
        let i2c = I2C::new_async(
            i2c,
            sda,
            scl,
            frequency,
            clocks,
        );
        Self {
            i2c,
            request_buffer: VecDeque::new(),
        }
    }
    pub async fn read(&mut self, device_address: u8, memory_address: &'d [u8], buffer: &'d mut [u8]) {
       self.request_buffer.push_back(
            I2cRequest{
                 device_address,
                 request_type: I2cRequestType::Read(memory_address, buffer),
                 status: I2cRequestStatus::Waiting,
            }
        );
    }
    pub async fn service_request(&mut self, mut request: I2cRequest) {
        match {
            match request.request_type {
            I2cRequestType::Read(a, b) => self.i2c.write_read(request.device_address, a, b).await,
            I2cRequestType::Write(d) => self.i2c.write(request.device_address, d).await,
        }
        } {
            Ok(()) => request.status = I2cRequestStatus::Fulfilled,
            Err(e) => request.status = I2cRequestStatus::Error(Box::new(e)),
        }
    }
}
