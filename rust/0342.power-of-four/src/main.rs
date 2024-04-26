// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute Force
pub fn is_power_of_four1(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    if n == 1 {
        return true;
    }

    let mut power = 1;
    while 0 < power && power < n {
        power <<= 2;
        if power == n {
            return true;
        }
    }
    false
}

// 递归法
pub fn is_power_of_four2(n: i32) -> bool {
    if n == 1 {
        return true;
    }
    if n <= 0 || n % 4 != 0 {
        return false;
    }
    is_power_of_four2(n / 4)
}

// 迭代的写法
pub fn is_power_of_four3(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    if n == 1 {
        return true;
    }

    let mut n = n;
    while n % 4 == 0 {
        n /= 4;
    }
    n == 1
}

// 整数为 4^x 的特性
pub fn is_power_of_four4(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    n.count_ones() == 1 && n.trailing_zeros() % 2 == 0
}

pub type SolutionFn = fn(i32) -> bool;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(i32, bool)] = &[
        (16, true),
        (5, false),
        (1, true),
        (0, false),
        (-16, false),
        (i32::MIN, false),
        (i32::MAX, false),
    ];
    for record in RECORDS {
        assert_eq!(func(record.0), record.1);
    }
}

fn main() {
    for i in 0..16 {
        let num: i32 = 1 << (i * 2);
        println!("{num:10} = 0b{num:032b}");
    }

    check_solution(is_power_of_four1);
    check_solution(is_power_of_four2);
    check_solution(is_power_of_four3);
    check_solution(is_power_of_four4);
}

#[cfg(test)]
mod tests {
    use super::{
        check_solution, is_power_of_four1, is_power_of_four2, is_power_of_four3, is_power_of_four4,
    };

    #[test]
    fn test_is_power_of_four1() {
        check_solution(is_power_of_four1);
    }

    #[test]
    fn test_is_power_of_four2() {
        check_solution(is_power_of_four2);
    }

    #[test]
    fn test_is_power_of_four3() {
        check_solution(is_power_of_four3);
    }

    #[test]
    fn test_is_power_of_four4() {
        check_solution(is_power_of_four4);
    }
}
