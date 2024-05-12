use alloc::collections::VecDeque;

use self::{apps::daemon::Daemon, kernel::Kernel};


pub mod kernel;
pub mod drivers;
pub mod devices;
pub mod apps;

pub struct Os<'a> {
    kernel: Kernel<'static>,
    daemons: VecDeque<&'a dyn Daemon>,
    // current_program: Program,
}

impl<'a> Os<'a> {
    fn new() -> Self {
        Self {
            kernel: Kernel::new(),
            daemons: VecDeque::new(),
        }
    }
}
