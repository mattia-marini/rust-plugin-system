#![allow(unused)]
use std::sync::OnceLock;

static APIS_DATA: OnceLock<&ApisData> = OnceLock::new();

#[repr(C)]
pub struct ApisData {
    pub val: i32,
    pub api1: extern "C" fn(),
    pub api2: extern "C" fn(),
}

impl ApisData {
    pub fn new(val: i32, api1: extern "C" fn(), api2: extern "C" fn()) -> Self {
        Self { val, api1, api2 }
    }
}

pub fn init_apis(apis_ptr: *const ApisData) {
    unsafe {
        APIS_DATA.set(&*apis_ptr);
    }
}

pub fn api1() {
    (APIS_DATA.get().unwrap().api1)();
}

pub fn api2() {
    (APIS_DATA.get().unwrap().api2)();
}
