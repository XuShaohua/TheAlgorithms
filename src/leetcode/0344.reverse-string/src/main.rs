// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::ptr_arg)]

// 靠拢型双指针
pub fn reverse_string1(s: &mut Vec<char>) {
    debug_assert!(!s.is_empty());

    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        // 首尾交换
        s.swap(left, right);

        left += 1;
        right -= 1;
    }
}

pub fn reverse_string2(s: &mut Vec<char>) {
    let len = s.len();
    for i in 0..(len / 2) {
        // 首尾交换
        s.swap(i, len - i - 1);
    }
}

pub type SolutionFn = fn(&mut Vec<char>);

fn check_solution(func: SolutionFn) {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    let exp = vec!['o', 'l', 'l', 'e', 'h'];
    func(&mut s);
    assert_eq!(s, exp);

    let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    let exp = vec!['h', 'a', 'n', 'n', 'a', 'H'];
    func(&mut s);
    assert_eq!(s, exp);
}

fn main() {
    check_solution(reverse_string1);
    check_solution(reverse_string2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, reverse_string1, reverse_string2};

    #[test]
    fn test_reverse_string1() {
        check_solution(reverse_string1);
    }

    #[test]
    fn test_reverse_string2() {
        check_solution(reverse_string2);
    }
}
