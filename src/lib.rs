mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn using_js_alert() {
    alert("Hello, rust-wasm-vite-react-ts!");
}

#[wasm_bindgen]
pub fn using_web_sys_console() {
    use web_sys::console;
    console::log_1(&"Hello using web-sys".into());
    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);
}
