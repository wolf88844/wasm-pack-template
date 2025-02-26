mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init(){
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn run(){
    greet();
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, {{project-name}}!");
}
