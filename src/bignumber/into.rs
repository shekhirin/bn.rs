use primitive_types::{H128, H160, H256, H512, U128, U256, U512};

use super::BigNumber;

macro_rules! from {
    ($type:ty) => {
        impl From<$type> for BigNumber {
            fn from(value: $type) -> Self {
                BigNumber::new(format!("{:#x}", value))
            }
        }
    };
}

from!(U128);
from!(U256);
from!(U512);
from!(H128);
from!(H160);
from!(H256);
from!(H512);
from!(isize);
from!(i8);
from!(i16);
from!(i32);
from!(i64);
from!(i128);
from!(usize);
from!(u8);
from!(u16);
from!(u32);
from!(u64);
from!(u128);

#[cfg(test)]
mod tests {
    use primitive_types::{H256, U128, U256};
    use wasm_bindgen_test::*;

    use super::BigNumber;

    #[wasm_bindgen_test]
    fn primitive_uint() {
        let value = U256::MAX - U256::from(U128::MAX);
        let bignumber = BigNumber::from(value.clone());

        assert_eq!(U256::try_from(bignumber).unwrap(), value);
    }

    #[wasm_bindgen_test]
    fn primitive_hash() {
        let value = H256::from(
            <[u8; 32]>::try_from([[u8::MAX; 8], [0; 8], [0; 8], [u8::MAX; 8]].concat()).unwrap(),
        );
        let bignumber = BigNumber::from(value.clone());

        assert_eq!(H256::try_from(bignumber).unwrap(), value);
    }

    #[wasm_bindgen_test]
    fn std_uint() {
        let value = u128::MAX - u64::MAX as u128;
        let bignumber = BigNumber::from(value.clone());

        assert_eq!(u128::try_from(bignumber).unwrap(), value);
    }

    #[wasm_bindgen_test]
    fn std_int() {
        let value = i128::MAX - i64::MAX as i128;
        let bignumber = BigNumber::from(value.clone());

        assert_eq!(i128::try_from(bignumber).unwrap(), value);
    }
}
