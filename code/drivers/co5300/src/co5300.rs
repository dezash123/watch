use core::error::Error;
use anyhow::Result;
use embedded_graphics_core::{geometry::{OriginDimensions, Point, Size}, pixelcolor::{Rgb565, Rgb888}};
use embedded_graphics_framebuf::FrameBuf;
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, digital::Wait, spi::SpiDevice};
use heapless::Vec;
use embedded_graphics_core::pixelcolor::RgbColor;
use crate::consts::*;

pub trait SupportedColor {}
impl SupportedColor for Rgb888 {}
impl SupportedColor for Rgb565 {}


pub struct Co5300<SPI, TE, RST, TMR, RGB> {
    spi: SPI,
    tearing_enable: TE,
    reset: RST,
    delay: TMR,
    colormode: RGB,
}

const fn convert_4wire_to_1wire(four_wire: u8) -> [u8; 4] {
        let four_wire_u32 = four_wire as u32;
        let mut output: u32 = 0;
        let mut bit: u8 = 8;
        while bit != 0 {
            bit -= 1;
            output <<= 3;
            output |= four_wire_u32 & (1 << bit);
        }
        output.to_be_bytes()
}

impl<SPI: SpiDevice, TE: Wait, RST: OutputPin, TMR: DelayNs, RGB: SupportedColor> Co5300<SPI, TE, RST, TMR, RGB> 
    where
        SPI::Error: Send + Sync + Error + 'static
{
    pub async fn new(spi: SPI, tearing_enable: TE, reset: RST, delay: TMR, colormode: RGB) -> Result<Self> {
        Self { spi, tearing_enable, reset, delay, colormode }.init().await
    }
    async fn init(mut self) -> Result<Self> {
        self.set_4wire().await?;

        self.send_command(C_SLPOUT).await?;
        self.delay.delay_ms(SLPOUT_DELAY_MS).await;

        self.send_param_command(SET_CMD_PAGE, [0]).await?;

        // self.send_param_command(WC_TEARON, [0x00]).await?;
        
        self.send_param_command(W_SPIMODECTL, [1 << 7]).await?;

        // self.send_param_command(W_MADCTL, MADCTL_COLOR_ORDER).await?; // RGB/BGR

        // self.send_param_command(W_PIXFMT, [0x55]).await?; // Interface Pixel Format 16bit/pixel
        // self.send_param_command(W_PIXFMT, [0x66]).await?; // Interface Pixel Format 18bit/pixel
        self.send_param_command(W_PIXFMT, [0x77]).await?; // Interface Pixel Format 24bit/pixel

        self.send_param_command(W_WCTRLD1, [1 << 5]).await?; // en/disable brightness control
        self.send_param_command(W_WDBRIGHTNESSVALHBM, [0xFF]).await?;

        self.send_param_command(W_CASET, [0x00, 0x06, 0x01, 0xD7]).await?; // 6 to 471 incl (466 px)
        self.send_param_command(W_PASET, [0x00, 0x00, 0x01, 0xD1]).await?; // 0 to 465 incl (466 px)

        self.send_command(C_DISPON).await?;

        self.send_param_command(W_WCE, [Contrast::ContrastOff as u8]).await?;

        Ok(self)
    }

    async fn set_1wire(&mut self) -> Result<()> {
        self.spi.write(&[SET_SINGLE_SPI; 4]).await?;
        Ok(())
    }
    
    async fn set_4wire(&mut self) -> Result<()> {
        self.set_1wire().await?;
        self.spi.write(&[SET_QUAD_SPI]).await?;
        Ok(())
    }

    // async fn set_pixel_location(&mut self, pixel: Point) -> Result<()> {
    //     let x: [u8; 2] = ((pixel.x - 6) as u16).to_be_bytes();
    //     let y: [u8; 2] = (pixel.y as u16).to_be_bytes();

    //     self.send_param_command(CASET, x).await?;
    //     self.send_param_command(RASET, y).await
    // }
    // async fn first_color_write(&mut self, color: Rgb888) -> Result<()> {
    //     self.send_param_command(RAMWR_START, [color.r(), color.g(), color.b()]).await
    // }

    #[inline]
    async fn send_command(&mut self, command: u8) -> Result<()> {
        self.spi.write(&[0x02u8.to_be(), 0x00, command.to_be(), 0x00]).await?;
        Ok(())
    }
    #[inline]
    async fn send_param_command<const N: usize>(&mut self, command: u8, parameters: [u8; N]) -> Result<()>
    where
        [u8; N + 4]:,
    {
        let mut data: Vec<u8, { N + 4 }> = Vec::from_slice(&[0x02u8.to_be(), 0x00, command.to_be(), 0x00]).unwrap();
        data.extend_from_slice(&parameters).unwrap();
        self.spi.write(&data.into_array::<{N + 4}>().unwrap()).await?;
        Ok(())
    }
    // async fn flush(&mut self) -> Result<()> {
    //     Ok(())
    // }
}

impl<SPI, TE, RST, TMR, RGB> OriginDimensions for Co5300<SPI, TE, RST, TMR, RGB> {
    fn size(&self) -> Size {
        Size::new(466, 466)
    }
}



