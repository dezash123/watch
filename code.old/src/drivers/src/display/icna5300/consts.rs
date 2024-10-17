pub const SET_CMD_PAGE: u8 = 0xFE; // set CMD page (CMD Page Switch)
pub const SET_SPI_MODE: u8 = 0xC4; //SPI mode select (SetSPIMode)
pub const COLMOD: u8 = 0x3A; // color mode select
pub const TEON: u8 = 0x35; // 0 = vsync, 1 = v+hsync
pub const WRCTRLD: u8 = 0x53; // en/disable brightness control
pub const WRDISBV: u8 = 0x51; // set brightness
pub const WRHBMDISBV: u8 = 0x51; // set brightness in HBM mode (?)
pub const CASET: u8 = 0x2A; // set column start address
pub const RASET: u8 = 0x2B; // set row start address
pub const SLPOUT: u8 = 0x11; // exit sleep mode
pub const DISPON: u8 = 0x29; //turn on display
pub const RAMWR_START: u8 = 0x2C;
pub const RAMWR_CONTINUOUS: u8 = 0x3C;
pub const ALLPON: u8 = 0x23;
pub const SPA_ID: u8 = 0x15;
pub const LPA_ID: u8 = 0x29;
