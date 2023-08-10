// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

    
/// Calculate the apparent power in a single-phase AC circuit.
///
/// [Apparent power](https://en.wikipedia.org/wiki/AC_power#Apparent_power)
#[must_use]
pub fn apparent_power(voltage: f64, current: f64, voltage_angle: f64, current_angle: f64
) -> complex {
    // Convert angles from degrees to radians
    let voltage_angle_rad = voltage_angle.to_radians()
    let current_angle_rad = current_angle.to_radians()

    // Convert voltage and current to rectangular form
    let voltage_rect = cmath.rect(voltage, voltage_angle_rad)
    let current_rect = cmath.rect(current, current_angle_rad)

    // Calculate apparent power
    voltage_rect * current_rect
}

#[cfg(test)]
mod tests {
    use super::apparent_power;

    #[test]
    fn test_apparent_power() {
    apparent_power(100, 5, 0, 0)
    (500+0j)
    >>> apparent_power(100, 5, 90, 0)
    (3.061616997868383e-14+500j)
    >>> apparent_power(100, 5, -45, -60)
    (-129.40952255126027-482.9629131445341j)
    >>> apparent_power(200, 10, -30, -90)
    (-999.9999999999998-1732.0508075688776j)
    }
}
