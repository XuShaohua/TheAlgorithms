// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::{HashMap, HashSet};

// 使用哈稀表计数器
// 时间复杂度: O(n)
// 空间复杂度: O(n)
pub fn find_duplicate1(nums: Vec<i32>) -> i32 {
    let len: i32 = nums.len() as i32;
    assert!(len >= 2);
    let n: i32 = len - 1;
    assert!(n >= 1);
    for &num in &nums {
        assert!(num >= 1 && num <= n);
    }

    let mut map: HashMap<i32, usize> = HashMap::new();
    for &num in &nums {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }
    for (key, count) in map {
        if count > 1 {
            return key;
        }
    }
    -1
}

// 使用 HashSet 来记录整数是否被插入过
// 时间复杂度: O(n)
// 空间复杂度: O(n)
pub fn find_duplicate11(nums: Vec<i32>) -> i32 {
    let mut set: HashSet<i32> = HashSet::with_capacity(nums.len());
    for &num in &nums {
        // 如果元素已经在集合中, 就返回false; 如果是第一次插入, 就返回 true.
        if !set.insert(num) {
            return num;
        }
    }
    -1
}

// BitSet
// 利用标志位来标记出现次数多于一次的整数.
// 时间复杂度: O(n)
// 空间复杂度: O(n)
pub fn find_duplicate2(nums: Vec<i32>) -> i32 {
    let mut bits: Vec<bool> = vec![false; nums.len()];
    for num in nums {
        let num_usize = num as usize;
        if bits[num_usize] {
            return num;
        }
        bits[num_usize] = true;
    }
    -1
}

// BitSet
// 利用标志位来标记出现次数多于一次的整数.
// 使用真正的bitset, 而不是 Vec<bool>.
// 时间复杂度: O(n)
// 空间复杂度: O(n)
pub fn find_duplicate22(nums: Vec<i32>) -> i32 {
    let mut bits = [0u64; (100_000) / 64 + 1];

    for num in nums {
        let num_usize = num as usize;
        let slot = num_usize / 64;
        let pos = num_usize % 64;

        let mask = 1 << pos;
        // 检查特定的比特位.
        if bits[slot] & mask == mask {
            return num;
        }
        bits[slot] |= mask;
    }
    -1
}

// Two Pointers
// 快慢型双指针
// 时间复杂度: O(n)
// 空间复杂度: O(1)
// TODO(Shaohua):
pub fn find_duplicate3(nums: Vec<i32>) -> i32 {
    let mut slow: i32 = 0;
    let mut fast: i32 = 0;

    loop {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];
        if slow == fast {
            break;
        }
    }

    slow = 0;
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }

    slow
}

// 使用正负数来标记该整数已经出现过
// 时间复杂度: O(n)
// 空间复杂度: O(1)
// 但是它需要修改数组
pub fn find_duplicate4(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    for i in 0..len {
        let index: i32 = nums[i].abs();
        let index_usize: usize = index as usize;
        if nums[index_usize] < 0 {
            return index;
        }
        nums[index_usize] *= -1;
    }
    -1
}

// 先对数组排序, 然后遍历它, 找到重复元素
// 时间复杂度: O(n log(n))
// 空间复杂度: O(n)
// 但是会修改原数组.
pub fn find_duplicate5(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    let len = nums.len();
    for i in 0..(len - 1) {
        if nums[i] == nums[i + 1] {
            return nums[i];
        }
    }
    -1
}

// TODO: Bit Manipulation
// 遍历每个整数的每个比特位, 然后使用

// Binary Search
// 时间复杂度: O(n log(n))
// 空间复杂度: O(1)
pub fn find_duplicate6(nums: Vec<i32>) -> i32 {
    assert!(nums.len() >= 2);

    let len = nums.len();
    let mut left: i32 = 0;
    let mut right: i32 = len as i32 - 1;

    while left < right {
        let mid: i32 = left + (right - left) / 2;

        let mut count = 0;
        for &num in &nums {
            if num <= mid {
                count += 1;
            }
        }

        if count <= mid {
            // 如果 count <= mid, 则说明重复的整数出现在右侧区域.
            left = mid + 1;
        } else {
            // 如果 count > mid, 说明重复的整数出现在左侧区域.
            right = mid;
        }
    }
    left
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 3, 4, 2, 2];
    assert_eq!(func(nums), 2);

    let nums = vec![3, 1, 3, 4, 2];
    assert_eq!(func(nums), 3);

    let nums = vec![3, 3, 3, 3, 3];
    assert_eq!(func(nums), 3);

    let nums = vec![1, 1, 2];
    assert_eq!(func(nums), 1);

    let nums = vec![3, 1, 3, 4, 2];
    assert_eq!(func(nums), 3);
}

fn main() {
    check_solution(find_duplicate1);
    check_solution(find_duplicate11);
    check_solution(find_duplicate2);
    check_solution(find_duplicate22);
    check_solution(find_duplicate3);
    check_solution(find_duplicate4);
    check_solution(find_duplicate5);
    check_solution(find_duplicate6);
}

#[cfg(test)]
mod tests {
    use super::{
        check_solution, find_duplicate1, find_duplicate11, find_duplicate2, find_duplicate22,
        find_duplicate3, find_duplicate4, find_duplicate5, find_duplicate6,
    };

    #[test]
    fn test_find_duplicate1() {
        check_solution(find_duplicate1);
    }

    #[test]
    fn test_find_duplicate11() {
        check_solution(find_duplicate11);
    }

    #[test]
    fn test_find_duplicate2() {
        check_solution(find_duplicate2);
    }

    #[test]
    fn test_find_duplicate22() {
        check_solution(find_duplicate22);
    }

    #[test]
    fn test_find_duplicate3() {
        check_solution(find_duplicate3);
    }

    #[test]
    fn test_find_duplicate4() {
        check_solution(find_duplicate4);
    }

    #[test]
    fn test_find_duplicate5() {
        check_solution(find_duplicate5);
    }

    #[test]
    fn test_find_duplicate6() {
        check_solution(find_duplicate6);
    }
}
