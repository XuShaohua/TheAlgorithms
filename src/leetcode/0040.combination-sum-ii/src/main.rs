// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn combination_sum1(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    #[allow(clippy::comparison_chain)]
    fn backtrace(
        nums: &[i32],
        index: usize,
        target: i32,
        visited: &mut [bool],
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        // 终止条件是, 当前的路径中元素的和大于或等于 target
        let sum: i32 = path.iter().sum();
        if sum == target {
            res.push(path.clone());
            return;
        } else if sum > target {
            // 元素之和太大了, 直接忽略.
            return;
        }

        // 遍历剩下的所有元素
        for i in index..nums.len() {
            // 跳过重复元素.
            if i > 0 && nums[i] == nums[i - 1] && !visited[i - 1] {
                continue;
            }

            if !visited[i] {
                // 访问元素
                path.push(nums[i]);
                visited[i] = true;

                // 遍历搜索
                backtrace(nums, i + 1, target, visited, path, res);

                // 撤销访问
                path.pop();
                visited[i] = false;
            }
        }
    }

    debug_assert!((1..=30).contains(&target));
    debug_assert!(!candidates.is_empty());

    // 先对数组进行排序, 方便处理重复的元素.
    let mut nums = candidates;
    nums.sort_unstable();

    // 存放所有结果.
    let mut res: Vec<Vec<i32>> = Vec::new();
    // 存放当前的路径.
    let mut path: Vec<i32> = Vec::new();
    // 记录元素是否被访问.
    let mut visited = vec![false; nums.len()];
    backtrace(&nums, 0, target, &mut visited, &mut path, &mut res);
    res
}

// Backtracking
pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    #[allow(clippy::comparison_chain)]
    fn backtrack(nums: &[i32], target: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if target == 0 {
            res.push(path.clone());
            return;
        } else if target < 0 {
            return;
        }

        let mut previous = i32::MIN;
        // 遍历所有元素.
        for (index, &num) in nums.iter().enumerate() {
            // 忽略重复元素.
            if previous == num {
                continue;
            }

            // 访问元素
            path.push(num);

            // 递归搜索
            backtrack(&nums[index + 1..], target - num, path, res);

            // 撤销访问
            path.pop();
            previous = num;
        }
    }

    let mut nums = candidates;
    nums.sort_unstable();

    let mut res = Vec::new();
    let mut path: Vec<i32> = Vec::new();
    backtrack(&nums, target, &mut path, &mut res);
    res
}

pub type SolutionFn = fn(Vec<i32>, i32) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let candidates = vec![10, 1, 2, 7, 6, 1, 5];
    let target = 8;
    let expected = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
    assert_eq!(func(candidates, target), expected);

    let candidates = vec![2, 5, 2, 1, 2];
    let target = 5;
    let expected = vec![vec![1, 2, 2], vec![5]];
    assert_eq!(func(candidates, target), expected);
}

fn main() {
    check_solution(combination_sum1);
    check_solution(combination_sum2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, combination_sum1, combination_sum2};

    #[test]
    fn test_combination_sum1() {
        check_solution(combination_sum1);
    }

    #[test]
    fn test_combination_sum2() {
        check_solution(combination_sum2);
    }
}
