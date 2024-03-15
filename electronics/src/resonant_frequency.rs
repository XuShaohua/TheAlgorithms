// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! An LC circuit, also called a resonant circuit, tank circuit, or tuned circuit,
//! is an electric circuit consisting of an inductor, represented by the letter L,
//! and a capacitor, represented by the letter C, connected together.
//!
//! The circuit can act as an electrical resonator, an electrical analogue of a
//! tuning fork, storing energy oscillating at the circuit's resonant frequency.
//!
//! [LC circuit](https://en.wikipedia.org/wiki/LC_circuit)

use std::f64::consts::PI;

/// This function can calculate the resonant frequency of LC circuit,
///
/// for the given value of inductance and capacitnace.
#[must_use]
pub fn resonant_frequency(inductance: f64, capacitance: f64) -> f64 {
    debug_assert!(inductance > 0.0);
    debug_assert!(capacitance > 0.0);
    1.0 / (2.0 * PI * (inductance * capacitance).sqrt())
}

#[cfg(test)]
mod tests {
    use super::resonant_frequency;

    #[test]
    fn test_resonant_frequency() {
        assert_eq!(resonant_frequency(10.0, 5.0), 0.022_507_907_903_927_652);
    }
}
