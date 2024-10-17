use alloc::vec::Vec;

pub enum ProcessState {
    Unused,
    Fetus,
    Eepy,
    Ready,
    Running,
    Zombie,
}

pub struct Process<'a> {
    state: ProcessState,
    memory: Vec<u8>,
    pid: u16,
    parent: Option<&'a Process<'a>>,
}
