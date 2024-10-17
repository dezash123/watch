use core::convert::Infallible;

extern crate alloc;

use embedded_graphics_core::pixelcolor::raw::ToBytes;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::{PixelColor, Rgb888, RgbColor},
    Pixel,
};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::spi::SpiDevice as BlockingSpiDevice;
use embedded_hal_async::digital::Wait;
use embedded_hal_async::spi::{ErrorType, SpiDevice};
use thiserror_no_std::Error;
use fugit::{MillisDurationU32, ExtU32};

use crate::consts::*;

pub trait Backend {
    fn write(&mut self, data: &[u8]);
    fn delay(&mut self, duration: MillisDurationU32);
}

pub struct Icna5300<B: Backend, TE: Wait, RST: OutputPin> {
    backend: B,
    tearing_enable: TE,
    reset: RST,
    color_type: Rgb888,
}

#[derive(Error, Debug)]
pub enum Icna5300Error<SPI: ErrorType> {
    #[error("SPI transaction failed!")]
    Communication(#[from] SPI),
}

impl<B: Backend, TE: Wait, RST: OutputPin> Icna5300<B, TE, RST> {

    const fn to_command<const N: usize>(address: u8, parameters: [u8; N]) -> [u8; N + 4]
    where [u8; N + 4]: {
        let mut data = [0; N + 4];
        data[0] = CMD_WRITE;
        data[2] = address;
        let mut nparams = 0;
        while nparams < N {
            data[nparams + 4] = parameters[nparams];
            nparams += 1;
        }
        data
    }
    fn push_command<const N: usize>(&mut self, address: u8, parameters: [u8; N]) 
    where [u8; N + 4]: {
        self.backend.write(&Self::to_command(address, parameters));
    }

    const fn prep_pixel(point: Point) -> ([u8; 4], [u8; 4]) {
        let x: [u8; 4] = ((point.x as u32 + X_START) << 16 | X_END).to_be_bytes();
        let y: [u8; 4] = ((point.y as u32) << 16 | Y_END).to_be_bytes();
        (x, y)
    }
    fn set_pixel_location(&mut self, point: Point) {
        let xy = Self::prep_pixel(point);
        self.push_command(CASET, xy.0);
        self.push_command(RASET, xy.1);
    }

    pub fn write_single_pixel(&mut self, pixel: Pixel<Rgb888>) {
        self.set_pixel_location(pixel.0);
        self.push_command(RAMWR_START, pixel.1.to_be_bytes());
    }

    const fn to_single_spi(data: u8) -> [u8; 4] {
        let data_u32 = data as u32;
        let mut output: u32 = 0;
        let mut bit: u8 = 8;
        while bit != 0 {
            bit -= 1;
            output <<= 3;
            output |= data_u32 & (1 << bit);
        }
        output.to_be_bytes()
    }
    pub fn init(&mut self) {
        self.backend.write(&[SET_SINGLE_SPI; 4]);
        self.backend.write(&Self::to_single_spi(SET_QUAD_SPI));
        self.push_command(SET_CMD_PAGE, [0]);
        self.push_command(SET_SPI_MODE, [1 << 7]);
        self.push_command(COLMOD, [0b111 << 4 | 0b111]);
        self.push_command(TEON, [0]);
        self.push_command(WRCTRLD, [1 << 5]);
        self.push_command(WRDISBV, [0xFF]);
        self.push_command(WRHBMDISBV, [0xFF]);
        self.push_command(CASET, [0x00, 0x06, 0x01, 0xD7]); // 6 to 471 incl (466 px)
        self.push_command(RASET, [0x00, 0x00, 0x01, 0xD1]); // 0 to 465 incl (466 px)
        self.push_command(SLPOUT, []);
        self.backend.delay(60.millis());
        self.push_command(DISPON, []);
        self.push_command(ALLPON, []);
    }
}

pub struct BlockingSpiBackend<SPI: BlockingSpiDevice, D: DelayNs> {
    pub spi: SPI,
    pub delay: D,
}

impl<SPI: BlockingSpiDevice, D: DelayNs> Backend for BlockingSpiBackend<SPI, D> {
    fn write(&mut self, data: &[u8]) {
        self.spi.write(data).unwrap();
    }
    fn delay(&mut self, duration: MillisDurationU32) {
        self.delay.delay_ms(duration.to_millis());
    }
}

impl<B: Backend, TE: Wait, RST: OutputPin> DrawTarget for Icna5300<B, TE, RST> {
    type Color = Rgb888;
    type Error = Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            self.write_single_pixel(pixel);
        }
        Ok(())
    }
}

