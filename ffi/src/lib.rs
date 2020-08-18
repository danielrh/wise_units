#![forbid(unused_imports)]
#![deny(unused_extern_crates)]
#![warn(
    future_incompatible,
    missing_copy_implementations,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications,
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::pedantic,
    clippy::perf,
    clippy::nursery,
    clippy::style
)]
// C bindings don't include the module name, so ffi functions will need it.
#![allow(clippy::module_name_repetitions)]

use std::os::raw::c_char;
use std::ptr;
pub mod measurement;
pub mod unit;

pub use wise_units::{Measurement, Unit};

fn set_error_and_return<T>(message: String) -> *const T {
    ffi_common::error::set_last_err_msg(&message);
    std::ptr::null()
}

unsafe fn return_string_in_buffer(string: String, buffer: *mut c_char, length: i32) -> i32 {
    if buffer.is_null() {
        ffi_common::error::set_last_err_msg(
            "Null pointer passed into return_string_in_buffer() as the buffer",
        );
        return -1;
    }

    let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

    if string.len() >= buffer.len() {
        ffi_common::error::set_last_err_msg(
            format!(
                "Buffer provided for writing the message is too small.
            Expected at least {} bytes but got {}",
                string.len() + 1,
                buffer.len()
            )
            .as_str(),
        );
        buffer[0] = 0;
        return (string.len() + 1) as i32;
    }

    ptr::copy_nonoverlapping(string.as_ptr(), buffer.as_mut_ptr(), string.len());

    // Add a trailing null so people using the string as a `char *` don't
    // accidentally read into garbage.
    buffer[string.len()] = 0;

    return string.len() as i32;
}
