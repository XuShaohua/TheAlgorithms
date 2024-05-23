// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn combination_sum1(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    #[allow(clippy::comparison_chain)]
    fn backtrack(
        nums: &[i32],
        index: usize,
        target: i32,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        // 终止的条件是, 当前 path 中所有元素之和等于 target.
        let sum: i32 = path.iter().sum();
        if sum == target {
            res.push(path.clone());
            return;
        } else if sum > target {
            // 当前组合不合适, 超过了 target
            return;
        }

        // 遍历数组中的所有元素.
        for i in index..nums.len() {
            // 访问元素
            path.push(nums[i]);

            // 递归搜索
            backtrack(nums, i, target, path, res);

            // 撤销访问
            path.pop();
        }
    }

    debug_assert!((1..=40).contains(&target));
    debug_assert!(!candidates.is_empty());

    // 记录所有结果
    let mut res: Vec<Vec<i32>> = Vec::new();
    // 记录当前的组合.
    let mut path: Vec<i32> = Vec::new();
    backtrack(&candidates, 0, target, &mut path, &mut res);
    res
}

pub type SolutionFn = fn(Vec<i32>, i32) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    let expected = vec![vec![2, 2, 3], vec![7]];
    assert_eq!(func(candidates, target), expected);

    let candidates = vec![2, 3, 5];
    let target = 8;
    let expected = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
    assert_eq!(func(candidates, target), expected);

    let candidates = vec![2];
    let expected: Vec<Vec<i32>> = vec![];
    let target = 1;
    assert_eq!(func(candidates, target), expected);
}

fn main() {
    check_solution(combination_sum1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, combination_sum1};

    #[test]
    fn test_combination_sum1() {
        check_solution(combination_sum1);
    }
}
