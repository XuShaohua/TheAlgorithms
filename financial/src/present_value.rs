// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! An algorithm that calculates the present value of a stream of yearly cash flows given...
//!
//! 1. The discount rate (as a decimal, not a percent)
//! 2. An array of cash flows, with the index of the cash flow being the associated year
//!
//! Note: This algorithm assumes that cash flows are paid at the end of the specified year.
//!
//! [present value](https://www.investopedia.com/terms/p/presentvalue.asp)

#![allow(clippy::module_name_repetitions)]

#[must_use]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_possible_truncation)]
pub fn present_value(discount_rate: f64, cash_flows: &[f64]) -> f64 {
    debug_assert!(discount_rate >= 0.0);
    debug_assert!(!cash_flows.is_empty());
    cash_flows
        .iter()
        .enumerate()
        .map(|(index, cash_flow)| cash_flow / (1.0 + discount_rate).powi(index as i32))
        .sum()
    //return round(present_value, ndigits=2)
}

#[must_use]
pub fn present_value_round2(discount_rate: f64, cash_flows: &[f64]) -> f64 {
    let p = present_value(discount_rate, cash_flows);
    (p * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::present_value_round2;

    #[test]
    fn test_present_value() {
        assert!(
            (present_value_round2(0.13, &[10.0, 20.70, -293.0, 297.0]) - 4.69).abs() < f64::EPSILON
        );
        assert!(
            (present_value_round2(0.07, &[-109_129.39, 30923.23, 15098.93, 29734.0, 39.0])
                + 42739.63)
                .abs()
                < f64::EPSILON
        );
        assert!(
            (present_value_round2(0.07, &[109_129.39, 30923.23, 15098.93, 29734.0, 39.0])
                - 175_519.15)
                .abs()
                < f64::EPSILON
        );
    }
}
