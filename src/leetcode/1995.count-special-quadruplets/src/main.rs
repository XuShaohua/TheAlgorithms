// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// Bruteforce
pub fn count_quadruplets1(nums: Vec<i32>) -> i32 {
    debug_assert!(nums.len() >= 4);

    let mut count: i32 = 0;

    let len: usize = nums.len();
    for a in 0..(len - 3) {
        for b in (a + 1)..(len - 2) {
            for c in (b + 1)..(len - 1) {
                for d in (c + 1)..len {
                    if nums[a] + nums[b] + nums[c] == nums[d] {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

// HashTable
// FIXME(Shaohua): count invalid
pub fn count_quadruplets2(nums: Vec<i32>) -> i32 {
    debug_assert!(nums.len() >= 4);

    let len: usize = nums.len();
    let mut count: i32 = 0;
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    // 倒数第一个整数 - 倒数第二个整数
    map.insert(nums[len - 1] - nums[len - 2], 1);

    // 两层嵌套遍历
    for b in (1..=(len - 3)).rev() {
        for a in (0..=(b - 1)).rev() {
            println!("a: {a}, b: {b}");
            let sum: i32 = nums[a] + nums[b];
            if let Some(sub_count) = map.get(&sum) {
                count += sub_count;
            }

            // 然后更新缓存.
            for d in ((b + 1)..len).rev() {
                let sub: i32 = nums[d] - nums[b];
                *map.entry(sub).or_default() += 1;
            }
        }
    }
    println!("map: {map:?}");

    count
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 3, 6];
    assert_eq!(func(nums), 1);

    let nums = vec![3, 3, 6, 4, 5];
    assert_eq!(func(nums), 0);

    let nums = vec![1, 1, 1, 3, 5];
    assert_eq!(func(nums), 4);

    let nums = vec![28, 8, 49, 85, 37, 90, 20, 8];
    assert_eq!(func(nums), 1);
}

fn main() {
    check_solution(count_quadruplets1);
    //check_solution(count_quadruplets2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, count_quadruplets1};

    #[test]
    fn test_count_quadruplets1() {
        check_solution(count_quadruplets1);
    }
}
