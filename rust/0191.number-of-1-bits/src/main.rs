// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn hamming_weight1(n: i32) -> i32 {
    let mut n = n as u32;
    let mut count = 0;
    while n != 0 {
        count += n % 2;
        n /= 2;
    }
    count as i32
}

pub fn hamming_weight2(n: i32) -> i32 {
    let mut count = 0;
    for i in 0..32 {
        if n >> i & 1 == 1 {
            count += 1;
        }
    }
    count
}

pub fn hamming_weight3(n: i32) -> i32 {
    let mut count = 0;
    for i in 0..32 {
        count += n >> i & 1;
    }
    count
}

pub fn hamming_weight4(n: i32) -> i32 {
    (0..32).map(|i| n >> i & 1).sum()
}

pub fn hamming_weight_best(n: i32) -> i32 {
    n.count_ones() as i32
}

type SolutionFn = fn(i32) -> i32;

#[allow(overflowing_literals)]
fn check_solution(func: SolutionFn) {
    let n = 0b00000000000000000000000000001011;
    assert_eq!(func(n), 3);

    let n = 0b00000000000000000000000010000000;
    assert_eq!(func(n), 1);

    let n = 0b11111111111111111111111111111101;
    assert_eq!(func(n), 31);
}

fn main() {
    check_solution(hamming_weight1);
    check_solution(hamming_weight2);
    check_solution(hamming_weight3);
    check_solution(hamming_weight4);
    check_solution(hamming_weight_best);
}

#[cfg(test)]
mod tests {
    use super::{
        check_solution, hamming_weight1, hamming_weight2, hamming_weight3, hamming_weight4,
        hamming_weight_best,
    };

    #[test]
    fn test_hamming_weight1() {
        check_solution(hamming_weight1);
    }

    #[test]
    fn test_hamming_weight2() {
        check_solution(hamming_weight2);
    }

    #[test]
    fn test_hamming_weight3() {
        check_solution(hamming_weight3);
    }

    #[test]
    fn test_hamming_weight4() {
        check_solution(hamming_weight4);
    }
    #[test]
    fn test_hamming_weight_best() {
        check_solution(hamming_weight_best);
    }
}
