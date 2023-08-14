// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Calculate real power from apparent power and power factor.
#[must_use]
pub fn real_power(apparent_power: f64, power_factor: f64) -> f64 {
    debug_assert!((-1.0..=1.0).contains(&power_factor));
    apparent_power * power_factor
}

/// Calculate reactive power from apparent power and power factor.
#[must_use]
pub fn reactive_power(apparent_power: f64, power_factor: f64) -> f64 {
    debug_assert!((-1.0..=1.0).contains(&power_factor));
    apparent_power * power_factor.mul_add(-power_factor, 1.0).sqrt()
}

#[cfg(test)]
mod tests {
    use super::{reactive_power, real_power};

    #[test]
    fn test_real_power() {
        assert_eq!(real_power(100.0, 0.9), 90.0);
        assert_eq!(real_power(0.0, 0.8), 0.0);
        assert_eq!(real_power(100.0, -0.9), -90.0);
    }

    #[test]
    fn test_reactive_power() {
        assert_eq!(reactive_power(100.0, 0.9), 43.58898943540673);
        assert_eq!(reactive_power(0.0, 0.8), 0.0);
        assert_eq!(reactive_power(100.0, -0.9), 43.58898943540673);
    }
}
