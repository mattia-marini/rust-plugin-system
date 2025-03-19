#[no_mangle]
pub extern "C" fn init_apis(apis_ptr: *const apis::ApisData) {
    apis::init_apis(apis_ptr);
}

#[no_mangle]
pub extern "C" fn main() {
    println!("Hello from plugin 1!");

    apis::api1();
    apis::api2();
}

//#[link(name = "my_plugin")]
//extern "C" {
//    fn add(a: u32, b: u32) -> u32;
//}
