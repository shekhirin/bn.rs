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

    #[wasm_bindgen(method)]
    pub fn toString(this: &BN, base: u32) -> String;
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use super::BN;

    #[wasm_bindgen_test]
    fn new() {
        assert_eq!(BN::new("10".into(), 10).toString(10), "10".to_string());
        assert_eq!(BN::new("dead".into(), 16).toString(16), "dead".to_string());
    }

    #[wasm_bindgen_test]
    fn new_from_array() {
        assert_eq!(
            BN::new_from_array(&[0, 10, 0], 10).toString(10),
            "2560".to_string()
        );
        assert_eq!(
            BN::new_from_array(&[30, 0], 16).toString(16),
            "1e00".to_string()
        );
    }

    #[wasm_bindgen_test]
    fn negative() {
        assert_eq!(BN::new("10".into(), 10).negative(), 0);
        assert_eq!(BN::new("-10".into(), 10).negative(), 1);
    }

    #[wasm_bindgen_test]
    fn byte_length() {
        assert_eq!(BN::new("10".into(), 10).byte_length(), 1);
        assert_eq!(BN::new("10000".into(), 10).byte_length(), 2);
    }

    #[wasm_bindgen_test]
    fn to_array() {
        assert_eq!(
            *BN::new("10000".into(), 10)
                .to_array("le".into(), 5)
                .unwrap(),
            [16, 39, 0, 0, 0]
        );
        assert_eq!(
            *BN::new("10000".into(), 10)
                .to_array("be".into(), 5)
                .unwrap(),
            [0, 0, 0, 39, 16]
        );
    }

    #[wasm_bindgen_test]
    fn to_string() {
        assert_eq!(*BN::new("10".into(), 10).toString(10), "10".to_string());
        assert_eq!(*BN::new("dead".into(), 16).toString(16), "dead".to_string());
    }
}
