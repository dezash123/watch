use esp_hal::{clock::Clocks, gpio::{InputPin, OutputPin}, i2c::Instance, peripheral::Peripheral, spi::{master::Spi, IsFullDuplex, SpiMode}};
use fugit::{HertzU32, MicrosDuration};

pub struct QSPI<'a, T: Instance, M: IsFullDuplex> {
    spi: Spi<'a, T, M>
}

impl<'b, T: Instance, M: IsFullDuplex> QSPI<'b, T, M> {
    fn new<SCK: OutputPin, MOSI: OutputPin + InputPin, MISO: OutputPin + InputPin, SIO2: OutputPin + InputPin, SIO3: OutputPin + InputPin, CS: OutputPin> (
    spi: impl Peripheral<P = T> + 'b,
    frequency: HertzU32,
    mode: SpiMode,
    clocks: &Clocks<'_>,
    sck: Option<impl Peripheral<P = SCK> + 'b>,
    mosi: Option<impl Peripheral<P = MOSI> + 'b>,
    miso: Option<impl Peripheral<P = MISO> + 'b>,
    sio2: Option<impl Peripheral<P = SIO2> + 'b>,
    sio3: Option<impl Peripheral<P = SIO3> + 'b>,
    cs: Option<impl Peripheral<P = CS> + 'b>
) -> Self {
        let mut spi = Spi::new(
            spi,
            frequency,
            mode,
            clocks,
        );
        Self {
            spi: spi.with_pins(
                sck,
                mosi,
                miso,
                sio2,
                sio3,
                cs,
            )
        }
    }
}
