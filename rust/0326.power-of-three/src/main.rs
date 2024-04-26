// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 递归法
pub fn is_power_of_three1(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    if n == 1 {
        return true;
    }

    if n % 3 != 0 {
        return false;
    }
    is_power_of_three1(n / 3)
}

// 将递归法改写为迭代的形式
pub fn is_power_of_three2(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    if n == 1 {
        return true;
    }

    let mut n = n;
    while n % 3 == 0 {
        n /= 3;
    }
    n == 1
}

// 利用次幂的特性:
// 如果 n == 3^x, 而 max_n = 3^max_x, 则 max_n % n == 0
pub fn is_power_of_three3(n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    // 找到 i32 中最大的 3 的次幂
    const fn max_power_of_three() -> i32 {
        let mut power: i32 = 1;
        loop {
            let (new_power, is_overflow) = power.overflowing_mul(3);
            if is_overflow {
                break;
            }
            power = new_power;
        }
        power
    }

    max_power_of_three() % n == 0
}

pub type SolutionFn = fn(i32) -> bool;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(i32, bool)] = &[
        (27, true),
        (42, false),
        (0, false),
        (-1, false),
        (-9, false),
        (i32::MIN, false),
        (i32::MAX, false),
    ];

    for record in RECORDS {
        assert_eq!(func(record.0), record.1);
    }
}

fn main() {
    for i in 0..20 {
        let num: i32 = 3_i32.pow(i);
        println!("{num:12} = 0b{num:0b}");
    }

    check_solution(is_power_of_three1);
    check_solution(is_power_of_three2);
    check_solution(is_power_of_three3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_power_of_three1, is_power_of_three2, is_power_of_three3};

    #[test]
    fn test_is_power_of_three1() {
        check_solution(is_power_of_three1);
    }

    #[test]
    fn test_is_power_of_three2() {
        check_solution(is_power_of_three2);
    }

    #[test]
    fn test_is_power_of_three3() {
        check_solution(is_power_of_three3);
    }
}
