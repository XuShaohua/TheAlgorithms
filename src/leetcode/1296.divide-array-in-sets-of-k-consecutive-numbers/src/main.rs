// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::{BTreeMap, BinaryHeap, HashMap};

// Priority Queue
pub fn is_possible_divide1(nums: Vec<i32>, k: i32) -> bool {
    assert!(k >= 1 && k as usize <= nums.len());
    let k = k as usize;

    // 处理边角情况.
    if nums.len() % k != 0 {
        return false;
    }

    // 降序排列所有整数.
    let mut heap: BinaryHeap<i32> = BinaryHeap::from(nums);
    // 存放临时元素.
    let mut aux_vec: Vec<i32> = Vec::new();
    // 我们按照降序来处理.

    let mut group: Vec<i32> = Vec::with_capacity(k);

    while let Some(top) = heap.pop() {
        if let Some(group_last) = group.last() {
            if group_last - 1 == top {
                // 可以继续进入组.
                group.push(top);
            } else {
                // 放在临时数组里.
                aux_vec.push(top);
            }
        } else {
            // 直接进入组, 并作为最大的值.
            group.push(top);
        }

        if group.len() == k {
            //println!("group: {group:?}");
            // 重置本组信息.
            group.clear();

            // 将临时元素再次加入到队列里, 等待下个循环来读取.
            for &num in &aux_vec {
                heap.push(num);
            }
            aux_vec.clear();
        }
        //println!("group: {group:?}, aux: {aux_vec:?}, heap: {heap:?}");
    }

    // 如果 aux_vec 不为空, 说明上面无法组成一个有序的组.
    heap.is_empty() && aux_vec.is_empty()
}

// HashMap
// 计数 + 有序数组
pub fn is_possible_divide2(nums: Vec<i32>, k: i32) -> bool {
    assert!(k >= 1 && k as usize <= nums.len());
    let k_usize = k as usize;

    // 处理边角情况.
    if nums.len() % k_usize != 0 {
        return false;
    }

    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    for &num in &nums {
        *map.entry(num).or_default() += 1;
    }

    let mut nums_count: Vec<(i32, usize)> = map.into_iter().collect();
    // 基于整数升序排列
    nums_count.sort_by(|a, b| a.0.cmp(&b.0));
    let len = nums_count.len();

    // 从数组头部依次向后遍历.
    for i in 0..len {
        // 贪心, 尽可能去构造连续的有序数组.
        // 一次尝试组装第一个元素.
        while nums_count[i].1 > 0 {
            nums_count[i].1 -= 1;
            let num = nums_count[i].0;
            //print!("group: [{num}, ");
            for j in 1..k {
                let index = i + j as usize;
                // 缺少元素, 无效索引.
                if index >= len {
                    return false;
                }

                // 不能组成连续的有序数组.
                if nums_count[index].0 != num + j || nums_count[index].1 == 0 {
                    //println!("Failed to group numbers");
                    return false;
                }
                // 消去一个计数.
                nums_count[index].1 -= 1;
                //print!("{}, ", nums_count[index].0);
            }
            //println!("]");
        }
    }

    true
}

// BTreeMap
// 计数, 并保证 key 是有序的
// 减少一个数组
pub fn is_possible_divide3(nums: Vec<i32>, k: i32) -> bool {
    assert!(k >= 1 && k as usize <= nums.len());
    let k_usize = k as usize;

    // 处理边角情况.
    if nums.len() % k_usize != 0 {
        return false;
    }

    let mut nums = nums;
    nums.sort_unstable();

    let mut map: BTreeMap<i32, usize> = BTreeMap::new();
    for &num in &nums {
        *map.entry(num).or_default() += 1;
    }

    // 遍历所有的整数.
    for &num in &nums {
        // 计数到0, 用完了.
        if map[&num] == 0 {
            continue;
        }
        for offset in 0..k {
            let next_num: i32 = offset + num;
            match map.get_mut(&next_num) {
                Some(count) if *count > 0 => {
                    *count -= 1;
                }
                // 计数到0, 用完了;
                // 或者整数就不存在, 无法成组
                _ => return false,
            }
        }
    }

    true
}

pub type SolutionFn = fn(Vec<i32>, i32) -> bool;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 3, 3, 4, 4, 5, 6];
    let k = 4;
    assert!(func(nums, k));

    let nums = vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11];
    let k = 3;
    assert!(func(nums, k));

    let nums = vec![1, 2, 3, 4];
    let k = 3;
    assert!(!func(nums, k));

    let nums = vec![1, 2, 4];
    let k = 3;
    assert!(!func(nums, k));

    let nums = vec![3, 3, 2, 2, 1, 1];
    let k = 3;
    assert!(func(nums, k));
}

fn main() {
    //check_solution(is_possible_divide1);
    //check_solution(is_possible_divide2);
    check_solution(is_possible_divide3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_possible_divide1, is_possible_divide2, is_possible_divide3};

    #[test]
    fn test_is_possible_divide1() {
        check_solution(is_possible_divide1);
    }

    #[test]
    fn test_is_possible_divide2() {
        check_solution(is_possible_divide2);
    }
    #[test]
    fn test_is_possible_divide3() {
        check_solution(is_possible_divide3);
    }
}
