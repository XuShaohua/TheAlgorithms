// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Greedy
pub fn can_jump1(nums: Vec<i32>) -> bool {
    debug_assert!((1..=10_000).contains(&nums.len()));

    // 最远跳跃距离
    let mut max_pos: usize = 0;

    for (index, &num) in nums.iter().enumerate() {
        let num_usize: usize = num as usize;

        // max_pos >= index, 表示能从之前的节点跳到这里.
        // index + num_usize > max_pos, 表示基于当前的点, 可以跳得更远.
        if max_pos >= index && index + num_usize > max_pos {
            // 更新最远跳跃距离
            max_pos = index + num_usize;
        }
    }
    // 是否遍历完数组, 到达最远的位置
    max_pos + 1 >= nums.len()
}

pub fn can_jump2(nums: Vec<i32>) -> bool {
    debug_assert!((1..=10_000).contains(&nums.len()));

    // 最远跳跃距离
    let mut max_pos: usize = 0;

    for (index, &num) in nums.iter().enumerate() {
        // index > max_pos, 表示能从之前的节点无法跳到这里.
        if index > max_pos {
            return false;
        }

        // index + num_usize > max_pos, 表示基于当前的点, 可以跳得更远.
        let num_usize: usize = num as usize;
        if max_pos >= index && index + num_usize > max_pos {
            // 更新最远跳跃距离.
            max_pos = index + num_usize;

            // 如果这次跳得足够远, 超过了数组长度, 就直接提前返回 true.
            if max_pos + 1 >= nums.len() {
                return true;
            }
        }
    }

    true
}

pub type SolutionFn = fn(Vec<i32>) -> bool;

fn check_solution(func: SolutionFn) {
    let nums = vec![2, 3, 1, 1, 4];
    assert!(func(nums));

    let nums = vec![3, 2, 1, 0, 4];
    assert!(!func(nums));
}

fn main() {
    check_solution(can_jump1);
    check_solution(can_jump2);
}

#[cfg(test)]
mod tests {
    use super::{can_jump1, can_jump2, check_solution};

    #[test]
    fn test_can_jump1() {
        check_solution(can_jump1);
    }

    #[test]
    fn test_can_jump2() {
        check_solution(can_jump2);
    }
}
