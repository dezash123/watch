#![no_std]
#![no_main]
#![feature(future_join)]
#![feature(generic_const_exprs)]
#![allow(clippy::unusual_byte_groupings)]

extern crate alloc;

mod kernel;
mod os_programs;
mod process_management;
mod tests;
mod util;
