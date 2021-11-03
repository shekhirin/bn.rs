use bn_rs::BN;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum(a: BN, b: BN) -> Result<BN, JsValue> {
    let a = u128::try_from(a)?;
    let b: u128 = b.try_into()?;

    let result = a + b;

    Ok(result.into())
}
