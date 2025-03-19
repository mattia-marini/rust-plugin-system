// End goal would be to have a semantic verioning system for APIs
// in x.0.0 breaking changes are allowed  versions
// in 0.x.0 versions adding apis is allowed , but old apis are not allowed to change
// in 0.0.x only fixes and transparent changes are allowed

// TODO have proc macro to decide which of the following functions are exported as apis
#[no_mangle]
pub extern "C" fn api1() {
    println!("API 1 called from plugin");
}

#[no_mangle]
pub extern "C" fn api2() {
    println!("API 2 called from plugin");
}

// TODO having macro to automatically define apis
pub const APIS: apis::ApisData = apis::ApisData {
    val: 42,
    api1,
    api2,
};
