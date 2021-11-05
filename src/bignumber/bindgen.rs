use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "@ethersproject/bignumber")]
extern "C" {
    pub type BigNumber;

    #[wasm_bindgen(static_method_of = BigNumber, js_name = from)]
    pub fn new(number: String) -> BigNumber;

    #[wasm_bindgen(static_method_of = BigNumber, js_name = from)]
    pub fn new_from_array(number: &[u8]) -> BigNumber;

    #[wasm_bindgen(method, getter, js_name = _hex)]
    pub fn hex(this: &BigNumber) -> String;
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use super::BigNumber;

    #[wasm_bindgen_test]
    fn new_then_hex() {
        assert_eq!(
            BigNumber::new("0x1e00".to_string()).hex(),
            "0x1e00".to_string()
        );
    }

    #[wasm_bindgen_test]
    fn new_from_array_then_hex() {
        assert_eq!(
            BigNumber::new_from_array(&[30, 0]).hex(),
            "0x1e00".to_string()
        );
    }
}
