#![allow(unused)]

#[repr(C)]
pub struct Apis {
    pub val: i32,
    pub api1: extern "C" fn(),
    pub api2: extern "C" fn(),
}
