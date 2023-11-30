// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

fn sum_of_polynomial(arr: &[f64], x: f64) -> f64 {
    if arr.is_empty() {
        return 0.0;
    }
    let mut p = 0.0;
    for (index, item) in arr.iter().enumerate() {
        p += item * x.powf(index as f64)
    }

    p
}

fn sum_of_polynomial2(arr: &[f64], x: f64) -> f64 {
    if arr.is_empty() {
        return 0.0;
    }
    let mut p = 0.0;
    for item in arr.iter().rev() {
        p = item + x * p;
    }

    p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method1() {
        let arr = (1..10).map(|i| i as f64).collect::<Vec<_>>();
        let x = 12.4;
        let sum = sum_of_polynomial(&arr, x);
        assert_eq!(sum, 5418501173.194776);
    }

    #[test]
    fn test_method2() {
        let arr = (1..10).map(|i| i as f64).collect::<Vec<_>>();
        let x = 12.4;
        let sum = sum_of_polynomial2(&arr, x);
        assert_eq!(sum, 5418501173.1947775);
    }

    #[bench]
    fn bench_method1(b: &mut test::Bencher) {
        let arr = (1..10).map(|i| i as f64).collect::<Vec<_>>();
        let x = 12.4;
        b.iter(|| assert_eq!(sum_of_polynomial(&arr, x), 5418501173.194776));
    }

    #[bench]
    fn bench_method2(b: &mut test::Bencher) {
        let arr = (1..10).map(|i| i as f64).collect::<Vec<_>>();
        let x = 12.4;
        b.iter(|| assert_eq!(sum_of_polynomial2(&arr, x), 5418501173.1947775));
    }
}
