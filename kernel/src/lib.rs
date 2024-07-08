#![no_std]
#![no_main]
// enable this in a bit
// #![warn(missing_docs)]

use core::alloc::{GlobalAlloc, Layout};

pub mod config;
pub mod drivers;
pub mod peripherals;
pub mod virtualization;

extern crate alloc;
