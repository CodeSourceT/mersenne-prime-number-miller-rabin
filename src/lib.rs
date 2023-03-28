extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::One;
use std::str::FromStr;

fn mersenne_number(n: u32) -> BigUint {
    (BigUint::one()<< n) - BigUint::one()
}

pub fn is_prime(n_str: &str) -> bool {
    let n = BigUint::from_str(n_str).unwrap();

    if n < BigUint::from(2u32) {
        return false;
    }

    if n == BigUint::from(2u32) || n == BigUint::from(3u32) || n == BigUint::from(5u32) {
        return true;
    }

    if (&n % BigUint::from(2u32)) == BigUint::from(0u32) {
        return false;
    }

    let n_sub = n.clone() - BigUint::from(1u32);
    let mut exponent = n_sub.clone();
    let mut trials = 0;

    while (&exponent % BigUint::from(2u32)) == BigUint::from(0u32) {
        exponent /= 2u32;
        trials += 1;
    }

    'LOOP: for i in 1..((n.to_string().len()) + 2) {
        let mut result = bmodpow(&(BigUint::from(2u32) + i), &exponent, &n);

        if result == BigUint::from(1u32) || result == n_sub {
            continue;
        }

        for _ in 1..trials {
            result = result.pow(2) % &n;

            if result == BigUint::from(1u32) {
                return false;
            }

            if result == n_sub {
                continue 'LOOP;
            }
        }

        return false;
    }

    true
}

fn bmodpow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    if *base == BigUint::from(0u32) {
        return match *exponent == BigUint::from(0u32) {
            true => BigUint::from(1u32),
            false => BigUint::from(0u32),
        };
    }

    if *modulus == BigUint::from(1u32) {
        return BigUint::from(0u32);
    }

    let exponent_in_binary = exponent.to_radix_le(2);
    let mut my_base = base.clone();
    let mut result = BigUint::from(1u32);

    for next_bit in exponent_in_binary {
        if next_bit == 1 {
            result = (result * my_base.clone()) % modulus;
        }

        my_base = my_base.pow(2) % modulus;
    }

    result
}

#[cfg(test)]
mod test;