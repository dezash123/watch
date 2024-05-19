use alloc::string::String;
use async_trait::async_trait;
use alloc::boxed::Box;

pub mod pcf8574t;

#[async_trait(?Send)]
pub trait TextDisplay {
    async fn clear(&mut self);
    async fn display_text(&mut self, text: String);
    async fn display_on_line(&mut self, text: String, line: u8);
}
