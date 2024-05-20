// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use sort::util::{is_sorted, read_ints, show_brief};

pub fn quick_sort<T: Clone + PartialOrd>(nums: &mut [T]) {
    if !nums.is_empty() {
        quick_sort_helper(nums, 0, nums.len() - 1);
    }
}

fn quick_sort_helper<T: Clone + PartialOrd>(nums: &mut [T], low: usize, high: usize) {
    if low < high {
        // 按照基数的位置, 将数组划分成左右两个子数组.
        let pivot = quick_sort_partition(nums, low, high);
        // 对左右两个子数组分别进行递归快速排序
        if pivot > low + 1 {
            quick_sort_helper(nums, low, pivot - 1);
        }
        if pivot + 1 < high {
            quick_sort_helper(nums, pivot + 1, high);
        }
    }
}

fn quick_sort_partition<T: Clone + PartialOrd>(nums: &mut [T], low: usize, high: usize) -> usize {
    let pivot = nums[low].clone();
    // 使用双指针法遍历数组, 以 pivot 为基准, 把数组划分成三部分:
    // 小于 pivot, pivot, 大于 pivot

    let mut left = low;
    let mut right = high;
    while left < right {
        // 更新 right 的位置, 使得 nums[right] 大于 pivot
        while left < right && nums[right] >= pivot {
            right -= 1;
        }

        // 更新 left 的位置, 使得 nums[left] 大于 pivot
        while left < right && nums[left] <= pivot {
            left += 1;
        }
        // 交换元素
        nums.swap(left, right);
    }

    // 最后, 把基准数 pivot 移到合适的位置. 此时, 数组中元素的顺序满足以下条件:
    // 小于 pivot, pivot, 大于 pivot
    nums.swap(low, left);
    // 返回的是 pivot 所在的位置
    left
}

fn main() {
    // TODO(Shaohua): Remove quick_sort()
    let mut list = read_ints();
    println!("[QuickSort] LIST:");
    show_brief(&list);
    quick_sort(&mut list);
    println!("RESULT:");
    assert!(is_sorted(&list));
    show_brief(&list);
}
