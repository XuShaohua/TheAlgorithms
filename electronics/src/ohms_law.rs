// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Apply Ohm's Law, on any two given electrical values, which can be voltage, current,
//! and resistance.
//!
//![Ohm's law](https://en.wikipedia.org/wiki/Ohm%27s_law)

#[must_use]
pub fn get_voltage(current: f64, resistance: f64) -> f64 {
    debug_assert!(resistance >= 0.0);
    current * resistance
}

#[must_use]
pub fn get_current(voltage: f64, resistance: f64) -> f64 {
    debug_assert!(resistance > 0.0);
    voltage / resistance
}
#[must_use]
pub fn get_resistance(voltage: f64, current: f64) -> f64 {
    debug_assert!(current != 0.0);
    voltage / current
}
