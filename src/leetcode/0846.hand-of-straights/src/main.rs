// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::BTreeMap;

// BTreeMap
// 计数, 并保证 key 是有序的
pub fn is_n_straight_hand1(hand: Vec<i32>, group_size: i32) -> bool {
    let nums = hand;
    let k = group_size;

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

// TODO(Shaohua): BinaryHeap

pub type SolutionFn = fn(Vec<i32>, i32) -> bool;

fn check_solution(func: SolutionFn) {
    let hand = vec![1, 2, 3, 6, 2, 3, 4, 7, 8];
    let group_size = 3;
    assert!(func(hand, group_size));

    let hand = vec![1, 2, 3, 4, 5];
    let group_size = 4;
    assert!(!func(hand, group_size));
}

fn main() {
    check_solution(is_n_straight_hand1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_n_straight_hand1};

    #[test]
    fn test_is_n_straight_hand1() {
        check_solution(is_n_straight_hand1);
    }
}
