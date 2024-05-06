// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn my_pow1(x: f64, n: i32) -> f64 {
    assert!(x != 0.0 || n > 0);
    if n == 0 {
        return 1.0;
    }

    let mut n: i64 = n as i64;
    let mut x = x;
    if n < 0 {
        n = -n;
        x = 1.0 / x;
    }

    let mut ans = 1.0;

    // 分治
    while n > 0 {
        if n % 2 == 0 {
            n /= 2;
            x *= x;
        } else {
            n -= 1;
            ans *= x;
        }
    }

    ans
}

pub type SolutionFn = fn(f64, i32) -> f64;

fn check_solution(func: SolutionFn) {
    fn nearly_equal(a: f64, b: f64) {
        println!("a: {a}, b: {b}");

        let diff: f64 = (a - b).abs();
        let scale: f64 = 100_000.0;

        assert!(diff * scale < 1.0);
    }

    let x = 2.00000;
    let n = 10;
    nearly_equal(func(x, n), 1024.00000);

    let x = 2.10000;
    let n = 3;
    nearly_equal(func(x, n), 9.26100);

    let x = 2.00000;
    let n = -2;
    nearly_equal(func(x, n), 0.25000);

    let x = 0.00001;
    let n = i32::MAX;
    nearly_equal(func(x, n), 0.00000);

    let x = 2.00000;
    let n = i32::MIN;
    nearly_equal(func(x, n), 0.00000);

    let x = -1.00000;
    let n = i32::MIN;
    nearly_equal(func(x, n), 1.00000);
}

fn main() {
    check_solution(my_pow1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, my_pow1};

    #[test]
    fn test_my_pow1() {
        check_solution(my_pow1);
    }
}
