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
    fn from_primitive_uint() {
        let value = U256::MAX - U256::from(U128::MAX);
        let bn = BN::from(value.clone());

        assert_eq!(TryInto::<U256>::try_into(bn).unwrap(), value);
    }

    #[wasm_bindgen_test]
    fn from_primitive_hash() {
        let value = H256::from(
            <[u8; 32]>::try_from([[0; 8], [u8::MAX; 8], [u8::MAX; 8], [0; 8]].concat()).unwrap(),
        );
        let bn = BN::from(value.clone());

        assert_eq!(TryInto::<H256>::try_into(bn).unwrap(), value);
    }

    #[wasm_bindgen_test]
    fn from_std() {
        let value = u128::MAX - u64::MAX as u128;
        let bn = BN::from(value.clone());

        assert_eq!(TryInto::<u128>::try_into(bn).unwrap(), value);
    }
}
