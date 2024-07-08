use crate::peripherals::spi::{QuadSpi, SpiBus};

pub struct Co5300<QPI: SpiBus<dyn QuadSpi>> {
    bus: QPI,
}

impl<QPI: SpiBus<dyn QuadSpi>> Co5300<QPI> {}
