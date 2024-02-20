// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::permutation::Permutation;
use euler::primes::get_prime_list;

/// Problem:
///
/// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms
/// increases by 3330, is unusual in two ways: (i) each of the three terms
/// are prime, and, (ii) each of the 4-digit numbers are permutations of
/// one another.
///
/// There are no arithmetic sequences made up of three 1-, 2-, or
/// 3-digit primes, exhibiting this property, but there is one other 4-digit
/// increasing sequence.
///
/// What 12-digit number do you form by concatenating the three terms in
/// this sequence?

fn method1() -> Option<(usize, usize, usize)> {
    let primes = get_prime_list(10000);
    let get_digits = |mut num: usize| -> Vec<usize> {
        let mut v = vec![];
        while num > 0 {
            v.push(num % 10);
            num /= 10;
        }
        v
    };

    for prime in &primes {
        if prime < &1000 {
            continue;
        }

        let mut nums = vec![*prime];
        let p = Permutation::new(get_digits(*prime));
        for digits in p.into_iter() {
            let n = digits.iter().fold(0, |prod, i| prod * 10 + i);
            if n > 1000 && &n > prime && primes.binary_search(&n).is_ok() {
                nums.push(n);
            }
        }
        nums.sort();
        nums.dedup();
        if nums.len() < 3 {
            continue;
        }

        // combinations
        let len = nums.len();
        for i in 0..len - 2 {
            for j in i + 1..len - 1 {
                for k in j + 1..len {
                    if nums[j] - nums[i] == nums[k] - nums[j] && nums[i] != 1487 {
                        return Some((nums[i], nums[j], nums[k]));
                    }
                }
            }
        }
    }

    None
}

fn main() {
    println!("method1: {:?}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), Some((2969, 6299, 9629))));
}
