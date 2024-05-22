// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Builtin sorting
pub fn sort_array1(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();
    nums
}

// Merge sort
pub fn sort_array2(nums: Vec<i32>) -> Vec<i32> {
    fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
        let mut nums = Vec::with_capacity(left.len() + right.len());

        let mut left_index = 0;
        let mut right_index = 0;
        // 同时遍历两个数组, 将较小的数放到 nums 数组.
        while left_index < left.len() && right_index < right.len() {
            if left[left_index] < right[right_index] {
                nums.push(left[left_index]);
                left_index += 1;
            } else {
                nums.push(right[right_index]);
                right_index += 1;
            }
        }

        // 如果 left 数组还有剩余, 就把它们都合并到 nums.
        while left_index < left.len() {
            nums.push(left[left_index]);
            left_index += 1;
        }

        // 如果 right 数组还有剩余, 就把它们都合并到 nums.
        while right_index < right.len() {
            nums.push(right[right_index]);
            right_index += 1;
        }

        nums
    }

    fn merge_sort(nums: &[i32]) -> Vec<i32> {
        // 如果数组元素不超过 1 个, 就直接返回它
        if nums.len() < 2 {
            return nums.to_vec();
        }

        let middle = nums.len() / 2;
        let left_nums = merge_sort(&nums[..middle]);
        let right_nums = merge_sort(&nums[middle..]);
        merge(&left_nums, &right_nums)
    }

    merge_sort(&nums)
}

// Quick sort
// FIXME(Shaohua): Time out
pub fn sort_array3(nums: Vec<i32>) -> Vec<i32> {
    fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
        // 选择左侧第一个元素为基数 pivot
        let pivot = nums[low];

        // 使用双指针法, 以 pivot 为基准, 重新排布数组中的元素, 大的在右边,
        // 小的在左边

        let mut left: usize = low;
        let mut right: usize = high;
        while left < right {
            // 找到右侧第一个比 pivot 小的数
            while left < right && nums[right] >= pivot {
                right -= 1;
            }

            // 找到左侧第一个比 pivot 大的数
            while left < right && nums[left] <= pivot {
                left += 1;
            }
            nums.swap(left, right);
        }

        // 把基数放在中间合适的位置
        nums.swap(left, low);

        // 返回基数的索引位置
        left
    }

    fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
        if low < high {
            // 计算基数
            let pivot: usize = partition(nums, low, high);
            // 对左右两侧数组单独排序
            if low + 1 < pivot {
                quick_sort(nums, low, pivot - 1);
            }
            quick_sort(nums, pivot + 1, high);
        }
    }

    if nums.is_empty() {
        return nums;
    }
    let mut nums = nums;
    let len = nums.len();
    quick_sort(&mut nums, 0, len - 1);
    nums
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums = vec![5, 2, 3, 1];
    assert_eq!(func(nums), [1, 2, 3, 5]);

    let nums = vec![5, 1, 1, 2, 0, 0];
    assert_eq!(func(nums), [0, 0, 1, 1, 2, 5]);
}

fn main() {
    check_solution(sort_array1);
    check_solution(sort_array2);
    check_solution(sort_array3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, sort_array1, sort_array2, sort_array3};

    #[test]
    fn test_sort_array1() {
        check_solution(sort_array1);
    }

    #[test]
    fn test_sort_array2() {
        check_solution(sort_array2);
    }

    #[test]
    fn test_sort_array3() {
        check_solution(sort_array3);
    }
}
