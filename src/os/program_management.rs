use alloc::boxed::Box;

use crate::os::ProgramCommand;

use super::kernel::drivers::sensor_data::SensorData;

pub trait Program {
    const DEPENDENCIES: [SensorData];
    fn start() -> Self;
    fn give_data(&mut self) -> Result<(), ProgramError>;
    fn mainloop(&mut self, command: ProgramCommand) -> ProgramStatus;
}

pub enum ProgramError {

}

pub enum ProgramStatus {
    KillMe,
    Good,
}
