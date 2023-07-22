use wasm_bindgen::prelude::*;

#[wasm_bindgen(inline_js = "export function snippetTest() { console.log('Hello from JS FFI!'); }")]
extern "C"
{
    pub fn snippetTest();
}

#[wasm_bindgen]
extern "C"
{
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn console_log(_s: &str);
}
