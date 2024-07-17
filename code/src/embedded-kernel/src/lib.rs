#![no_std]
#![no_main]
// enable this in a bit
// #![warn(missing_docs)]

extern crate alloc;

pub mod allocator;
pub mod config;
pub mod drivers;
pub mod peripherals;
