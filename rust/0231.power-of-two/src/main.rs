// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Brute Force
pub fn is_power_of_two1(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    if n == 1 {
        return true;
    }

    let mut power = 1;
    while 0 < power && power < n {
        power <<= 1;
        if power == n {
            return true;
        }
    }
    false
}

// Count ones
pub fn is_power_of_two2(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    n.count_ones() == 1
}

pub type SolutionFn = fn(i32) -> bool;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(i32, bool)] = &[
        (1, true),
        (16, true),
        (3, false),
        (-32, false),
        (i32::MIN, false),
        (i32::MAX, false),
    ];
    for record in RECORDS {
        println!("record: {record:?}");
        assert_eq!(func(record.0), record.1);
    }
}

fn main() {
    for i in 0..31 {
        let num: i32 = 1 << i;
        println!("{num:011} = 0b{num:0b}");
    }

    check_solution(is_power_of_two1);
    check_solution(is_power_of_two2);
}

#[cfg(test)]
mod test {
    use super::{check_solution, is_power_of_two1, is_power_of_two2};

    #[test]
    fn test_is_power_of_two1() {
        check_solution(is_power_of_two1);
    }

    #[test]
    fn test_is_power_of_two2() {
        check_solution(is_power_of_two2);
    }
}
