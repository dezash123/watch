use alloc::string::String;

pub trait TextDisplay {
   async fn clear(&mut self);
   async fn display_text(&mut self, text: String);
   async fn display_on_line(&mut self, text: String, line: u8);
}
