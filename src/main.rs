#![no_std]
#![no_main]
#![feature(future_join)]
#![feature(generic_const_exprs)]
#![allow(clippy::unusual_byte_groupings)]

extern crate alloc;

mod util;
mod os;
mod tests;
mod apps;
