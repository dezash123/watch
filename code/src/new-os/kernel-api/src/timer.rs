use fugit::{Duration, MillisDuration, NanosDuration};

pub trait Timer<T> {
    async fn delay<const NOM: u32, const DENOM: u32>(duration: Duration<T, NOM, DENOM>);
    async fn delay_ns(duration: NanosDuration<T>);
    async fn delay_ms(duration: MillisDuration<T>);
}
