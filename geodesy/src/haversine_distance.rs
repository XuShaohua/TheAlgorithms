// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub const AXIS_A: f64 = 6_378_137.0;
pub const AXIS_B: f64 = 6_356_752.314_245;
pub const RADIUS: f64 = 6_378_137.0;

/// Calculate great circle distance between two points in a sphere, given longitudes and latitudes.
///
/// We know that the globe is "sort of" spherical, so a path between two points
/// isn't exactly a straight line.
/// We need to account for the Earth's curvature when calculating distance
/// from point A to B.
/// This effect is negligible for small distances but adds up as distance increases.
/// The Haversine method treats the earth as a sphere which allows us to "project"
/// the two points A and B onto the surface of that sphere and approximate
/// the spherical distance between them.
/// Since the Earth is not a perfect sphere, other methods which model the
/// Earth's ellipsoidal nature are more accurate but a quick and modifiable
/// computation like Haversine can be handy for shorter range distances.
///
/// # Parameters
/// - `lat1` - latitude of coordinate 1
/// - `lon1` - longitude of coordinate 1
/// - `lat2` - latitude of coordinate 2
/// - `lon2` - longitude of coordinate 2
///
/// Returns geographical distance between two points in metres.
///
/// [Haversine formula](https://en.wikipedia.org/wiki/Haversine_formula)
/// [CONSTANTS per WGS84](https://en.wikipedia.org/wiki/World_Geodetic_System)
/// [Haversine Formulation Equation](https://en.wikipedia.org/wiki/Haversine_formula#Formulation)
#[must_use]
pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let flattening = (AXIS_A - AXIS_B) / AXIS_A;
    let phi_1 = (1.0 - flattening) * lat1.to_radians().tan().atan();
    let phi_2 = (1.0 - flattening) * lat2.to_radians().tan().atan();
    let lambda_1 = lon1.to_radians();
    let lambda_2 = lon2.to_radians();
    // Equation
    let sin_sq_phi = ((phi_2 - phi_1) / 2.0).sin();
    let sin_sq_lambda = ((lambda_2 - lambda_1) / 2.0).sin();
    // Square both values
    let sin_sq_phi = sin_sq_phi * sin_sq_phi;
    let sin_sq_lambda = sin_sq_lambda * sin_sq_lambda;
    let h_value = ((phi_1.cos() * phi_2.cos()).mul_add(sin_sq_lambda, sin_sq_phi)).sqrt();
    2.0 * RADIUS * h_value.asin()
}

#[cfg(test)]
mod tests {
    use super::haversine_distance;

    #[test]
    fn test_haversine_distance() {
        let distance = haversine_distance(37.774_856, -122.424_227, 37.864_742, -119.537_521);
        assert_eq!(distance, 254_466.402_295_254_6);
    }
}
