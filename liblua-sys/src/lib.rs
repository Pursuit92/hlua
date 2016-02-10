#![allow(non_camel_case_types)]

mod all;
mod macros;

pub use all::*;
pub use macros::*;

use std::ptr;
use std::ffi::CStr;

pub type c_void = ::std::os::raw::c_void;
pub type c_int = ::std::os::raw::c_int;

// called whenever lua encounters an unexpected error.
pub unsafe extern "C" fn panic(lua: *mut lua_State) -> c_int {
    let err = lua_tolstring(lua, -1, ptr::null_mut());
    let err = CStr::from_ptr(err);
    let err = String::from_utf8(err.to_bytes().to_vec()).unwrap();
    panic!("PANIC: unprotected error in call to Lua API ({})\n", err);
}
