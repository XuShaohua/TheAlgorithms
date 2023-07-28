// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Shear stress is a component of stress that is coplanar to the material cross-section.
//!
//! [Shear stress](https://en.wikipedia.org/wiki/Shear_stress)

#[must_use]
pub fn get_area(stress: f64, tangential_force: f64) -> f64 {
    debug_assert!(stress > 0.0);
    debug_assert!(tangential_force > 0.0);
    tangential_force / stress
}

#[must_use]
pub fn get_stress(area: f64, tangential_force: f64) -> f64 {
    debug_assert!(tangential_force > 0.0);
    debug_assert!(area > 0.0);
    tangential_force / area
}

#[must_use]
pub fn get_tangential_force(area: f64, stress: f64) -> f64 {
    debug_assert!(stress > 0.0);
    debug_assert!(area > 0.0);
    stress * area
}

#[cfg(test)]
mod tests {
    use super::{get_area, get_stress, get_tangential_force};

    #[test]
    fn test_get_area() {
        assert_eq!(get_area(25.0, 100.0), 4.0);
    }

    #[test]
    fn test_get_stress() {
        assert_eq!(get_stress(200.0, 1600.0), 8.0);
    }

    #[test]
    fn test_get_tangential_force() {
        assert_eq!(get_tangential_force(1200.0, 1000.0), 1200000.0);
    }
}
