// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

// Dynamic programming
pub fn nth_super_ugly_number1(n: i32, primes: Vec<i32>) -> i32 {
    if n == 1 {
        return 1;
    }

    let n: usize = n as usize;
    let primes_len: usize = primes.len();
    let mut index: Vec<usize> = vec![0; primes_len];
    let mut uglys: Vec<i32> = vec![1; n];

    for i in 1..n {
        let mut min_val: i64 = i32::MAX as i64;
        for j in 0..primes_len {
            min_val = min_val.min((primes[j] as i64) * (uglys[index[j]] as i64));
        }

        uglys[i] = min_val as i32;
        for j in 0..primes_len {
            if min_val == (primes[j] as i64) * (uglys[index[j]] as i64) {
                index[j] += 1;
            }
        }
    }

    uglys[n - 1]
}

// Binary heap
// 最小堆
pub fn nth_super_ugly_number2(n: i32, primes: Vec<i32>) -> i32 {
    let n = n as usize;
    let mut uglys: Vec<i32> = vec![1; n];
    let mut heap = BinaryHeap::from_iter(primes.iter().map(|&prime| Reverse(prime)));

    for ugly in uglys.iter_mut().skip(1) {
        let Reverse(min) = heap.pop().unwrap();
        *ugly = min;

        for &prime in &primes {
            if let Some(new_min) = prime.checked_mul(min) {
                heap.push(Reverse(new_min));
            }
            if min % prime == 0 {
                break;
            }
        }
    }

    uglys[n - 1]
}

pub type SolutionFn = fn(i32, Vec<i32>) -> i32;

fn check_nth_super_ugly_number(func: SolutionFn) {
    let n = 12;
    let primes = vec![2, 7, 13, 19];
    assert_eq!(func(n, primes), 32);

    let n = 1;
    let primes = vec![2, 3, 5];
    assert_eq!(func(n, primes), 1);
}

fn main() {
    check_nth_super_ugly_number(nth_super_ugly_number1);
    check_nth_super_ugly_number(nth_super_ugly_number2);
}

#[cfg(test)]
mod tests {
    use super::{check_nth_super_ugly_number, nth_super_ugly_number1, nth_super_ugly_number2};

    #[test]
    fn test_nth_super_ugly_number1() {
        check_nth_super_ugly_number(nth_super_ugly_number1);
    }

    #[test]
    fn test_nth_super_ugly_number2() {
        check_nth_super_ugly_number(nth_super_ugly_number2);
    }
}
