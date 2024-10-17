pub trait I2cBackend {
    const ADDR: u8;
    pub fn read<const N: usize>(&mut self, addr: u8) -> [u8; N];
    pub fn write(&mut self, addr: u8, data: &[u8]);
}
