#[cfg(feature = "ethers-js")]
pub use bignumber::{BigNumber, BigNumberError};
#[cfg(feature = "bn-js")]
pub use bn::{BNError, BN};

mod bignumber;
mod bn;
