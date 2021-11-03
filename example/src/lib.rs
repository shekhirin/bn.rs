use std::str::FromStr;

use primitive_types::{H160, U128};
use wasm_bindgen::prelude::*;

use bn_rs::BN;

#[wasm_bindgen]
pub fn sum(a: BN, b: BN) -> Result<BN, JsValue> {
    // `BNError` implements `Into<JsValue>`, so we can use `?` here
    let a = u128::try_from(a)?; // std uints are supported
    let b: U128 = b.try_into()?; // primitive-types uints supported too

    let result = a + b.as_u128();

    Ok(result.into())
}

#[wasm_bindgen]
pub fn is_dead(hash: BN) -> Result<bool, JsValue> {
    let hash = H160::try_from(hash)?; // primitive-types hashes supported too
    let dead = H160::from_str("0x000000000000000000000000000000000000dead").unwrap();

    Ok(hash == dead)
}
