// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// n 是有穷级数, 计算出级数中各项的系数, 只能是0或者1
pub fn check_powers_of_three1(n: i32) -> bool {
    assert!(n >= 1);
    let mut n = n;
    while n != 0 {
        if n % 3 == 2 {
            return false;
        }
        n /= 3;
    }
    true
}

pub type SolutionFn = fn(i32) -> bool;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(i32, bool)] = &[
        (12, true),
        (91, true),
        (1, true),
        (21, false),
        (81, true),
        (10_i32.pow(7), false),
    ];
    for record in RECORDS {
        assert_eq!(func(record.0), record.1);
    }
}

fn main() {
    check_solution(check_powers_of_three1);
}

#[cfg(test)]
mod tests {
    use super::{check_powers_of_three1, check_solution};

    #[test]
    fn test_check_powers_of_three1() {
        check_solution(check_powers_of_three1);
    }
}
