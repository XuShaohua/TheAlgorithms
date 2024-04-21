// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! [investopdia](https://www.investopedia.com)

#![allow(clippy::module_name_repetitions)]

#[must_use]
pub fn simple_interest(
    principal: f64,
    daily_interest_rate: f64,
    days_between_payments: f64,
) -> f64 {
    debug_assert!(days_between_payments > 0.0);
    debug_assert!(daily_interest_rate >= 0.0);
    debug_assert!(principal > 0.0);
    principal * daily_interest_rate * days_between_payments
}

#[must_use]
pub fn compound_interest(
    principal: f64,
    nominal_annual_interest_rate_percentage: f64,
    number_of_compounding_periods: f64,
) -> f64 {
    debug_assert!(number_of_compounding_periods > 0.0);
    debug_assert!(nominal_annual_interest_rate_percentage >= 0.0);
    debug_assert!(principal > 0.0);

    principal
        * ((1.0 + nominal_annual_interest_rate_percentage).powf(number_of_compounding_periods)
            - 1.0)
}

#[must_use]
pub fn apr_interest(
    principal: f64,
    nominal_annual_percentage_rate: f64,
    number_of_years: f64,
) -> f64 {
    debug_assert!(number_of_years > 0.0);
    debug_assert!(nominal_annual_percentage_rate >= 0.0);
    debug_assert!(principal > 0.0);

    compound_interest(
        principal,
        nominal_annual_percentage_rate / 365.0,
        number_of_years * 365.0,
    )
}

#[cfg(test)]
mod tests {
    use super::{apr_interest, compound_interest, simple_interest};

    #[test]
    fn test_simple_interest() {
        assert!((simple_interest(18000.0, 0.06, 3.0) - 3240.00).abs() < f64::EPSILON);
        assert!((simple_interest(0.5, 0.06, 3.0) - 0.09).abs() < f64::EPSILON);
        assert!((simple_interest(18000.0, 0.01, 10.0) - 1800.0).abs() < f64::EPSILON);
        assert!((simple_interest(18000.0, 0.0, 3.0) - 0.0).abs() < f64::EPSILON);
        assert!((simple_interest(5500.0, 0.01, 100.0) - 5500.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_compound_interest() {
        assert!(
            (compound_interest(10000.0, 0.05, 3.0) - 1_576.250_000_000_001_4).abs() < f64::EPSILON
        );
        assert!(
            (compound_interest(10000.0, 0.05, 1.0) - 500.000_000_000_000_45).abs() < f64::EPSILON
        );
        assert!(
            (compound_interest(0.5, 0.05, 3.0) - 0.078_812_500_000_000_06).abs() < f64::EPSILON
        );
    }

    #[test]
    fn test_apr_interest() {
        assert!((apr_interest(10000.0, 0.05, 3.0) - 1_618.223_072_263_547).abs() < f64::EPSILON);
        assert!((apr_interest(10000.0, 0.05, 1.0) - 512.674_964_674_473_2).abs() < f64::EPSILON);
        assert!((apr_interest(0.5, 0.05, 3.0) - 0.080_911_153_613_177_36).abs() < f64::EPSILON);
    }
}
