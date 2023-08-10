// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use crate::haversine_distance::{haversine_distance, AXIS_A, AXIS_B, RADIUS};

/// Calculate the shortest distance along the surface of an ellipsoid between
/// two points on the surface of earth given longitudes and latitudes.
///
/// Representing the earth as an ellipsoid allows us to approximate distances between
/// points on the surface much better than a sphere.
/// Ellipsoidal formulas treat the Earth as an oblate ellipsoid which means
/// accounting for the flattening that happens at the North and South poles.
/// Lambert's formulae provide accuracy on the order of 10 meteres over thousands of kilometeres.
/// Other methods can provide millimeter-level accuracy but this is a simpler method
/// to calculate long range distances without increasing computational intensity.
///
/// # Parameters:
/// - `lat1` - latitude of coordinate 1
/// - `lon1` - longitude of coordinate 1
/// - `lat2` - latitude of coordinate 2
/// - `lon2` - longitude of coordinate 2
///
/// Returns geographical distance between two points in metres
///
/// [Lambert's formular](https://en.wikipedia.org/wiki/Geographical_distance#Lambert's_formula_for_long_lines)
/// [Parametric lattitude](https://en.wikipedia.org/wiki/Latitude#Parametric_(or_reduced)_latitude)
#[must_use]
#[allow(clippy::suboptimal_flops)]
pub fn lamberts_ellipsoidal_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let flattening = (AXIS_A - AXIS_B) / AXIS_A;
    // Parametric latitudes
    let b_lat1 = ((1.0 - flattening) * lat1.to_radians().tan()).atan();
    let b_lat2 = ((1.0 - flattening) * lat2.to_radians().tan()).atan();

    // Compute central angle between two points
    // using haversine theta. sigma =  haversine_distance / equatorial radius
    let sigma = haversine_distance(lat1, lon1, lat2, lon2) / RADIUS;

    // Intermediate P and Q values
    let p_value = (b_lat1 + b_lat2) / 2.0;
    let q_value = (b_lat2 - b_lat1) / 2.0;

    // Intermediate X value
    // X = (sigma - sin(sigma)) * sin^2Pcos^2Q / cos^2(sigma/2)
    let x_numerator = p_value.sin().powi(2) * q_value.cos().powi(2);
    let x_demonimator = (sigma / 2.0).cos().powi(2);
    let x_value = (sigma - sigma.sin()) * (x_numerator / x_demonimator);

    // Intermediate Y value
    // Y = (sigma + sin(sigma)) * cos^2Psin^2Q / sin^2(sigma/2)
    let y_numerator = p_value.cos().powi(2) * q_value.sin().powi(2);
    let y_denominator = (sigma / 2.0).sin().powi(2);
    let y_value = (sigma + sigma.sin()) * (y_numerator / y_denominator);

    RADIUS * (sigma - ((flattening / 2.0) * (x_value + y_value)))
}

pub type Point2D = (f64, f64);

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn lamberts_ellipsoidal_distance_helper(pt1: Point2D, pt2: Point2D) -> i64 {
    lamberts_ellipsoidal_distance(pt1.0, pt1.1, pt2.0, pt2.1).round() as i64
}

#[cfg(test)]
mod tests {

    use super::{lamberts_ellipsoidal_distance_helper, Point2D};

    #[test]
    fn test_lamberts_ellipsoidal_distance_helper() {
        const SAN_FRANCISCO: Point2D = (37.774856, -122.424227);
        const YOSEMITE: Point2D = (37.864742, -119.537521);
        const NEW_YORK: Point2D = (40.713019, -74.012647);
        const VENICE: Point2D = (45.443012, 12.313071);
        assert_eq!(
            lamberts_ellipsoidal_distance_helper(SAN_FRANCISCO, YOSEMITE),
            254_351
        );
        assert_eq!(
            lamberts_ellipsoidal_distance_helper(SAN_FRANCISCO, NEW_YORK),
            4_138_992
        );
        assert_eq!(
            lamberts_ellipsoidal_distance_helper(SAN_FRANCISCO, VENICE),
            9_737_326
        );
    }
}
