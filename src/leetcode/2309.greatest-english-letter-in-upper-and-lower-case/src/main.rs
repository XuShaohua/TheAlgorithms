// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashSet;

// HashTable
pub fn greatest_letter1(s: String) -> String {
    debug_assert!(!s.is_empty());

    // 构造字符集
    let set: HashSet<char> = s.chars().collect();

    let mut largest_letter: char = ' ';

    // 遍历字符串所有的字符
    for chr in s.chars() {
        if (chr.is_ascii_lowercase() && set.contains(&chr.to_ascii_uppercase()))
            || (chr.is_ascii_uppercase() && set.contains(&chr.to_ascii_lowercase()))
        {
            largest_letter = largest_letter.max(chr);
        }
    }
    if largest_letter == ' ' {
        String::new()
    } else {
        largest_letter.to_ascii_uppercase().into()
    }
}

pub type SolutionFn = fn(String) -> String;

fn check_solution(func: SolutionFn) {
    let s = "lEeTcOdE".to_owned();
    assert_eq!(func(s), "E".to_owned());

    let s = "arRAzFif".to_owned();
    assert_eq!(func(s), "R".to_owned());

    let s = "AbCdEfGhIjK".to_owned();
    assert_eq!(func(s), "".to_owned());
}

fn main() {
    check_solution(greatest_letter1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, greatest_letter1};

    #[test]
    fn test_greatest_letter1() {
        check_solution(greatest_letter1);
    }
}
