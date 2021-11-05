import {sum, is_dead} from './pkg'
import BN from 'bn.js'
import {BigNumber} from "@ethersproject/bignumber"

const a = BigNumber.from(2 ** 26)
const b = new BN(2 ** 26, 10)
const wasmResult = sum(a, b)
const bnResult = a.add(BigNumber.from(b.toString()))

console.log(`a = ${a}, b = ${b}`)
console.log(`sum(a, b) = ${wasmResult}`)
console.log(`a.add(b) = ${bnResult}`)
console.assert(wasmResult.toString() == bnResult.toString())

console.log()

const hash = new BN('dead', 'hex')
const dead = BigNumber.from('0xdead')
const isDeadResult = is_dead(hash, dead)

console.log(`hash = ${hash.toString('hex')}, dead = ${dead.toHexString()}`)
console.log(`is_dead(hash, dead) = ${isDeadResult}`)
console.assert(isDeadResult)
