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

#[cfg(test)]
mod tests {
    use super::{get_factors, Counter};

    #[test]
    fn test_get_factors() {
        assert_eq!(get_factors(45), Counter::from([(3, 2), (5, 1)]));
        assert_eq!(
            get_factors(2520),
            Counter::from([(2, 3), (3, 2), (5, 1), (7, 1)])
        );
    }
}
