use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum BigNumberError {
    #[error(transparent)]
    Std(#[from] std::num::ParseIntError),
    #[error(transparent)]
    PrimitiveUint(#[from] uint::FromStrRadixErr),
    #[error(transparent)]
    PrimitiveHash(#[from] rustc_hex::FromHexError),
}

impl From<BigNumberError> for JsValue {
    fn from(err: BigNumberError) -> Self {
        format!("{:?}", err).into()
    }
}
