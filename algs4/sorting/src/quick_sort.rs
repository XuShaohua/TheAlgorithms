// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::generics::exch;

/// 最好情况: O(NlgN)
/// 最差情况: 1/2 * N^2
/// 平均情况: 1.39NlgN
/// 比 merge sort 要快, 因为不需要一个额外的数组, 减少了数据移动操作.
/// 不是stable sort.
///
/// 优化:
/// 1. 当 subarray 元素小于 10 时, 可以使用 insertion sort
/// 2. 在分区时选择 median 作为 pivot, median-of-3 random items.
pub fn quick_sort<T>(vec: &mut Vec<T>)
where
    T: Clone + PartialOrd,
{
    quick_sort_helper(vec, 0, vec.len() - 1);
}

fn quick_sort_helper<T>(vec: &mut Vec<T>, low: usize, high: usize)
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

fn partition<T>(vec: &mut Vec<T>, low: usize, high: usize) -> usize
where
    T: Clone + PartialOrd,
{
    let pivot = vec[low].clone();
    let mut i = high + 1;
    let mut j: usize = high;
    while j > low {
        if vec[j] > pivot {
            i -= 1;
            exch(vec, i, j);
        }
        j -= 1;
    }
    exch(vec, i - 1, low);
    return i - 1;
}
