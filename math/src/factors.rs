// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

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

/// Returns all available factors of a possitive integer.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_sign_loss)]
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

#[cfg(test)]
mod tests {
    use super::{get_factor_list, get_factors, Counter};

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
}
