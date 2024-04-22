// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! # References
//! - [Charge carrier density](https://en.wikipedia.org/wiki/Charge_carrier_density)
//! - [equilibrium carrier concentration](https://www.pveducation.org/pvcdrom/pn-junctions/equilibrium-carrier-concentration)
//! - [carriers concentrations](http://www.ece.utep.edu/courses/ee3329/ee3329/Studyguide/ToC/Fundamentals/Carriers/concentrations.html)

#[must_use]
pub fn get_electron_conc(hole_conc: f64, intrinsic_conc: f64) -> f64 {
    debug_assert!(hole_conc > 0.0);
    debug_assert!(intrinsic_conc >= 0.0);
    intrinsic_conc.powi(2) / hole_conc
}

#[must_use]
pub fn get_hole_conc(electron_conc: f64, intrinsic_conc: f64) -> f64 {
    debug_assert!(electron_conc > 0.0);
    debug_assert!(intrinsic_conc >= 0.0);

    intrinsic_conc.powi(2) / electron_conc
}

#[must_use]
pub fn get_intrinsic_conc(electron_conc: f64, hole_conc: f64) -> f64 {
    debug_assert!(electron_conc >= 0.0);
    debug_assert!(hole_conc >= 0.0);

    (electron_conc * hole_conc).sqrt()
}

#[cfg(test)]
mod tests {
    use super::{get_electron_conc, get_hole_conc, get_intrinsic_conc};

    #[test]
    fn test_get_electron_conc() {
        assert!((get_electron_conc(1600.0, 200.0) - 25.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_get_hole_conc() {
        assert!((get_hole_conc(1000.0, 1200.0) - 1440.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_get_intrinsic_conc() {
        assert!((get_intrinsic_conc(25.0, 100.0) - 50.0).abs() < f64::EPSILON);
    }
}
