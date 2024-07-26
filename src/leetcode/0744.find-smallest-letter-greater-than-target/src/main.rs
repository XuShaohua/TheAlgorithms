// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Binary Search
pub fn next_greatest_letter1(letters: Vec<char>, target: char) -> char {
    debug_assert!(letters.len() >= 2);

    let mut left = 0;
    let mut right = letters.len() - 1;
    while left < right {
        let middle = left + (right - left) / 2;
        if letters[middle] <= target {
            // 如果 middle 处的字符小于或者等于 target, 就将 left 向右移
            left = middle + 1;
        } else {
            // 如果 middle 处的字符大于 target, 就将 right 向左移
            right = middle;
        }
    }

    // 要判断有没有找到
    if letters[left] > target {
        letters[left]
    } else {
        letters[0]
    }
}

pub type SolutionFn = fn(Vec<char>, char) -> char;

fn check_solution(func: SolutionFn) {
    let letters = vec!['c', 'f', 'j'];
    let target = 'a';
    assert_eq!(func(letters, target), 'c');

    let letters = vec!['c', 'f', 'j'];
    let target = 'c';
    assert_eq!(func(letters, target), 'f');

    let letters = vec!['x', 'x', 'y', 'y'];
    let target = 'z';
    assert_eq!(func(letters, target), 'x');
}

fn main() {
    check_solution(next_greatest_letter1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, next_greatest_letter1};

    #[test]
    fn test_next_greatest_letter1() {
        check_solution(next_greatest_letter1);
    }
}
