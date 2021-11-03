import {sum, is_dead} from './pkg'
import BN from 'bn.js'

const a = new BN(2 ** 26, 10)
const b = new BN(2 ** 26, 10)

const wasmResult = sum(a, b)
const bnResult = a.add(b)

console.log(`a = ${a}, b = ${b}`)
console.log(`sum(a, b) = ${wasmResult}`)
console.log(`a.add(b) = ${bnResult}`)

console.log('\n')

const hash = new BN('dead', 'hex')

console.log(`hash = ${hash}`)
console.log(`is_dead(hash) = ${is_dead(hash)}`)
