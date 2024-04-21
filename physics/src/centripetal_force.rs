// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Centripetal force is the force acting on an object in curvilinear motion
//! directed towards the axis of rotation or centre of curvature.
//!
//! [Centripetal and centrifugal force](https://byjus.com/physics/centripetal-and-centrifugal-force)

/// The Centripetal Force formula is given as: (m*v*v)/r
#[must_use]
pub fn centripetal(mass: f64, velocity: f64, radius: f64) -> f64 {
    debug_assert!(mass >= 0.0);
    debug_assert!(radius > 0.0);
    mass * velocity.powi(2) / radius
}

#[cfg(test)]
mod tests {
    use super::centripetal;

    #[test]
    fn test_centripetal() {
        const PAIRS: &[(f64, f64, f64, f64)] = &[
            (15.5, -30.0, 10.0, 1395.0),
            (10.0, 15.0, 5.0, 450.0),
            (20.0, -50.0, 15.0, 3333.33),
            (12.25, 40.0, 25.0, 784.0),
            (50.0, 100.0, 50.0, 10000.0),
        ];
        for (mass, velocity, radius, result) in PAIRS {
            let cent = centripetal(*mass, *velocity, *radius);
            let cent_rounded = (cent * 100.0).round() / 100.0;
            assert!((cent_rounded - result).abs() < f64::EPSILON);
        }
    }
}
