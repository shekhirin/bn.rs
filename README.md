# <h1 align="center"> bn.rs </h1>

**[bn.js](https://github.com/indutny/bn.js/)
and [ethers.js BigNumber](https://github.com/ethers-io/ethers.js/tree/master/packages/bignumber) bindings for Rust &
WebAssembly with [primitive-types](https://github.com/paritytech/parity-common/tree/master/primitive-types) support**

---

Write Rust code that uses bn.js and ethers.js BigNumber numbers

```rust
use primitive_types::{H160, U128};
use wasm_bindgen::prelude::*;

use bn_rs::{BigNumber, BN};

#[wasm_bindgen]
pub fn sum(a: BigNumber, b: BN) -> Result<BigNumber, JsValue> {
    // `BigNumberError` and `BNError` implement `Into<JsValue>`, so we can use `?` here

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
```

Call it from JavaScript

```javascript
import {sum, is_dead} from './pkg'
import BN from 'bn.js'
import {BigNumber} from "@ethersproject/bignumber"

// Initialize bn.js and ethers.js BigNumber numbers
const a = BigNumber.from(2 ** 26)
const b = new BN(2 ** 26, 10)
const hash = new BN('dead', 'hex')
const dead = BigNumber.from('0xdead')

// Call Rust code with both BN and BigNumber passed and returned
console.log(sum(a, b)) // == BigNumber.from(4)
console.log(is_dead(hash, dead)) // == true
```

## Run example

```shell
$ cd example
$ yarn
$ yarn start
```
