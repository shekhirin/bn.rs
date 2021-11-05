use std::str::FromStr;

use primitive_types::{H128, H160, H256, H512, U128, U256, U512};

use super::{BigNumber, BigNumberError};

macro_rules! try_from_primitive_uint {
    ($type:ty) => {
        impl TryFrom<BigNumber> for $type {
            type Error = BigNumberError;

            fn try_from(value: BigNumber) -> Result<$type, Self::Error> {
                Ok(<$type>::from_str_radix(&value.hex(), 16)?)
            }
        }
    };
}

macro_rules! try_from_primitive_hash {
    ($type:ty) => {
        impl TryFrom<BigNumber> for $type {
            type Error = BigNumberError;

            fn try_from(value: BigNumber) -> Result<$type, Self::Error> {
                // TODO: fix $type.len() > value.hex().len() (e.g. H256 from 20-byte BigNumber)
                Ok(<$type>::from_str(&value.hex())?)
            }
        }
    };
}

macro_rules! try_from_std {
    ($type:ty) => {
        impl TryFrom<BigNumber> for $type {
            type Error = BigNumberError;

            fn try_from(value: BigNumber) -> Result<$type, Self::Error> {
                let hex = value.hex();

                if hex.starts_with('-') {
                    Ok(<$type>::from_str_radix(
                        &[&hex[..1], &hex[3..]].concat(),
                        16,
                    )?)
                } else {
                    Ok(<$type>::from_str_radix(&hex[2..], 16)?)
                }
            }
        }
    };
}

try_from_primitive_uint!(U128);
try_from_primitive_uint!(U256);
try_from_primitive_uint!(U512);
try_from_primitive_hash!(H128);
try_from_primitive_hash!(H160);
try_from_primitive_hash!(H256);
try_from_primitive_hash!(H512);
try_from_std!(isize);
try_from_std!(i8);
try_from_std!(i16);
try_from_std!(i32);
try_from_std!(i64);
try_from_std!(i128);
try_from_std!(usize);
try_from_std!(u8);
try_from_std!(u16);
try_from_std!(u32);
try_from_std!(u64);
try_from_std!(u128);

#[cfg(test)]
mod tests {
    use primitive_types::{H160, H256, U128, U256, U512};
    use wasm_bindgen_test::*;

    use super::{BigNumber, BigNumberError};

    #[wasm_bindgen_test]
    fn primitive_uint() {
        let bignumber = BigNumber::new(U256::MAX.to_string());
        let middle_bignumber = BigNumber::new((U256::MAX - U256::from(U128::MAX)).to_string());

        assert!(matches!(
            U128::try_from(BigNumber::from(bignumber.clone())).err(),
            Some(BigNumberError::PrimitiveUint(_))
        ));
        assert_eq!(
            U256::try_from(BigNumber::from(bignumber.clone())).unwrap(),
            U256::MAX
        );
        assert_eq!(
            U256::try_from(middle_bignumber).unwrap(),
            U256::MAX - U256::from(U128::MAX)
        );
        assert_eq!(U512::try_from(bignumber).unwrap(), U512::from(U256::MAX));
    }

    #[wasm_bindgen_test]
    fn primitive_hash() {
        let h256 = H256::from([u8::MAX; 32]);
        let bignumber = BigNumber::new(format!("{:#x}", h256));
        // let middle_h256 = H256::from(
        //     <[u8; 32]>::try_from([[0; 8], [u8::MAX; 8], [u8::MAX; 8], [0; 8]].concat()).unwrap(),
        // );
        // let middle_bignumber = BigNumber::new(format!("{:#x}", middle_h256));

        assert!(matches!(
            H160::try_from(BigNumber::from(bignumber.clone())).err(),
            Some(BigNumberError::PrimitiveHash(_))
        ));
        assert_eq!(
            H256::try_from(BigNumber::from(bignumber.clone())).unwrap(),
            H256::from([u8::MAX; 32]),
        );
        // assert_eq!(H256::try_from(middle_bignumber).unwrap(), middle_h256);
        // assert_eq!(
        //     H512::try_from(bignumber).unwrap(),
        //     H512::from(<[u8; 64]>::try_from([[0; 32], [u8::MAX; 32]].concat()).unwrap()),
        // );
    }

    #[wasm_bindgen_test]
    fn std_uint() {
        let bignumber = BigNumber::new(u64::MAX.to_string());
        let middle_u64 = u64::MAX - u32::MAX as u64;
        let middle_bignumber = BigNumber::new(middle_u64.to_string());

        assert!(matches!(
            u8::try_from(BigNumber::from(bignumber.clone())).err(),
            Some(BigNumberError::Std(_))
        ));
        assert_eq!(u64::try_from(middle_bignumber).unwrap(), middle_u64);
        assert_eq!(u128::try_from(bignumber).unwrap(), u64::MAX as u128);
    }

    #[wasm_bindgen_test]
    fn std_int() {
        let bignumber = BigNumber::new(i64::MIN.to_string());
        let middle_i64 = i64::MIN - i32::MIN as i64;
        let middle_bignumber = BigNumber::new(middle_i64.to_string());

        assert!(matches!(
            u8::try_from(BigNumber::from(bignumber.clone())).err(),
            Some(BigNumberError::Std(_))
        ));
        assert_eq!(i64::try_from(middle_bignumber).unwrap(), middle_i64);
        assert_eq!(i128::try_from(bignumber).unwrap(), i64::MIN as i128);
    }
}
