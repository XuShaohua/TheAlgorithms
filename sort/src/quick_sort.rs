// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// 最好情况: `O(n log(n))`
/// 最差情况: `O(n^2 / 2)`
/// 平均情况: `O(1.39 n log(n))`
/// 比 merge sort 要快, 因为不需要一个额外的数组, 减少了数据移动操作.
/// 不是stable sort.
///
/// 优化:
/// 1. 当 sub-array 元素小于 10 时, 可以使用 insertion sort
/// 2. 在分区时选择 median 作为 pivot, median-of-3 random items.
pub fn quick_sort<T>(arr: &mut [T])
where
    T: Clone + PartialOrd,
{
    if !arr.is_empty() {
        quick_sort_helper(arr, 0, arr.len() - 1);
    }
}

fn quick_sort_helper<T>(vec: &mut [T], low: usize, high: usize)
where
    T: Clone + PartialOrd,
{
    if low < high {
        let pivot_index = partition(vec, low, high);
        if pivot_index > low {
            quick_sort_helper(vec, low, pivot_index - 1);
        }
        quick_sort_helper(vec, pivot_index + 1, high);
    }
}

fn partition<T>(vec: &mut [T], low: usize, high: usize) -> usize
where
    T: Clone + PartialOrd,
{
    let pivot = vec[low].clone();
    let mut i: usize = low;
    let mut j: usize = high;
    while i < j {
        while i < j && vec[i] <= pivot {
            i += 1;
        }
        while i < j && vec[j] >= pivot {
            j -= 1;
        }
        vec.swap(i, j);
    }
    vec.swap(i, low);
    i
}
