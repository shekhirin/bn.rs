use js_sys::JsString;
use primitive_types::{H128, H160, H256, H512, U128, U256, U512};

use crate::{bn::BN, error::BNError};

macro_rules! try_into {
    ($type:ty, $byte_length:expr, $from_fn:ident) => {
        impl TryInto<$type> for BN {
            type Error = BNError;

            fn try_into(self) -> Result<$type, Self::Error> {
                if self.negative() == 1 {
                    return Err(BNError::NegativeUint);
                }

                if self.byte_length() > $byte_length {
                    return Err(BNError::Overflow(stringify!($type).into()));
                }

                let bytes = <[u8; $byte_length as usize]>::try_from(
                    &*self
                        .to_array("be".into(), $byte_length)
                        .map_err(|err| BNError::Throw(JsString::from(err).into()))?,
                )
                .unwrap();

                Ok(<$type>::$from_fn(bytes))
            }
        }
    };
}

macro_rules! try_into_primitive {
    ($type:ty) => {
        try_into!($type, std::mem::size_of::<$type>() as u32, from);
    };
}

macro_rules! try_into_std {
    ($type:ty) => {
        try_into!($type, <$type>::BITS / 8, from_be_bytes);
    };
}

try_into_primitive!(U128);
try_into_primitive!(U256);
try_into_primitive!(U512);
try_into_primitive!(H128);
try_into_primitive!(H160);
try_into_primitive!(H256);
try_into_primitive!(H512);
// try_into_std!(isize);
// try_into_std!(i8);
// try_into_std!(i16);
// try_into_std!(i32);
// try_into_std!(i64);
// try_into_std!(i128);
try_into_std!(usize);
try_into_std!(u8);
try_into_std!(u16);
try_into_std!(u32);
try_into_std!(u64);
try_into_std!(u128);

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use primitive_types::{H160, H256, H512, U128, U256, U512};
    use wasm_bindgen_test::*;

    use crate::{error::BNError, BN};

    #[wasm_bindgen_test]
    fn try_into_primitive_uint() {
        let bn = BN::new(U256::MAX.to_string(), 10);
        let middle_bn = BN::new((U256::MAX - U256::from(U128::MAX)).to_string(), 10);

        assert!(matches!(
            TryInto::<U128>::try_into(BN::from(bn.clone())).err(),
            Some(BNError::Overflow(_))
        ));
        assert_eq!(
            TryInto::<U256>::try_into(BN::from(bn.clone())).unwrap(),
            U256::MAX
        );
        assert_eq!(
            TryInto::<U256>::try_into(middle_bn).unwrap(),
            U256::MAX - U256::from(U128::MAX)
        );
        assert_eq!(
            TryInto::<U512>::try_into(bn).unwrap(),
            U512::from(U256::MAX)
        );
    }

    #[wasm_bindgen_test]
    fn try_into_primitive_hash() {
        let h256 = H256::from([u8::MAX; 32]);
        let bn = BN::new(format!("{:x}", h256), 16);
        let middle_h256 = H256::from(
            <[u8; 32]>::try_from([[0; 8], [u8::MAX; 8], [u8::MAX; 8], [0; 8]].concat()).unwrap(),
        );
        let middle_bn = BN::new(format!("{:x}", middle_h256), 16);

        assert!(matches!(
            TryInto::<H160>::try_into(BN::from(bn.clone())).err(),
            Some(BNError::Overflow(_))
        ));
        assert_eq!(
            TryInto::<H256>::try_into(BN::from(bn.clone())).unwrap(),
            H256::from([u8::MAX; 32]),
        );
        assert_eq!(TryInto::<H256>::try_into(middle_bn).unwrap(), middle_h256,);
        assert_eq!(
            TryInto::<H512>::try_into(bn).unwrap(),
            H512::from(<[u8; 64]>::try_from([[0; 32], [u8::MAX; 32]].concat()).unwrap()),
        );
    }

    #[wasm_bindgen_test]
    fn try_into_std() {
        let bn = BN::new(u64::MAX.to_string(), 10);
        let middle_u64 = u64::MAX - u32::MAX as u64;
        let middle_bn = BN::new(middle_u64.to_string(), 10);

        assert!(matches!(
            TryInto::<u8>::try_into(BN::from(bn.clone())).err(),
            Some(BNError::Overflow(_))
        ));
        assert_eq!(TryInto::<u64>::try_into(middle_bn).unwrap(), middle_u64);
        assert_eq!(TryInto::<u128>::try_into(bn).unwrap(), u64::MAX as u128);
    }
}
