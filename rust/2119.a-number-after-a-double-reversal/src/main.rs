// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn is_same_after_reversals(num: i32) -> bool {
    if num == 0 {
        return true;
    }
    num % 10 != 0
}

pub type SolutionFn = fn(i32) -> bool;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(i32, bool)] = &[(526, true), (1800, false), (0, true), (-8, true), (8, true)];
    for record in RECORDS {
        assert_eq!(func(record.0), record.1);
    }
}

fn main() {
    check_solution(is_same_after_reversals);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_same_after_reversals};

    #[test]
    fn test_is_same_after_reversals() {
        check_solution(is_same_after_reversals);
    }
}
