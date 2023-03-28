# Mersenne Prime - Rust
This package it's develop to study Mersenne prime number. It hava functions for generate Mersenne number and check if it's probabily prime with Miller-Rabin Algorithm.
## Mersenne Number
Numbers of the form $M_n=2^n-1$ are called **Mersenne numbers**. If $M_n$ is prime, $n$ is also prime, but the converse is not true. The smallest counterexample is $M_{11}=2047=23\cdot 89$. 

[Mersenne prime \- Wikipedia](https://en.wikipedia.org/wiki/Mersenne_prime)

## Miller-Rabin Algorithm
Miller-Rabin is an algorithm check if a number is probabily prime or composite.
[Miller-Rabin Algorithm \- Wikipedia](https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test)
## Package
### Documentation
Check on crate.io
### Install
Put this code on your Cargo.toml
```rust
[dependencies]
mersenne-prime-number-miller-rabin = "0.1.0"
```
### Exemple
Let's see a basic exemple
```rust
use mersenne_prime_number_miller_rabin::is_prime;
use mersenne_prime_number_miller_rabin::mersenne_number;

let one_mersenne_number = mersenne_number(61);
let is_prime_result = is_prime(one_mersenne_number);
println!("Is prime ? {} ", is_prime_result);
```