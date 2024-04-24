// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn solution1(n: i32) -> i32 {
    let mut n = n as u32;
    let mut count = 0;
    while n != 0 {
        count += n % 2;
        n /= 2;
    }
    count as i32
}

pub fn solution2(n: i32) -> i32 {
    let mut count = 0;
    for i in 0..32 {
        if n >> i & 1 == 1 {
            count += 1;
        }
    }
    count
}

pub fn solution3(n: i32) -> i32 {
    let mut count = 0;
    for i in 0..32 {
        count += n >> i & 1;
    }
    count
}

pub fn solution4(n: i32) -> i32 {
    (0..32).map(|i| n >> i & 1).sum()
}

pub fn solution_best(n: i32) -> i32 {
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
    check_solution(solution1);
    check_solution(solution2);
    check_solution(solution3);
    check_solution(solution4);
    check_solution(solution_best);
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