impl<B: Backend, TE: Wait, RST: OutputPin> OriginDimensions
    for Icna5300<B, TE, RST>
{
    fn size(&self) -> Size {
        Size::new(466, 466)
    }
}





// impl<SPI: SpiDevice, TE: Wait, RST: OutputPin, RGB: PixelColor> Icna5300<SPI, TE, RST, RGB>
// where
//     Icna5300Error<SPI>: From<SPI::Error>,
// {
//     pub async fn new<D: Timer<u32>>(
//         spi: SPI,
//         tearing_enable: TE,
//         reset: RST,
//         color_type: RGB,
//     ) -> Result<Self, Icna5300Error<SPI>> {
//         Self {
//             spi,
//             tearing_enable,
//             reset,
//             color_type,
//             write_queue: Vec::new(),
//         }
//         .init::<D>()
//         .await
//     }
//     async fn init<D: Timer<u32>>(mut self) -> Result<Self, Icna5300Error<SPI>> {
//         self.push_param_command(SET_CMD_PAGE, [0]);
//         self.push_param_command(SET_SPI_MODE, [1 << 7]);
//         self.push_param_command(COLMOD, [0b111 << 4 | 0b111]);
//         self.push_param_command(TEON, [0]);
//         self.push_param_command(WRCTRLD, [1 << 5]);
//         self.push_param_command(WRDISBV, [0xFF]);
//         self.push_param_command(WRHBMDISBV, [0xFF]);
//         self.push_param_command(CASET, [0x00, 0x06, 0x01, 0xD7]); // 6 to 471 incl (466 px)
//         self.push_param_command(RASET, [0x00, 0x00, 0x01, 0xD1]); // 0 to 465 incl (466 px)
//         self.push_command(SLPOUT);
//         self.flush().await?;
//         D::delay_ms(60.millis()).await;
//         self.push_command(DISPON);
//         self.push_command(ALLPON);
//         self.flush().await?;
//         Ok(self)
//     }
//     async fn flush(&mut self) -> Result<(), Icna5300Error<SPI>> {
//         for transaction in self.write_queue.iter() {
//             self.tearing_enable.wait_for_low().await.unwrap();
//             self.spi.write(transaction).await.unwrap();
//         }
//         self.write_queue = Vec::new();
//         Ok(())
//     }
// }
// 
// impl<SPI: SpiDevice, TE: Wait, RST: OutputPin, RGB: PixelColor> OriginDimensions
//     for Icna5300<SPI, TE, RST, RGB>
// {
//     fn size(&self) -> Size {
//         Size::new(466, 466)
//     }
// }
// 
// impl<SPI: SpiDevice, TE: Wait, RST: OutputPin> DrawTarget for Icna5300<SPI, TE, RST, Rgb888>
// where
//     Icna5300Error<SPI>: From<SPI::Error>,
// {
//     type Color = Rgb888;
//     type Error = Infallible;
//     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
//     where
//         I: IntoIterator<Item = Pixel<Self::Color>>,
//     {
//         for pixel in pixels {
//             self.set_pixel_location(pixel.0);
//             self.first_color_write(pixel.1);
//         }
//         Ok(())
//     }
// }
// 
// 
