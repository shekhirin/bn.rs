use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "bn.js")]
extern "C" {
    pub type BN;

    #[wasm_bindgen(constructor)]
    pub fn new(number: String, base: u32) -> BN;

    #[wasm_bindgen(constructor)]
    pub fn new_from_array(number: &[u8], base: u32) -> BN;

    #[wasm_bindgen(method, getter)]
    pub fn negative(this: &BN) -> u8; // positive = 0, negative = 1

    #[wasm_bindgen(method, js_name = byteLength)]
    pub fn byte_length(this: &BN) -> u32;

    #[wasm_bindgen(catch, method, js_name = toArray)]
    pub fn to_array(this: &BN, endian: String, length: u32) -> Result<Box<[u8]>, JsValue>;
}
