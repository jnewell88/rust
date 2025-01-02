//! This is a library that provides utilities for math operations
//! So far it only adds two numbers together
//! # Examples:
//! ```
//! use math_utils::add;
//! let input = add(1,1);
//! ```
//! # Panics:
//! The `add` function will panic if it fails to add the two numbers together.
//!```
//! use math_utils::factorial;
//! let input = factorial(2);
//! ```
//! # Panics:
//! The `factorial` function will panic if it fails to find the factorial of the number provided.
//!```
//! use math_utils::gcd;
//! let input = gcd(18,24);
//! ```
//! # Panics:
//! The `gcd` function will panic if it fails to find the greatest common divisor of the two numbers.
//!```
//! use math_utils::prime;
//! let input = prime(17);
//! ```
//! # Panics:
//! The `prime` function will panic if the input number is invalid.
//!

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        1
    } else {
        num * factorial(num - 1)
    }
}

pub fn gcd(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        // loop while right is not 0
        let temp = left; // store left into temp location.
        left = right; // set right number = left.
        right = temp % right; // find what number left is divisible into right.
    }
    left // return final number stored in left when right == 0
}

pub fn prime(num: u64) -> bool {
    if num <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false; // If divisible by any number other than 1 and itself, it's not prime
        }
    }
    true // If not divisible by any number, it's prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_numbers() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn find_factorial() {
        let result = factorial(2);
        assert_eq!(result, 2);
    }

    #[test]
    fn find_gcd() {
        let result = gcd(18, 24);
        assert_eq!(result, 6)
    }

    #[test]
    fn is_prime() {
        let result = prime(17);
        assert_eq!(result, true);
    }
}
