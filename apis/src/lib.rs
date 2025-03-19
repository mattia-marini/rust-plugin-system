#![allow(unused)]
#[no_mangle] // Prevent Rust from renaming the function
pub fn api() {
    println!("API 1");
}

#[repr(C)]
#[derive(Clone)]
pub struct Apis {
    pub val: i32,
    pub api1: extern "C" fn(),
    pub api2: extern "C" fn(),
}
