// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Bruteforce
pub fn is_three1(n: i32) -> bool {
    debug_assert!((1..=10_000).contains(&n));

    let mut count: i32 = 0;

    for a in 1..=n {
        if n % a == 0 {
            count += 1;
            println!("n: {n}, a: {a}, count: {count}");
        }
        if count > 3 {
            return false;
        }
    }
    count == 3
}

// 考虑平方根
pub fn is_three2(n: i32) -> bool {
    // n 的平方根
    let square_root: i32 = (n as f64).sqrt() as i32;
    println!("n: {n}, root: {square_root}");

    if square_root * square_root != n {
        return false;
    }

    // (1, n)
    let mut count: i32 = if n == 1 { 1 } else { 2 };

    for a in 2..=square_root {
        if n % a == 0 {
            count += 1;
        }
        if count > 3 {
            return false;
        }
    }
    count == 3
}

// 利用质数特性
// (1, prime, prime^2)
// TODO(Shaohua):

pub type SolutionFn = fn(i32) -> bool;

fn check_solution(func: SolutionFn) {
    assert!(!func(2));
    assert!(func(4));
    assert!(func(9));
    assert!(!func(14));
    assert!(func(25));
}

fn main() {
    check_solution(is_three1);
    check_solution(is_three2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_three1};

    #[test]
    fn test_is_three1() {
        check_solution(is_three1);
    }
}
