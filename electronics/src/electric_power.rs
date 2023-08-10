// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! [Electric power](https://en.m.wikipedia.org/wiki/Electric_power)

#[must_use]
pub fn get_voltage(current: f64, power: f64) -> f64 {
    debug_assert!(current > 0.0);
    debug_assert!(power >= 0.0);
    power / current
}

#[must_use]
pub fn get_current(voltage: f64, power: f64) -> f64 {
    debug_assert!(voltage > 0.0);
    debug_assert!(power >= 0.0);
    power / voltage
}

#[must_use]
pub fn get_power(voltage: f64, current: f64) -> f64 {
    debug_assert!(voltage >= 0.0);
    debug_assert!(current >= 0.0);
    voltage * current
}
