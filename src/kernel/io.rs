use bitflags::bitflags;
use uom::si::ratio::percent;
use uom::si::Unit;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AxisRequest: u8 {
        const X = 1 << 7;
        const Y = 1 << 6;
        const Z = 1 << 5;

        const ThetaX = 1 << 4;
        const ThetaY = 1 << 3;
        const ThetaZ = 1 << 2;

        const Total = 1 << 1;

        const Now = 1;
    }
}

pub trait ThreeAxisSupplier<U: Unit> {
    fn get_x() -> Result<U>;
    fn get_y() -> Result<U>;
    fn get_z() -> Result<U>;

    fn service(request: AxisRequest<U>) {}
}

pub struct ThreeAxisData<U: Unit> {
    x: Option<U>,
    y: Option<U>,
    z: Option<U>,
    theta_x: Option<U>,
    theta_y: Option<U>,
    theta_z: Option<U>,
    total: Option<U>,
}

pub enum DataRequest {
    Position(u8),
    Velocity(u8),
    Acceleration(u8),
    AngualarVelocity(u8),
    MagneticFluxDensity(u8),

    Temperature,
    Sound,
    HeartRate,
    TouchEvent,
}

pub enum OutputData {
    Vibes(),
    FrameBuffer(),
    LED(percent),
}
