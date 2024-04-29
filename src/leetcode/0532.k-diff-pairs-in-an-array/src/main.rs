// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::{HashMap, HashSet};

// 哈稀表
pub fn find_pairs1(nums: Vec<i32>, k: i32) -> i32 {
    assert!(!nums.is_empty());

    let mut map = HashMap::new();
    for num in &nums {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut set = HashSet::new();
    for &num in &nums {
        // k = diff - num;
        // diff1 >= num
        let diff1 = num + k;
        if let Some(count) = map.get(&diff1) {
            if diff1 > num {
                set.insert(vec![num, diff1]);
            } else if diff1 == num {
                if *count > 1 {
                    set.insert(vec![num, diff1]);
                }
            }
        }

        // k = num - diff;
        // diff2 <= num
        let diff2 = num - k;
        if let Some(count) = map.get(&diff2) {
            if diff2 < num {
                set.insert(vec![diff2, num]);
            } else if diff2 == num {
                if *count > 1 {
                    set.insert(vec![num, diff2]);
                }
            }
        }
    }

    set.len() as i32
}

// 二分查找
pub fn find_pairs2(nums: Vec<i32>, k: i32) -> i32 {
    assert!(!nums.is_empty());

    // 先排序
    let mut nums = nums;
    nums.sort();
    let mut set = HashSet::new();

    for i in 0..(nums.len() - 1) {
        let val = nums[i] + k;
        if let Ok(_val_index) = nums[i + 1..].binary_search(&val) {
            set.insert((nums[i], val));
        }
    }

    set.len() as i32
}

// 快慢型双指针
pub fn find_pairs3(nums: Vec<i32>, k: i32) -> i32 {
    assert!(!nums.is_empty());

    // 先排序
    let mut nums = nums;
    nums.sort();

    let mut fast = 0;
    let len = nums.len();
    let mut count = 0;

    // 遍历数组
    while fast < len {
        let curr_val = nums[fast];
        let expected_val = curr_val + k;
        // 当前值和期待值是否相等, 即 k 是否为0.
        if expected_val == curr_val {
            // 只保留两个重复元素, 跳过其它的.
            let mut added = false;
            while fast + 1 < len && curr_val == nums[fast + 1] {
                if !added {
                    count += 1;
                    added = true;
                }
                fast += 1;
            }
        } else {
            // 跳过所有重复元素
            while fast + 1 < len && curr_val == nums[fast + 1] {
                fast += 1;
            }
            // 使用二分查找法在后面的元素里搜索 `expected_val`.
            if fast + 1 < len && nums[fast + 1..].binary_search(&expected_val).is_ok() {
                count += 1;
            }
        }

        // 指针向前走一步
        fast += 1;
    }

    count
}

// 根据 k == 0 做优化
pub fn find_pairs4(nums: Vec<i32>, k: i32) -> i32 {
    assert!(!nums.is_empty());

    // 先排序
    let mut nums = nums;
    nums.sort();
    let mut count = 0;
    let mut fast = 0;

    if k == 0 {
        let len = nums.len();

        // 遍历数组
        while fast < len {
            let curr_val = nums[fast];
            // 只保留两个重复元素, 跳过其它的.
            if fast + 1 < len && curr_val == nums[fast + 1] {
                count += 1;
                fast += 1;
            }
            while fast + 1 < len && curr_val == nums[fast + 1] {
                fast += 1;
            }

            // 指针向前走一步
            fast += 1;
        }
    } else {
        // 去掉重复元素
        nums.dedup();
        let len = nums.len();

        // 遍历数组
        while fast < len {
            let curr_val = nums[fast];
            let expected_val = curr_val + k;
            // 使用二分查找法在后面的元素里搜索 `expected_val`.
            if fast + 1 < len && nums[fast + 1..].binary_search(&expected_val).is_ok() {
                count += 1;
            }

            // 指针向前走一步
            fast += 1;
        }
    }

    count
}

pub type SolutionFn = fn(Vec<i32>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![3, 1, 4, 1, 5];
    let k = 2;
    assert_eq!(func(nums, k), 2);

    let nums = vec![1, 2, 3, 4, 5];
    let k = 1;
    assert_eq!(func(nums, k), 4);

    let nums = vec![1, 3, 1, 5, 4];
    let k = 0;
    assert_eq!(func(nums, k), 1);
}

fn main() {
    check_solution(find_pairs1);
    check_solution(find_pairs2);
    check_solution(find_pairs3);
    check_solution(find_pairs4);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_pairs1, find_pairs2, find_pairs3, find_pairs4};

    #[test]
    fn test_find_pairs1() {
        check_solution(find_pairs1);
    }

    #[test]
    fn test_find_pairs2() {
        check_solution(find_pairs2);
    }

    #[test]
    fn test_find_pairs3() {
        check_solution(find_pairs3);
    }

    #[test]
    fn test_find_pairs4() {
        check_solution(find_pairs4);
    }
}
