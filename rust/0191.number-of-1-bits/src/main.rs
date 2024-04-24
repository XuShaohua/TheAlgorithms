// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

//! Problem: [Number of 1 bits](https://leetcode.com/problems/number-of-1-bits)

fn solution1(n: u32) -> i32 {
    let mut n = n;
    let mut count = 0;
    while n != 0 {
        count += n % 2;
        n /= 2;
    }
    count as i32
}

fn solution2(n: u32) -> i32 {
    let mut count = 0;
    for i in 0..32 {
        if n >> i & 1 == 1 {
            count += 1;
        }
    }
    count
}

fn solution3(n: u32) -> i32 {
    n.count_ones() as i32
}

type SolutionFn = fn(u32) -> i32;

fn check_solution(func: SolutionFn) {
    let n = 0b00000000000000000000000000001011;
    assert_eq!(func(n), 3);

    let n = 0b00000000000000000000000010000000;
    assert_eq!(func(n), 1);

    let n = 0b11111111111111111111111111111101;
    assert_eq!(func(n), 31);
}

fn main() {
    check_solution(solution1);
    check_solution(solution2);
    check_solution(solution3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, solution1, solution2, solution3};

    #[test]
    fn test_solution1() {
        check_solution(solution1);
    }

    #[test]
    fn test_solution2() {
        check_solution(solution2);
    }

    #[test]
    fn test_solution3() {
        check_solution(solution3);
    }
}
