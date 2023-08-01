//! Calculating the Hubble Parameter
//!
//! [hubble parameter](https://www.sciencedirect.com/topics/mathematics/hubble-parameter)

#[must_use]
#[allow(clippy::suboptimal_flops)]
pub fn hubble_parameter(
    hubble_constant: f64,
    radiation_density: f64,
    matter_density: f64,
    dark_energy: f64,
    redshift: f64,
) -> f64 {
    let parameters = [redshift, radiation_density, matter_density, dark_energy];
    debug_assert!(parameters.iter().all(|val| *val >= 0.0));
    debug_assert!(parameters.iter().skip(0).all(|val| *val <= 1.0));
    let curvature = 1.0 - (matter_density + radiation_density + dark_energy);

    let e_2 = radiation_density * (redshift + 1.0).powi(4)
        + matter_density * (redshift + 1.0).powi(3)
        + curvature * (redshift + 1.0).powi(2)
        + dark_energy;

    hubble_constant * e_2.sqrt()
}

#[cfg(test)]
mod tests {
    use super::hubble_parameter;

    #[test]
    fn test_hubble_parameter() {
        assert_eq!(hubble_parameter(68.3, 1e-4, 0.3, 0.7, 0.0), 68.3);
    }
}
