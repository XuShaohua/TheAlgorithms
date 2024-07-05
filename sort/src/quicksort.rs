// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(dead_code)]

#[inline]
pub fn quicksort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() < 2 {
        return;
    }
    quicksort_helper(arr, 0, arr.len() - 1);
}

fn quicksort_helper<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) {
    if low >= high {
        return;
    }

    // 按照基数的位置, 将数组划分成左右两个子数组.
    let pivot_index = partition_pivot_at_right(arr, low, high);
    // 对左右两个子数组分别执行快速排序
    if pivot_index > low + 1 {
        quicksort_helper(arr, low, pivot_index - 1);
    }
    if pivot_index + 1 < high {
        quicksort_helper(arr, pivot_index + 1, high);
    }
}

// 选择最右侧的元素作为基准值
fn partition_pivot_at_right<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot_index = high;

    // 以 pivot 为基准, 把数组划分成三部分: 小于 pivot, pivot, 大于等于 pivot
    // i 用于标记比 pivot 大的元素
    let mut i = low;
    // j 用于遍历整个数组
    for j in low..high {
        if arr[j] < arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }

    // 最后把基准值 pivot 移到合适的位置.
    // 此时, 数组中元素的顺序满足以下条件: 小于 pivot, pivot, 大于等于 pivot
    arr.swap(i, pivot_index);
    // 返回的是 pivot 所在的位置
    i
}

/// 选择最左侧的元素作为基准值
fn partition_pivot_at_left<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot_index = low;

    // 以 pivot 为基准, 把数组划分成三部分: 小于等于 pivot, pivot, 大于 pivot
    // i 用于标记比 pivot 大的元素
    let mut i = high;
    // j 用于遍历整个数组
    for j in ((low + 1)..=high).rev() {
        if arr[j] > arr[pivot_index] {
            arr.swap(i, j);
            i -= 1;
        }
    }

    // 最后把基准值 pivot 移到合适的位置.
    // 此时, 数组中元素的顺序满足以下条件: 小于等于 pivot, pivot, 大于 pivot
    arr.swap(i, pivot_index);
    // 返回的是 pivot 所在的位置
    i
}

/// 使用双指针法选择最左侧的元素作为基准值
fn partition_with_two_pointers<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot_index = low;

    // 使用双指针法遍历数组, 以 pivot 为基准, 把数组划分成三部分:
    // 小于等于 pivot, pivot, 大于 pivot
    let mut left: usize = low;
    let mut right: usize = high;
    while left < right {
        // right 的位置左移, 直到 arr[right] 小于等于 pivot
        while left < right && arr[right] > arr[pivot_index] {
            right -= 1;
        }

        // left 的位置右移, 直到 arr[left] 大于 pivot
        while left < right && arr[left] <= arr[pivot_index] {
            left += 1;
        }

        // 交换元素
        arr.swap(left, right);
    }

    // 最后把基准值 pivot 移到合适的位置.
    // 此时, 数组中元素的顺序满足以下条件: 小于等于 pivot, pivot, 大于 pivot
    arr.swap(left, pivot_index);
    // 返回的是 pivot 所在的位置
    left
}

#[cfg(test)]
mod tests {
    use crate::quicksort::quicksort;

    #[test]
    fn test_quicksort() {
        let mut list = [1, 8, 3, 9, 4];
        quicksort(&mut list);
        assert_eq!(list, [1, 3, 4, 8, 9]);

        let mut list = [0, 5, 3, 2, 2];
        quicksort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        quicksort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        quicksort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        quicksort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
