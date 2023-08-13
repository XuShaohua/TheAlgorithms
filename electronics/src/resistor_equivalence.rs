// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! [equivalent resistance formula](https://byjus.com/equivalent-resistance-formula)

/// Req = 1/ (1/R1 + 1/R2 + ... + 1/Rn)
#[must_use]
pub fn resistor_parallel(resistors: &[f64]) -> f64 {
    let mut first_sum = 0.00;
    for resistor in resistors {
        debug_assert!(*resistor > 0.0);
        first_sum += 1.0 / resistor;
    }
    1.0 / first_sum
}

/// Calculate the equivalent resistance for any number of resistors in parallel.
///
/// Req = R1 + R2 + ... + Rn
#[must_use]
pub fn resistor_series(resistors: &[f64]) -> f64 {
    let mut sum_r = 0.00;
    for resistor in resistors {
        debug_assert!(*resistor >= 0.0);
        sum_r += resistor;
    }
    sum_r
}

#[cfg(test)]
mod tests {
    use super::{resistor_parallel, resistor_series};

    #[test]
    fn test_resistor_parallel() {
        assert_eq!(resistor_parallel(&[3.21389, 2.0, 3.0]), 0.8737571620498019);
    }

    #[test]
    fn test_resistor_series() {
        assert_eq!(resistor_series(&[3.21389, 2.0, 3.0]), 8.21389);
    }
}
