// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// Hashble
// 字典计数
pub fn most_frequent_even1(nums: Vec<i32>) -> i32 {
    debug_assert!(!nums.is_empty());

    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut max_count: usize = 0;
    let mut ans: i32 = -1;

    for &num in &nums {
        if num % 2 == 0 {
            let count = *map.entry(num).and_modify(|count| *count += 1).or_insert(1);
            if (count > max_count) || (count == max_count && ans > num) {
                ans = num;
                max_count = count;
            }
        }
    }
    ans
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![0, 1, 2, 2, 4, 4, 1];
    assert_eq!(func(nums), 2);

    let nums = vec![4, 4, 4, 9, 2, 4];
    assert_eq!(func(nums), 4);

    let nums = vec![29, 47, 21, 41, 13, 37, 25, 7];
    assert_eq!(func(nums), -1);
}

fn main() {
    check_solution(most_frequent_even1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, most_frequent_even1};

    #[test]
    fn test_most_frequent_even1() {
        check_solution(most_frequent_even1);
    }
}
