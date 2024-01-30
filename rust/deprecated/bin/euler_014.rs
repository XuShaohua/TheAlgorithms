// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// The following iterative sequence is defined for the set of positive
/// integers:
///
///     n → n/2 (n is even)
///     n → 3n + 1 (n is odd)
///
/// Using the rule above and starting with 13, we generate the following
/// sequence:
///
///     13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
///
/// It can be seen that this sequence (starting at 13 and finishing at 1)
/// contains 10 terms. Although it has not been proved yet (Collatz Problem),
/// it is thought that all starting numbers finish at 1.
/// Which starting number, under one million, produces the longest chain?
///
/// NOTE: Once the chain starts the terms are allowed to go above one million.

fn method1() -> u32 {
    let get_collatz_sequence = |n: u32| -> u32 {
        let mut n: u64 = n as u64;
        let mut seq = 0;
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
            seq += 1;
        }
        seq
    };

    let mut largest_sequence = 0;
    let mut largest_sequence_num = 0;
    for i in 1..=1_000_000 {
        let seq = get_collatz_sequence(i);
        if seq > largest_sequence {
            largest_sequence_num = i;
            largest_sequence = seq;
        }
    }

    largest_sequence_num
}

fn method2() -> usize {
    const MAX: usize = 1_000_000;
    let cache: [u32; MAX + 1] = [0; MAX + 1];
    let get_collatz_sequence = |num: usize| -> u32 {
        let mut n = num;
        let mut seq = 0;
        while n != 1 {
            if n < num && cache[n] != 0 {
                seq += cache[n];
                break;
            }
            if n % 2 == 0 {
                n /= 2;
            } else {
                n = 3 * n + 1;
            }
            seq += 1;
        }
        seq
    };

    let mut largest_sequence = 0;
    let mut largest_sequence_num = 0;
    for i in 1..=MAX {
        let seq = get_collatz_sequence(i);
        if seq > largest_sequence {
            largest_sequence_num = i;
            largest_sequence = seq;
        }
    }

    largest_sequence_num
}

fn main() {
    println!("method1: {}", method1());
    println!("method2: {}", method2());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 837799));
}

#[bench]
fn bench_method2(_b: &mut test::Bencher) {
    // FIXME(Shaohua): Stackoverflow
    //b.iter(|| method2());
}
