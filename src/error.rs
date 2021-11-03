use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum BNError {
    #[error("cannot convert negative BN to uint")]
    NegativeUint,
    #[error("BN doesn't fit into {0}")]
    Overflow(String),
    #[error("BN threw {0}")]
    Throw(String),
}

impl From<BNError> for JsValue {
    fn from(err: BNError) -> Self {
        format!("{:?}", err).into()
    }
}
