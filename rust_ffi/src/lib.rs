#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn concatenate_strings(s1: *const c_char, s2: *const c_char) -> *mut c_char {
    // println!("s1: {:?}, s2: {:?}", s1, s2);

    // Convert C strings to Rust strings
    let c_str1 = unsafe { CStr::from_ptr(s1) };
    let c_str2 = unsafe { CStr::from_ptr(s2) };
    // println!("c_str1: {:?}, c_str2: {:?}", c_str1, c_str2);

    // Convert Rust strings to owned strings
    let rust_str1 = c_str1.to_string_lossy().into_owned();
    let rust_str2 = c_str2.to_string_lossy().into_owned();

    // Concatenate strings
    let result = rust_str1 + &rust_str2;

    // Convert the result back to a C string
    let c_string = CString::new(result).expect("Failed to create CString");
    // println!("c_string: {:?}", c_string);

    // Leak the CString to ensure it lives long enough to be used from other languages
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_concatenated_string(s: *mut c_char) {
    // Convert the C string back to a CString to free the memory
    unsafe {
        if s.is_null() {
            return;
        }
        let _ = CString::from_raw(s);
    }
}

#[no_mangle]
pub extern "C" fn print_string(s: *const c_char) {
    // Convert the C string to a Rust string
    let c_str = unsafe { CStr::from_ptr(s) };
    let rust_str = c_str.to_string_lossy().into_owned();

    // Print the Rust string
    println!("{}", rust_str);
}

#[no_mangle]
pub extern "C" fn get_string() -> *mut c_char{
    // Create a Rust string
    const RUST_STR: &str = "Hello from Rust!";

    // Convert the Rust string to a CString
    let c_string = CString::new(RUST_STR).expect("Failed to create CString");

    // Leak the CString to ensure it lives long enough to be used from other languages
    c_string.into_raw()
}

static STRING: &str = "Hello World!\0";

#[no_mangle]
extern "C" fn ffi_string() -> *const u8 {
  STRING.as_ptr()
}
