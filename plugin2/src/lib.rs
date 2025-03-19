#[no_mangle]
pub extern "C" fn init_apis(apis_ptr: *const apis::ApisData) {
    apis::init_apis(apis_ptr);
}

#[no_mangle]
pub extern "C" fn main() {
    println!("Hello from plugin 2!");

    apis::api1();
    apis::api2();
}
