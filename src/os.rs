extern crate alloc;

use esp_hal::entry;
use core::mem::MaybeUninit;
use alloc::collections::VecDeque;
use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::macros::main;
use esp_println::println;

use self::{apps::daemon::Daemon, kernel::custom_kernels::v1kernel::V1Kernel};

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

pub mod kernel;
pub mod apps;

pub struct Os<'a> {
    kernel: V1Kernel,
    daemons: VecDeque<&'a dyn Daemon>,
    // current_program: Program,
}

impl<'a> Os<'a> {
    async fn new() -> Self {
        let kernel = V1Kernel::new().await;
         
        Self {
            kernel,
            daemons: VecDeque::new(),
        }
    }
}

#[main]
async fn start(_spawner: Spawner) {
    init_heap();
    
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
}

