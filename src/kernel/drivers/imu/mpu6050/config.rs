pub struct Config {
    pub accelerometer_range: AccelerationRange,
    pub gyro_range: AngularVelocityRange,
}

// +/- range for accelerometer readings; g's
pub enum AccelerationRange {
    G2 = 0 << 4,
    G4 = 1 << 4,
    G8 = 2 << 4,
    G16 = 3 << 4,
}

impl AccelerationRange {
    pub fn conversion_factor(range: &Self) -> f64 {
        match range {
            Self::G2 => 1f64 / 16834f64,
            Self::G4 => 1f64 / 8192f64,
            Self::G8 => 1f64 / 4096f64,
            Self::G16 => 1f64 / 2048f64,
        }
    }
}

// +/- range for gyroscope readings; degrees per second
pub enum AngularVelocityRange {
    Dps250 = 0 << 4,
    Dps500 = 1 << 4,
    Dps1000 = 2 << 4,
    Dps2000 = 3 << 4,
}

impl AngularVelocityRange {
    pub fn conversion_factor(range: &Self) -> f64 {
        match range {
            Self::Dps250 => 1f64 / 131f64,
            Self::Dps500 => 1f64 / 65.5f64,
            Self::Dps1000 => 1f64 / 32.8f64,
            Self::Dps2000 => 1f64 / 16.4f64,
        }
    }
}
