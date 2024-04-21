// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Calculate price plus tax of a good or service given its price and a tax rate.

#[must_use]
pub fn price_plus_tax(price: f64, tax_rate: f64) -> f64 {
    price * (1.0 + tax_rate)
}

#[cfg(test)]
mod tests {
    use super::price_plus_tax;
    #[test]
    fn test_price_plus_tax() {
        assert!((price_plus_tax(100.0, 0.25) - 125.0).abs() < f64::EPSILON);
        assert!((price_plus_tax(125.50, 0.05) - 131.775).abs() < f64::EPSILON);
    }
}
