use primitive_types::{H128, H160, H256, H512, U128, U256, U512};

use super::BN;

macro_rules! from_primitive {
    ($type:ty) => {
        impl From<$type> for BN {
            fn from(value: $type) -> Self {
                let bytes = value
                    .0
                    .into_iter()
                    .rev()
                    .flat_map(|e| e.to_be_bytes())
                    .collect::<Vec<_>>();

                BN::new_from_array(bytes.as_slice(), 10)
            }
        }
    };
}

macro_rules! from_std {
    ($type:ty) => {
        impl From<$type> for BN {
            fn from(value: $type) -> Self {
                BN::new_from_array(&value.to_be_bytes(), 10)
            }
        }
    };
}

from_primitive!(U128);
from_primitive!(U256);
from_primitive!(U512);
from_primitive!(H128);
from_primitive!(H160);
from_primitive!(H256);
from_primitive!(H512);
// from_std!(isize);
// from_std!(i8);
// from_std!(i16);
// from_std!(i32);
// from_std!(i64);
// from_std!(i128);
from_std!(usize);
from_std!(u8);
from_std!(u16);
from_std!(u32);
from_std!(u64);
from_std!(u128);

#[cfg(test)]
mod tests {
    use primitive_types::{H256, U128, U256};
    use wasm_bindgen_test::*;

    use super::BN;

    #[wasm_bindgen_test]
    fn primitive_uint() {
        let value = U256::MAX - U256::from(U128::MAX);
        let bn = BN::from(value.clone());

        assert_eq!(U256::try_from(bn).unwrap(), value);
    }

    #[wasm_bindgen_test]
    fn primitive_hash() {
        let value = H256::from(
            <[u8; 32]>::try_from([[u8::MAX; 8], [0; 8], [0; 8], [u8::MAX; 8]].concat()).unwrap(),
        );
        let bn = BN::from(value.clone());

        assert_eq!(H256::try_from(bn).unwrap(), value);
    }

    // #[wasm_bindgen_test]
    // fn std_int() {
    //     let value = i128::MAX - i64::MAX as i128;
    //     let bn = BN::from(value.clone());
    //
    //     assert_eq!(i128::try_from(bn).unwrap(), value);
    // }

    #[wasm_bindgen_test]
    fn std_uint() {
        let value = u128::MAX - u64::MAX as u128;
        let bn = BN::from(value.clone());

        assert_eq!(u128::try_from(bn).unwrap(), value);
    }
}
