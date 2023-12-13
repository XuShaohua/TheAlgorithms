// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

#[must_use]
#[inline]
const fn is_even(n: usize) -> bool {
    n % 2 == 0
}

#[must_use]
pub const fn pow(x: i32, n: usize) -> i32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return x;
    }
    let mut accm = x;
    let mut m = n - 1;
    while m != 0 {
        accm *= accm;
        m /= 2;
    }
    if !is_even(n) {
        accm *= x;
    }
    accm
}

#[must_use]
pub const fn pow_recursive(x: i32, n: usize) -> i32 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return x;
    }
    if is_even(n) {
        pow(x * x, n / 2)
    } else {
        pow(x * x, n / 2) * x
    }
}

#[cfg(test)]
mod tests {
    use super::pow;

    #[test]
    fn test_pow() {
        let r = pow(3, 4);
        assert_eq!(r, 81);
    }
}
