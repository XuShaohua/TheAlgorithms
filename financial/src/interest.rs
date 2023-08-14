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
        assert_eq!(simple_interest(18000.0, 0.06, 3.0), 3240.00);
        assert_eq!(simple_interest(0.5, 0.06, 3.0), 0.09);
        assert_eq!(simple_interest(18000.0, 0.01, 10.0), 1800.0);
        assert_eq!(simple_interest(18000.0, 0.0, 3.0), 0.0);
        assert_eq!(simple_interest(5500.0, 0.01, 100.0), 5500.0);
    }

    #[test]
    fn test_compound_interest() {
        assert_eq!(compound_interest(10000.0, 0.05, 3.0), 1576.2500000000014);
        assert_eq!(compound_interest(10000.0, 0.05, 1.0), 500.00000000000045);
        assert_eq!(compound_interest(0.5, 0.05, 3.0), 0.07881250000000006);
    }

    #[test]
    fn test_apr_interest() {
        assert_eq!(apr_interest(10000.0, 0.05, 3.0), 1618.223072263547);
        assert_eq!(apr_interest(10000.0, 0.05, 1.0), 512.6749646744732);
        assert_eq!(apr_interest(0.5, 0.05, 3.0), 0.08091115361317736);
    }
}