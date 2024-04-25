// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[allow(clippy::manual_range_contains)]
pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut rev = 0;
    while x != 0 {
        if rev > i32::MAX / 10 || rev < i32::MIN / 10 {
            return 0;
        }
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    rev
}

pub type SolutionFn = fn(i32) -> i32;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(i32, i32)] = &[
        (123, 321),
        (-123, -321),
        (120, 21),
        (1534236469, 0),
        (-2147483648, 0),
        (-2147483412, -2143847412),
    ];
    for record in RECORDS {
        assert_eq!(func(record.0), record.1);
    }
}

fn main() {
    check_solution(reverse);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, reverse};

    #[test]
    fn test_reverse() {
        check_solution(reverse);
    }
}
