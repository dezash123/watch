use core::ffi::{c_int, c_short, c_uint, c_ushort};

#[no_mangle]
pub extern "C" fn i2cRead(id: c_uint, lenth: c_ushort, p_data: &[u8]) -> c_int {}
