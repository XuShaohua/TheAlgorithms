// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// TEMPERATURE (unit = K)
pub const T: f64 = 300.0;
pub const BOLTZMANN: f64 = 1.380_649e-23;
pub const ELECTRON_VOLT: f64 = 1.602_176_634e-19;

/// This function can calculate the Builtin Voltage of a pn junction diode.
///
/// This is calculated from the given three values.
///
/// # Parameters
/// - `donor_conc` - donor concentration
/// - `acceptor_conc` - acceptor concentration
/// - `intrinsic_conc` - intrinsic concentration
#[must_use]
pub fn builtin_voltage(donor_conc: f64, acceptor_conc: f64, intrinsic_conc: f64) -> f64 {
    debug_assert!(donor_conc > 0.0);
    debug_assert!(acceptor_conc > 0.0);
    debug_assert!(intrinsic_conc > 0.0);
    debug_assert!(donor_conc > intrinsic_conc);
    debug_assert!(acceptor_conc > intrinsic_conc);
    BOLTZMANN * T * ((donor_conc * acceptor_conc) / intrinsic_conc.powi(2)).ln() / ELECTRON_VOLT
}

#[cfg(test)]
mod tests {
    use super::builtin_voltage;

    #[test]
    fn test_builtin_voltage() {
        assert_eq!(builtin_voltage(1e17, 1e17, 1e10), 0.833370010652644);
    }
}
