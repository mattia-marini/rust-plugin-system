#[no_mangle]
pub extern "C" fn main(apis: &apis::Apis) {
    println!("Hello from plugin 2!");
    // println!("{:?}", apis.val);
    // println!("{:?}", apis);
    (apis.api1)();
    (apis.api2)();
}

//#[link(name = "my_plugin")]
//extern "C" {
//    fn add(a: u32, b: u32) -> u32;
//}
