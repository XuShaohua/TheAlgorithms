// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn combination_sum1(nums: Vec<i32>, target: i32) -> i32 {
    fn backtracking(
        nums: &[i32],
        target: i32,
        max_level: usize,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        println!("target: {target}, res count: {}", res.len());
        // 终止条件就是 target 不大于 0.
        if target == 0 {
            res.push(path.clone());
        }
        if target <= 0 || path.len() >= max_level {
            return;
        }

        // 遍历所有元素.
        for &num in nums.iter() {
            if num <= target && path.len() < max_level {
                // 访问元素
                path.push(num);

                // 递归搜索
                backtracking(nums, target - num, max_level, path, res);

                // 撤销访问
                path.pop();
            }
        }
    }

    debug_assert!((1..=200).contains(&nums.len()));
    debug_assert!((1..=1000).contains(&target));

    let mut nums = nums;
    nums.sort_unstable();

    let max_level: usize = (target / nums[0] + 1) as usize;

    // 保存所有结果.
    let mut res: Vec<Vec<i32>> = Vec::new();
    // 保存当前路径.
    let mut path: Vec<i32> = Vec::new();

    backtracking(&nums, target, max_level, &mut path, &mut res);
    //println!("res: {res:?}");
    res.len() as i32
}

pub fn combination_sum2(nums: Vec<i32>, target: i32) -> i32 {
    fn backtracking(nums: &[i32], target: i32, path: &mut Vec<i32>, count: &mut i32) {
        println!("target: {target}, res count: {count}");
        // 终止条件就是 target 不大于 0.
        if target == 0 {
            *count += 1;
        }
        if target <= 0 {
            return;
        }

        // 遍历所有元素.
        for &num in nums.iter() {
            if num <= target {
                // 访问元素
                path.push(num);

                // 递归搜索
                backtracking(nums, target - num, path, count);

                // 撤销访问
                path.pop();
            }
        }
    }

    debug_assert!((1..=200).contains(&nums.len()));
    debug_assert!((1..=1000).contains(&target));

    // 保存所有结果.
    let mut count: i32 = 0;
    // 保存当前路径.
    let mut path: Vec<i32> = Vec::new();

    backtracking(&nums, target, &mut path, &mut count);
    count
}

pub type SolutionFn = fn(Vec<i32>, i32) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 3];
    let target = 4;
    assert_eq!(func(nums, target), 7);

    let nums = vec![9];
    let target = 3;
    assert_eq!(func(nums, target), 0);

    // FIXME(Shaohua): Timeout
    //let nums = vec![1, 2, 3];
    //let target = 32;
    //assert_eq!(func(nums, target), 0);

    //let nums = vec![4, 2, 1];
    //let target = 32;
    //assert_eq!(func(nums, target), 0);
}

fn main() {
    check_solution(combination_sum1);
    check_solution(combination_sum2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, combination_sum1};

    #[test]
    fn test_combination_sum1() {
        check_solution(combination_sum1);
    }
}
