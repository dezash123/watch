use core::error::Error;
use anyhow::Result;
use embedded_graphics_core::{geometry::{OriginDimensions, Point, Size}, pixelcolor::Rgb888};
use embedded_hal::digital::OutputPin;
use embedded_hal_async::{delay::DelayNs, digital::Wait, spi::SpiDevice};
use heapless::Vec;
use embedded_graphics_core::pixelcolor::RgbColor;
use crate::consts::*;

pub struct Icna5300<SPI, TE, RST, TMR> {
    spi: SPI,
    tearing_enable: TE,
    reset: RST,
    delay: TMR,
}

impl<SPI: SpiDevice, TE: Wait, RST: OutputPin, TMR: DelayNs> Icna5300<SPI, TE, RST, TMR> 
    where
        SPI::Error: Send + Sync + Error + 'static
{
    pub async fn new(
        spi: SPI,
        tearing_enable: TE,
        reset: RST,
        delay: TMR,
    ) -> Result<Self> {
        Self {
            spi,
            tearing_enable,
            reset,
            delay,
        }
        .init()
        .await
    }
    async fn init(mut self) -> Result<Self> {
        self.send_param_command(SET_CMD_PAGE, [0]).await?;
        self.send_param_command(SET_SPI_MODE, [1 << 7]).await?;
        self.send_param_command(COLMOD, [0b111 << 4 | 0b111]).await?;
        self.send_param_command(TEON, [0]).await?;
        self.send_param_command(WRCTRLD, [1 << 5]).await?;
        self.send_param_command(WRDISBV, [0xFF]).await?;
        self.send_param_command(WRHBMDISBV, [0xFF]).await?;
        self.send_param_command(CASET, [0x00, 0x06, 0x01, 0xD7]).await?; // 6 to 471 incl (466 px)
        self.send_param_command(RASET, [0x00, 0x00, 0x01, 0xD1]).await?; // 0 to 465 incl (466 px)
        self.send_command(SLPOUT).await?;
        self.delay.delay_ms(60).await;
        self.send_command(DISPON).await?;
        self.send_command(ALLPON).await?;
        Ok(self)
    }
    async fn set_pixel_location(&mut self, pixel: Point) -> Result<()> {
        let x: [u8; 2] = ((pixel.x - 6) as u16).to_be_bytes();
        let y: [u8; 2] = (pixel.y as u16).to_be_bytes();

        self.send_param_command(CASET, x).await?;
        self.send_param_command(RASET, y).await
    }
    async fn first_color_write(&mut self, color: Rgb888) -> Result<()> {
        self.send_param_command(RAMWR_START, [color.r(), color.g(), color.b()]).await
    }
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
        let mut data: Vec<u8, { N + 4 }> =
            Vec::from_slice(&[0x02u8.to_be(), 0x00, command.to_be(), 0x00]).unwrap();
        data.extend_from_slice(&parameters).unwrap();
        self.spi.write(&data.into_array::<{N + 4}>().unwrap()).await?;
        Ok(())
    }
    // async fn flush(&mut self) -> Result<()> {
    //     Ok(())
    // }
}

impl<SPI: SpiDevice, TE: Wait, RST: OutputPin, TMR: DelayNs> OriginDimensions for Icna5300<SPI, TE, RST, TMR> {
    fn size(&self) -> Size {
        Size::new(466, 466)
    }
}

