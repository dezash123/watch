use esp_hal::entry;
use core::mem::MaybeUninit;
use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::macros::main;
use esp_println::println;

use self::kernel::{custom_kernels::v1_kernel::V1Kernel, Kernel};

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
pub mod program_management;

pub enum ProgramCommand {
    KysNow,
    Kys,
}

pub trait Error {}

pub struct Os<T: Kernel> {
    kernel: T,

    // current_program: Program,
}

impl<T: Kernel> Os<T> {
    async fn init() -> Self {
        let kernel = T::new().await.unwrap();
        Self {
            kernel,
        }
    }
    async fn start(&mut self) -> ! {
        loop {
            self.kernel.manage_inputs().await.unwrap();
            
            self.kernel.manage_outputs().await.unwrap();
        }
    }
}

#[main]
async fn start(_spawner: Spawner) -> ! {
    init_heap();
    esp_println::logger::init_logger_from_env();
    log::info!("Logger is setup");
    println!("Hello world!");
    Os::init().start()
}

