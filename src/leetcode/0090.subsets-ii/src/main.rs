// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn subsets_with_dup1(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(
        nums: &[i32],
        index: usize,
        visited: &mut [bool],
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        // 先把当前的路径加入到结果中.
        res.push(path.clone());

        // 遍历剩下的所有元素.
        for i in index..nums.len() {
            // 跳过相同的元素被重复访问的.
            if i > 0 && nums[i] == nums[i - 1] && !visited[i - 1] {
                continue;
            }

            if !visited[i] {
                // 访问当前元素
                path.push(nums[i]);
                visited[i] = true;

                // 递归搜索
                backtrack(nums, i + 1, visited, path, res);

                // 撤销访问
                path.pop();
                visited[i] = false;
            }
        }
    }

    debug_assert!((1..=10).contains(&nums.len()));

    // 先对数组排序, 方便处理重复元素.
    let mut nums = nums;
    nums.sort_unstable();
    // 保存所有结果
    let mut res = Vec::new();
    // 保存当前的路径
    let mut path = Vec::new();
    // 记录哪些重复的元素已经被访问.
    let mut visited = vec![false; nums.len()];

    backtrack(&nums, 0, &mut visited, &mut path, &mut res);
    res
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 2];
    let expected = vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 2],
        vec![2],
        vec![2, 2],
    ];
    assert_eq!(func(nums), expected);

    let nums = vec![0];
    let expected = vec![vec![], vec![0]];
    assert_eq!(func(nums), expected);
}

fn main() {
    check_solution(subsets_with_dup1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, subsets_with_dup1};

    #[test]
    fn test_subsets_with_dup1() {
        check_solution(subsets_with_dup1);
    }
}
