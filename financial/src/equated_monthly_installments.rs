// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Program to calculate the amortization amount per month, given
//! - Principal borrowed
//! - Rate of interest per annum
//! - Years to repay the loan
//!
//! [Equated monthly installment](https://en.wikipedia.org/wiki/Equated_monthly_installment)

/// Formula for amortization amount per month:
/// ```txt
/// A = p * r * (1 + r)^n / ((1 + r)^n - 1)
/// ```
/// where p is the principal, r is the rate of interest per month
/// and n is the number of payments
#[must_use]
pub fn equated_monthly_installments(
    principal: f64,
    rate_per_annum: f64,
    years_to_repay: i32,
) -> f64 {
    debug_assert!(principal > 0.0);
    debug_assert!(rate_per_annum >= 0.0);
    debug_assert!(years_to_repay > 0);

    // Yearly rate is divided by 12 to get monthly rate
    let rate_per_month: f64 = rate_per_annum / 12.0;

    // Years to repay is multiplied by 12 to get number of payments as payment is monthly
    let number_of_payments: i32 = years_to_repay * 12;

    principal * rate_per_month * (1.0 + rate_per_month).powi(number_of_payments)
        / ((1.0 + rate_per_month).powi(number_of_payments) - 1.0)
}

#[cfg(test)]
mod tests {
    use super::equated_monthly_installments;

    #[test]
    fn test_equated_monthly_installments() {
        assert_eq!(
            equated_monthly_installments(25000.0, 0.12, 3),
            830.35774532127930
        );
        assert_eq!(
            equated_monthly_installments(25000.0, 0.12, 10),
            358.6773710064682
        );
    }
}
