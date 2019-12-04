use wasm_bindgen::prelude::*;
use web_sys::console;
use std::string::String;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn hello() -> String {
    String::from("Hello")
}
#[wasm_bindgen]
pub struct PathFinder {

}
