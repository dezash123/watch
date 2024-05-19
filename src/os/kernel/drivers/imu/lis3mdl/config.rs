use deku::deku_derive;

#[deku_derive]
#[deku(endian="big", bytes=5)]
pub struct Config {
    pub temp_en: bool,
    pub magnetometer_range: MagneticFieldStrengthRange,
}

// +/- range for magnetic field strength; gauss
#[deku_derive]
#[deku(endian="big", bits=2)]
pub enum MagneticFieldStrengthRange {
    G4,
    G8,
    G12,
    G16,
}

impl MagneticFieldStrengthRange {
    pub fn conversion_factor(range: &Self) -> f64 {
        match range {
            Self::G2 => 1f64 / 16834f64,
            Self::G4 => 1f64 / 8192f64,
            Self::G8 => 1f64 / 4096f64,
            Self::G16 => 1f64 / 2048f64,
        }
    }
}
