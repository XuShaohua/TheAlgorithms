// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// units = C
pub const ELECTRON_CHARGE: f64 = 1.6021e-19;

#[must_use]
pub fn get_conductivity(electron_conc: f64, mobility: f64) -> f64 {
    debug_assert!(electron_conc >= 0.0);
    debug_assert!(mobility >= 0.0);
    mobility * electron_conc * ELECTRON_CHARGE
}

#[must_use]
pub fn get_electron_conc(conductivity: f64, mobility: f64) -> f64 {
    debug_assert!(conductivity >= 0.0);
    debug_assert!(mobility >= 0.0);
    conductivity / (mobility * ELECTRON_CHARGE)
}

#[must_use]
pub fn get_mobility(conductivity: f64, electron_conc: f64) -> f64 {
    debug_assert!(conductivity >= 0.0);
    debug_assert!(electron_conc > 0.0);
    conductivity / (electron_conc * ELECTRON_CHARGE)
}

#[cfg(test)]
mod tests {
    use super::{get_conductivity, get_electron_conc, get_mobility};

    #[test]
    fn test_get_mobility() {
        assert_eq!(get_mobility(25.0, 100.0), 1.560_451_906_872_230_1e18);
    }

    #[test]
    fn test_get_conductivity() {
        assert_eq!(get_conductivity(1600.0, 200.0), 5.12672e-14);
    }

    #[test]
    fn test_get_electron_conc() {
        assert_eq!(get_electron_conc(1000.0, 1200.0), 5.201_506_356_240_767e18);
    }
}
