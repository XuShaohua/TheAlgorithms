// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use std::collections::HashMap;

pub type Counter = HashMap<u64, usize>;

#[must_use]
pub fn get_factors(mut n: u64) -> Counter {
    let mut factors = Counter::new();
    if n == 1 {
        //factors.insert(1, 1);
        return factors;
    }

    let mut factor = 2;
    if n == factor {
        factors.insert(factor, 1);
        return factors;
    }

    while n > 1 {
        debug_assert!(n >= factor);

        let mut count = 0;
        while n % factor == 0 {
            n /= factor;
            count += 1;
        }
        if count > 0 {
            factors.insert(factor, count);
        }
        factor += 1;
    }

    factors
}

/// Returns prime factors of num as a list.
#[must_use]
pub fn get_prime_factors(mut num: u64) -> Vec<u64> {
    let mut pf = Vec::new();
    if num == 0 {
        return pf;
    }
    while num % 2 == 0 {
        pf.push(2);
        num /= 2;
    }

    for i in (3..=((num as f64).sqrt() as u64)).skip(2) {
        while num % i == 0 {
            pf.push(i);
            num /= i;
        }
    }
    if num > 2 {
        pf.push(num);
    }
    pf
}

/// Returns prime factors of num as a list.
#[must_use]
pub fn get_prime_factors2(mut num: u64) -> Vec<u64> {
    let mut factors = vec![];
    if num == 0 {
        return factors;
    }
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            num /= i;
            factors.push(i);
        } else {
            i += 1;
        }
    }
    if num > 2 {
        factors.push(num);
    }
    factors
}

/// Returns all available factors/divisors of a possitive integer.
#[must_use]
pub fn get_factor_list(num: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    if num < 1 {
        return factors;
    }
    factors.push(1);
    if num == 1 {
        return factors;
    }

    factors.push(num);
    for i in 2..=((num as f64).sqrt() as u64) {
        // If i is a factor of num
        if num % i == 0 {
            factors.push(i);

            // num//i is the other factor of num
            let d: u64 = num / i;
            // If d and i are distinct
            if d != i {
                factors.push(d);
            }
        }
    }
    factors.sort_unstable();
    factors
}

/// Calculate Number of Divisors of an Integer.
#[must_use]
pub fn num_of_divisors(num: u64) -> usize {
    get_factor_list(num).len()
}

/// Calculate Sum of Divisors.
#[must_use]
pub fn sum_of_divisors(num: u64) -> u64 {
    get_factor_list(num).iter().sum()
}

/// Calculate Euler's Phi Function.
#[must_use]
pub fn euler_phi(num: u64) -> u64 {
    let mut pf = get_prime_factors(num);
    pf.dedup();
    let mut s = num as f64;
    for x in &pf {
        s *= (x - 1) as f64 / *x as f64;
    }

    s as u64
}

#[cfg(test)]
mod tests {
    use super::{
        euler_phi, get_factor_list, get_factors, get_prime_factors, get_prime_factors2,
        num_of_divisors, Counter,
    };

    #[test]
    fn test_get_factors() {
        assert_eq!(get_factors(45), Counter::from([(3, 2), (5, 1)]));
        assert_eq!(
            get_factors(2520),
            Counter::from([(2, 3), (3, 2), (5, 1), (7, 1)])
        );
    }

    #[test]
    fn test_get_factor_list() {
        assert_eq!(get_factor_list(1), vec![1]);
        assert_eq!(get_factor_list(5), vec![1, 5]);
        assert_eq!(get_factor_list(24), vec![1, 2, 3, 4, 6, 8, 12, 24]);
    }

    #[test]
    fn test_num_of_divisors() {
        assert_eq!(num_of_divisors(100), 9);
    }

    #[test]
    fn test_get_prime_factors() {
        assert_eq!(get_prime_factors(0), Vec::<u64>::new());
        assert_eq!(get_prime_factors(100), vec![2, 2, 5, 5]);
        assert_eq!(get_prime_factors(2560), vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 5]);
    }

    #[test]
    fn test_get_prime_factors2() {
        assert_eq!(get_prime_factors2(0), Vec::<u64>::new());
        assert_eq!(get_prime_factors2(100), vec![2, 2, 5, 5]);
        assert_eq!(get_prime_factors2(2560), vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 5]);
    }

    #[test]
    fn test_euler_phi() {
        assert_eq!(euler_phi(100), 40);
    }
}
