use uom::si::i8::{Acceleration, Velocity};

pub struct SandProgram {

}

struct SandSimulation<const L: u16, const W: u16> {
    sim_field: [[SimPoint; L]; W],
    g: Acceleration,
}

enum SimPoint {
    Sand(Velocity),

}
