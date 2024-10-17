use core::convert::Infallible;

use alloc::vec::Vec;
use alloc::{borrow::ToOwned, boxed::Box};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::{PixelColor, Rgb888, RgbColor},
    Pixel,
};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::digital::Wait;
use embedded_hal_async::{
    delay::DelayNs,
    spi::{ErrorType, SpiDevice},
};
use heapless::Vec as HeaplessVec;
use thiserror_no_std::Error;

mod consts;
use consts::*;

pub trait Icna5300SPIWrapper {}

pub struct Icna5300<SPI: SpiDevice, TE: Wait, RST: OutputPin, RGB: PixelColor> {
    spi: SPI,
    tearing_enable: TE,
    reset: RST,
    color_type: RGB,
    write_queue: Vec<Box<[u8]>>,
}

#[derive(Error, Debug)]
pub enum Icna5300Error<SPI: ErrorType> {
    #[error("SPI transaction failed!")]
    Communication(#[from] SPI),
}

impl<SPI: SpiDevice, TE: Wait, RST: OutputPin, RGB: PixelColor> Icna5300<SPI, TE, RST, RGB>
where
    Icna5300Error<SPI>: From<SPI::Error>,
{
    pub async fn new<T: DelayNs>(
        spi: SPI,
        tearing_enable: TE,
        reset: RST,
        color_type: RGB,
        delay: &mut T,
    ) -> Result<Self, Icna5300Error<SPI>> {
        Self {
            spi,
            tearing_enable,
            reset,
            color_type,
            write_queue: Vec::new(),
        }
        .init(delay)
        .await
    }
    async fn init<T: DelayNs>(mut self, delay: &mut T) -> Result<Self, Icna5300Error<SPI>> {
        self.push_param_command(SET_CMD_PAGE, [0]);
        self.push_param_command(SET_SPI_MODE, [1 << 7]);
        self.push_param_command(COLMOD, [0b111 << 4 | 0b111]);
        self.push_param_command(TEON, [0]);
        self.push_param_command(WRCTRLD, [1 << 5]);
        self.push_param_command(WRDISBV, [0xFF]);
        self.push_param_command(WRHBMDISBV, [0xFF]);
        self.push_param_command(CASET, [0x00, 0x06, 0x01, 0xD7]); // 6 to 471 incl (466 px)
        self.push_param_command(RASET, [0x00, 0x00, 0x01, 0xD1]); // 0 to 465 incl (466 px)
        self.push_command(SLPOUT);
        self.flush().await?;
        delay.delay_ms(60).await;
        self.push_command(DISPON);
        self.push_command(ALLPON);
        self.flush().await?;
        Ok(self)
    }
    fn set_pixel_location(&mut self, pixel: Point) {
        let x: [u8; 2] = ((pixel.x - 6) as u16).to_be_bytes();
        let y: [u8; 2] = (pixel.y as u16).to_be_bytes();

        self.push_param_command(CASET, x);
        self.push_param_command(RASET, y);
    }
    fn first_color_write(&mut self, color: Rgb888) {
        self.push_param_command(RAMWR_START, [color.r(), color.g(), color.b()]);
    }
    #[inline]
    fn push_command(&mut self, command: u8) {
        self.write_queue
            .push(Box::new([0x02u8.to_be(), 0x00, command.to_be(), 0x00]));
    }
    #[inline]
    fn push_param_command<const N: usize>(&mut self, command: u8, parameters: [u8; N])
    where
        [(); N + 4]:,
    {
        let mut data: HeaplessVec<u8, { N + 4 }> =
            HeaplessVec::from_slice(&[0x02u8.to_be(), 0x00, command.to_be(), 0x00]).unwrap();
        data.extend_from_slice(&parameters).unwrap();
        self.write_queue
            .push(Box::<[u8; N + 4]>::new(data.into_array().unwrap()));
    }
    async fn flush(&mut self) -> Result<(), Icna5300Error<SPI>> {
        for transaction in self.write_queue.iter() {
            self.tearing_enable.wait_for_low().await.unwrap();
            self.spi.write(transaction).await.unwrap();
        }
        self.write_queue = Vec::new();
        Ok(())
    }
}

impl<SPI: SpiDevice, TE: Wait, RST: OutputPin, RGB: PixelColor> OriginDimensions
    for Icna5300<SPI, TE, RST, RGB>
{
    fn size(&self) -> Size {
        Size::new(466, 466)
    }
}

impl<SPI: SpiDevice, TE: Wait, RST: OutputPin> DrawTarget for Icna5300<SPI, TE, RST, Rgb888>
where
    Icna5300Error<SPI>: From<SPI::Error>,
{
    type Color = Rgb888;
    type Error = Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.set_pixel_location(pixel.0);
            self.first_color_write(pixel.1);
        }
        Ok(())
    }
}
