use primitive_types::{H160, U128};
use wasm_bindgen::prelude::*;

use bn_rs::{BigNumber, BN};

#[wasm_bindgen]
pub fn sum(a: BigNumber, b: BN) -> Result<BigNumber, JsValue> {
    // Both `BigNumberError` and `BNError` implement `Into<JsValue>`, so we can use `?` here

    let a = i128::try_from(a)?; // std uints are supported
    let b: U128 = b.try_into()?; // primitive-types uints supported too

    let result = a + b.as_u128() as i128;

    Ok(result.into())
}

#[wasm_bindgen]
pub fn is_dead(target_hash: BN, dead_hash: BigNumber) -> Result<bool, JsValue> {
    // primitive-types hashes supported too
    let hash = H160::try_from(target_hash)?;
    let dead = H160::try_from(dead_hash)?;

    Ok(hash == dead)
}
