// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

// Backtracking
pub fn subsets1(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtracking(nums: &[i32], index: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        // 先将当前的集合存到结果中.
        res.push(path.clone());

        // 终止条件, 就是访问了数组中的所有元素.
        if index >= nums.len() {
            return;
        }

        for i in index..nums.len() {
            // 选择元素
            path.push(nums[i]);
            // 递归搜索
            backtracking(nums, i + 1, path, res);
            // 撤销选择的元素
            path.pop();
        }
    }
    // 记录所有结果
    let mut res: Vec<Vec<i32>> = Vec::new();
    // 记录当前的路径
    let mut path: Vec<i32> = Vec::new();
    backtracking(&nums, 0, &mut path, &mut res);
    res
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 3];
    let mut expected = vec![
        vec![],
        vec![1],
        vec![2],
        vec![3],
        vec![1, 2],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ];
    let mut out = func(nums);
    out.sort();
    expected.sort();
    assert_eq!(out, expected);

    let nums = vec![0];
    let expected = vec![vec![], vec![0]];
    assert_eq!(func(nums), expected);
}

fn main() {
    check_solution(subsets1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, subsets1};

    #[test]
    fn test_subsets1() {
        check_solution(subsets1);
    }
}
