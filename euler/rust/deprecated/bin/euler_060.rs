// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

use euler::concate_number::ConcateNumber;
use euler::primes::{get_prime_list, IsPrime};
use std::collections::BTreeMap;

/// Problem:
///
/// The primes 3, 7, 109, and 673, are quite remarkable. By taking any
/// two primes and concatenating them in any order the result will always
/// be prime. For example, taking 7 and 109, both 7109 and 1097 are prime.
/// The sum of these four primes, 792, represents the lowest sum for a set
/// of four primes with this property.
///
/// Find the lowest sum for a set of five primes for which any two primes
/// concatenate to produce another prime.

fn method1() -> usize {
    const MAX: usize = 10_000;
    let primes = get_prime_list(MAX);

    let is_prime = |num: usize| -> bool { primes.binary_search(&num).is_ok() };

    type PrimePair = Vec<usize>;
    let mut prime_maps: BTreeMap<usize, PrimePair> = BTreeMap::new();

    for p1 in &primes {
        let mut prime_pair = PrimePair::new();
        for p2 in &primes {
            if p1 < p2 {
                let c1 = p1.concate(*p2);
                let c2 = p2.concate(*p1);
                if c1 > MAX || c2 > MAX {
                    if c1.is_prime() && c2.is_prime() {
                        prime_pair.push(*p2);
                    }
                } else {
                    if is_prime(c1) && is_prime(c2) {
                        prime_pair.push(*p2);
                    }
                }
            }
        }

        if prime_pair.len() > 3 {
            prime_maps.insert(*p1, prime_pair);
        }
    }

    // TODO(Shaohua): Replace with combination
    for (p0, pair0) in prime_maps.iter() {
        for p1 in pair0 {
            if let Some(pair1) = prime_maps.get(&p1) {
                for p2 in pair1 {
                    if !pair0.contains(p2) {
                        continue;
                    }
                    if let Some(pair2) = prime_maps.get(&p2) {
                        for p3 in pair2 {
                            if !pair0.contains(p3) {
                                continue;
                            }
                            if !pair1.contains(p3) {
                                continue;
                            }
                            println!("{}, {}, {}, {}", p0, p1, p2, p3);

                            if let Some(pair3) = prime_maps.get(&p3) {
                                for p4 in pair3 {
                                    if !pair0.contains(p4) {
                                        continue;
                                    }
                                    if !pair1.contains(p4) {
                                        continue;
                                    }
                                    if !pair2.contains(p4) {
                                        continue;
                                    }
                                    println!("{}, {}, {}, {}, {}", p0, p1, p2, p3, p4);
                                    return p0 + p1 + p2 + p3 + p4;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    0
}

fn main() {
    println!("method1: {}", method1());
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(), 26033));
}
