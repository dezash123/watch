pub trait I2cBackend {
    const ADDR: u8;
    fn read<const N: usize>(&mut self, addr: u8) -> [u8; N];
    fn write(&mut self, addr: u8, data: &[u8]);
}
