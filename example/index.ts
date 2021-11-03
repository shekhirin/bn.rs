import {sum} from './pkg'
import BN from 'bn.js'
import * as util from "util"

const a = new BN(2 ** 26, 10)
const b = new BN(2 ** 26, 10)

const wasmResult = sum(a, b)
const bnResult = a.add(b)

console.log(`a = ${a}, b = ${b}`)
console.log(`sum(a, b) = ${wasmResult} = ${util.inspect(wasmResult, {customInspect: false})}`)
console.log(`a.add(b) = ${bnResult} = ${util.inspect(bnResult, {customInspect: false})}`)
console.log(`sum(a, b) == a.add(b) = ${wasmResult == bnResult}`)
