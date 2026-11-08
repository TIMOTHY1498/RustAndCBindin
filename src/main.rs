use std::ffi::CString;
use std::os::raw::c_char;

unsafe extern "C" {
    fn printFromRust(passedValue: *const c_char) -> i32;
}

fn main() {
    let value = "Hello from Rust!";
    let c_value = CString::new(value).expect("CString::new failed");
    unsafe {
        printFromRust(c_value.as_ptr());
    }
}