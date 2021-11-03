# <h1 align="center"> bn.rs </h1>

**[bn.js](https://github.com/indutny/bn.js/) bindings for Rust & WebAssembly**

---

Write Rust code that uses [bn.js](https://github.com/indutny/bn.js/) numbers
```rust
use bn_rs::BN;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum(a: BN, b: BN) -> Result<BN, JsValue> {
    /// `BNError` implements `Into<JsValue>`, so we can use `?` here
    let a: u128 = a.try_into()?;
    let b: u128 = b.try_into()?;

    let result = a + b;

    Ok(result.into())
}
```

Call it from JavaScript
```javascript
import {sum} from './pkg'
import BN from 'bn.js'

const a = new BN(2, 10)
const b = new BN(2, 10)

console.log(sum(a, b)) // Call Rust code with BN passed and returned
```

## Run example
```shell
$ cd example
$ yarn
$ yarn start
```
