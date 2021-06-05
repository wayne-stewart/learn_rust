
// extern crate wasm_bindgen;
// use wasm_bindgen::prelude::*;

// import window.alert
// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// export a hello function
// #[wasm_bindgen]
// pub fn hello(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

// #[wasm_bindgen]
// pub fn get_name() -> &str {
//     "get name called from wasm"
// }

#[no_mangle]
pub extern "C" fn get_num() -> i32 {
    42
}

#[no_mangle]
pub extern "C" fn inc(num: i32) -> i32 {
    num + 1
}