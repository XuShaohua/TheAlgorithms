// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// HashTab
// 字典计数
pub fn are_occurrences_equal1(s: String) -> bool {
    debug_assert!(!s.is_empty());

    let mut map: HashMap<char, usize> = HashMap::with_capacity(s.len());

    for chr in s.chars() {
        *map.entry(chr).or_default() += 1;
    }

    let mut last_count: usize = 0;
    for count in map.values() {
        if last_count == 0 {
            last_count = *count;
        } else if last_count != *count {
            return false;
        }
    }
    true
}

pub type SolutionFn = fn(String) -> bool;

fn check_solution(func: SolutionFn) {
    let s = "abacbc".to_owned();
    assert!(func(s));

    let s = "aaabb".to_owned();
    assert!(!func(s));
}

fn main() {
    check_solution(are_occurrences_equal1);
}

#[cfg(test)]
mod tests {
    use super::{are_occurrences_equal1, check_solution};

    #[test]
    fn test_are_occurrences_equal1() {
        check_solution(are_occurrences_equal1);
    }
}
