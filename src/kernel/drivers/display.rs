use alloc::string::String;
use async_trait::async_trait;
use alloc::boxed::Box;

use crate::os::kernel::interfaces::Interface;

use super::Device;

pub mod pcf8574t;

#[async_trait(?Send)]
pub trait Display<I: Interface>: Device<I> {
    
}

#[async_trait(?Send)]
pub trait TextDisplay: Device {
    async fn clear(&mut self);
    async fn display_text(&mut self, text: String);
    async fn display_on_line(&mut self, text: String, line: u8);
}
