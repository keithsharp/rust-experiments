use std::ffi::CString;
use std::os::raw::c_char;

static PLUGIN_NAME: &'static str = "Test Plugin";

#[no_mangle]
pub extern "C" fn plugin_name() -> *mut c_char {
    let s = CString::new(PLUGIN_NAME).unwrap();
    s.into_raw()
}

#[no_mangle]
pub fn plugin_name_len() -> usize {
    PLUGIN_NAME.len()
}
