extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::One;
use std::str::FromStr;

/// Generate a Mersenne number.
///
/// Generate a Mersenne number base on formula (2**n)-1
/// where n is index of Mersenne number
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use mersenne_prime_number_miller_rabin::mersenne_number;
/// 
/// let a_mersenne_number = mersenne_number(61);
/// ```
pub fn mersenne_number(n: u32) -> BigUint {
    (BigUint::one()<< n) - BigUint::one()
}

/// Check if a number is probably prime
///
/// This function use Miller-Rabin algorithme for check if a number is probably prime
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use num_bigint::BigUint;
/// use std::str::FromStr;
/// use mersenne_prime_number_miller_rabin::is_prime;
/// 
/// let n = BigUint::from_str("13").unwrap();
/// let is_prime = is_prime(n);
/// ```
pub fn is_prime(n: BigUint) -> bool {
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

/// Check if a number is probably prime from a string
///
/// This function use Miller-Rabin algorithme for check if a number is probably prime
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use mersenne_prime_number_miller_rabin::is_prime_from_str;
/// 
/// let is_prime = is_prime_from_str("13");
/// ```
pub fn is_prime_from_str(n_str: &str) -> bool {
    let n = BigUint::from_str(n_str).unwrap();
    return is_prime(n);    
}

/// Performs modulus division on a number raised to the power of another number.
///
/// Performs modulus division on a number raised to the power of another number.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use num_bigint::BigUint;
/// use mersenne_prime_number_miller_rabin::bmodpow;
/// 
/// let base = BigUint::from(0u32);
/// let expo = BigUint::from(3 as u32);
/// let module = BigUint::from(1 as u32);
/// let result = bmodpow(&base, &expo, &module);
/// ```
pub fn bmodpow(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
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