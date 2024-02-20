// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![feature(test)]
extern crate test;

/// Problem:
///
/// In the United Kingdom the currency is made up of pound (£) and pence (p).
/// There are eight coins in general circulation:
///     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
/// It is possible to make £2 in the following way:
///     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
/// How many different ways can £2 be made using any number of coins?

fn method1(max_num: usize) -> usize {
    let mut count = 0;
    for i200 in 0..=max_num {
        let s200 = i200 * 200;
        if s200 == max_num {
            count += 1;
            break;
        }
        if s200 > max_num {
            break;
        }
        for i100 in 0..=max_num {
            let s100 = s200 + i100 * 100;
            if s100 == max_num {
                count += 1;
                break;
            }
            if s100 > max_num {
                break;
            }
            for i50 in 0..=max_num {
                let s50 = s100 + i50 * 50;
                if s50 == max_num {
                    count += 1;
                    break;
                }
                if s50 > max_num {
                    break;
                }

                for i20 in 0..=max_num {
                    let s20 = s50 + i20 * 20;
                    if s20 == max_num {
                        count += 1;
                        break;
                    }
                    if s20 > max_num {
                        break;
                    }

                    for i10 in 0..=max_num {
                        let s10 = s20 + i10 * 10;
                        if s10 == max_num {
                            count += 1;
                            break;
                        }
                        if s10 > max_num {
                            break;
                        }
                        for i5 in 0..=max_num {
                            let s5 = s10 + i5 * 5;
                            if s5 == max_num {
                                count += 1;
                                break;
                            }
                            if s5 > max_num {
                                break;
                            }
                            for i2 in 0..=max_num {
                                let s2 = s5 + i2 * 2;
                                if s2 == max_num {
                                    count += 1;
                                    break;
                                }
                                if s2 < max_num {
                                    //println!("s2: {}, i100:{}, i50: {}, i20: {}, i10: {}, i5: {}, i2: {}",s2, i100, i50, i20, i10, i5, i2);
                                    count += 1;
                                } else if s2 == max_num {
                                    //println!("s2: {}, i100:{}, i50: {}, i20: {}, i10: {}, i5: {}, i2: {}", s2, i100, i50, i20, i10, i5, i2);
                                    count += 1;
                                    break;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn method2(max_num: usize) -> usize {
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let mut ways = vec![0; max_num + 1];
    ways[0] = 1;
    for coin in coins {
        for j in coin..=max_num {
            ways[j] += ways[j - coin];
        }
    }

    ways[max_num]
}

fn main() {
    let max_num = 200;
    println!("ways: {}", method1(max_num));
    println!("ways: {}", method2(max_num));
}

#[bench]
fn bench_method1(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method1(200), 73682));
}

#[bench]
fn bench_method2(b: &mut test::Bencher) {
    b.iter(|| assert_eq!(method2(200), 73682));
}
