// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// Bruteforce
// HashTable
// 字典计数
pub fn find_duplicates1(nums: Vec<i32>) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for &num in &nums {
        *map.entry(num).or_default() += 1;
    }

    map.into_iter()
        .filter(|(_num, count)| *count == 2)
        .map(|(num, _count)| num)
        .collect()
}

// 把数值放到索引的位置.
pub fn find_duplicates2(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    // 长度不够, 少一个元素.
    nums.push(0);

    // 先遍历, 把元素放在它的位置上.
    let len = nums.len();
    for i in 0..len {
        while nums[i] != nums[nums[i] as usize] {
            let val_index = nums[i] as usize;
            nums.swap(i, val_index);
        }
    }

    // 到找不在位置上的元素.
    let mut res: Vec<i32> = Vec::new();
    for (index, num) in nums.iter().enumerate() {
        if index != *num as usize {
            res.push(*num);
        }
    }
    res
}

// TODO(Shaohua):
// HashTable
// in-place hash

// TODO(Shaohua):

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    fn values_equal(v1: Vec<i32>, v2: Vec<i32>) -> bool {
        let mut v1 = v1;
        v1.sort_unstable();
        let mut v2 = v2;
        v2.sort_unstable();
        v1 == v2
    }

    let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
    let exp = vec![2, 3];
    assert!(values_equal(func(nums), exp));

    let nums = vec![1, 1, 2];
    let exp = vec![1];
    assert!(values_equal(func(nums), exp));

    let nums = vec![1];
    let exp = vec![];
    assert!(values_equal(func(nums), exp));

    let nums = vec![5, 4, 6, 7, 9, 3, 10, 9, 5, 6];
    let exp = vec![9, 5, 6];
    assert!(values_equal(func(nums), exp));
}

fn main() {
    check_solution(find_duplicates1);
    check_solution(find_duplicates2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, find_duplicates1};

    #[test]
    fn test_find_duplicates1() {
        check_solution(find_duplicates1);
    }
}
