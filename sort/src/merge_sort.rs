// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use crate::insertion_sort::insertion_sort;
use crate::shell_sort::shell_sort;

#[inline]
pub fn merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    topdown_merge_sort(arr);
}

/// 对于元素个数为 `N` 的数组, 自顶向下的归并排序 (top-down merge sort)
/// 最多使用 `N log(N)` 次比较以及 `6N log(N)` 次元素访问操作.
#[inline]
pub fn topdown_merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    if arr.is_empty() {
        return;
    }
    sort(arr, 0, arr.len() - 1);
}

/// 排序 `arr[low..=high]` 部分.
fn sort<T>(arr: &mut [T], low: usize, high: usize)
where
    T: PartialOrd + Clone,
{
    if low >= high {
        return;
    }

    let middle = low + (high - low) / 2;

    // 递归排序左侧部分数组
    sort(arr, low, middle);
    // 递归排序右侧部分数组
    sort(arr, middle + 1, high);

    // 合并左右两侧部分数组
    if arr[middle] > arr[middle + 1] {
        merge(arr, low, middle, high);
    }
}

/// 合并 `arr[low..=middle]` 以及 `arr[middle+1..=high]` 两个子数组.
///
/// 它不是原地合并.
#[allow(clippy::needless_range_loop)]
fn merge<T>(arr: &mut [T], low: usize, middle: usize, high: usize)
where
    T: PartialOrd + Clone,
{
    // 辅助数组, 先将数组复制一份.
    let aux = arr[low..=high].to_vec();

    // 再合并回原数组.
    let mut i = low;
    let mut j = middle + 1;

    for k in low..=high {
        if i > middle {
            arr[k] = aux[j - low].clone();
            j += 1;
        } else if j > high {
            arr[k] = aux[i - low].clone();
            i += 1;
        } else if aux[j - low] < aux[i - low] {
            arr[k] = aux[j - low].clone();
            j += 1;
        } else {
            arr[k] = aux[i - low].clone();
            i += 1;
        }
    }
}

/// 对于元素数较少的数组, 使用插入排序
pub fn insertion_merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    if arr.is_empty() {
        return;
    }
    let cutoff: usize = 24;
    let mut aux = arr.to_vec();
    sort_cutoff_with_insertion(arr, 0, arr.len() - 1, cutoff, &mut aux);
}

/// 排序 `arr[low..=high]` 部分, 如果元数较少, 就使用插入排序.
fn sort_cutoff_with_insertion<T>(
    arr: &mut [T],
    low: usize,
    high: usize,
    cutoff: usize,
    aux: &mut Vec<T>,
) where
    T: PartialOrd + Clone,
{
    if low >= high {
        return;
    }

    if high - low <= cutoff {
        insertion_sort(&mut arr[low..=high]);
        return;
    }

    let middle = low + (high - low) / 2;

    // 递归排序左侧部分数组
    sort_cutoff_with_insertion(arr, low, middle, cutoff, aux);
    // 递归排序右侧部分数组
    sort_cutoff_with_insertion(arr, middle + 1, high, cutoff, aux);

    // 合并左右两侧部分数组
    if arr[middle] > arr[middle + 1] {
        merge_with_aux(arr, low, middle, high, aux);
    }
}

/// 合并 `arr[low..=middle]` 以及 `arr[middle+1..=high]` 两个子数组.
///
/// 它不是原地合并.
#[allow(clippy::needless_range_loop)]
fn merge_with_aux<T>(arr: &mut [T], low: usize, middle: usize, high: usize, aux: &mut [T])
where
    T: PartialOrd + Clone,
{
    // 辅助数组, 先将数组复制一份.
    for index in low..=high {
        aux[index].clone_from(&arr[index]);
    }

    // 再合并回原数组.
    let mut i = low;
    let mut j = middle + 1;

    for k in low..=high {
        if i > middle {
            arr[k] = aux[j].clone();
            j += 1;
        } else if j > high {
            arr[k] = aux[i].clone();
            i += 1;
        } else if aux[j] < aux[i] {
            arr[k] = aux[j].clone();
            j += 1;
        } else {
            arr[k] = aux[i].clone();
            i += 1;
        }
    }
}

/// 对于元素数较少的数组, 使用希尔排序
pub fn shell_merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    if arr.is_empty() {
        return;
    }

    let cutoff: usize = 72;
    let mut aux = arr.to_vec();
    sort_cutoff_with_shell(arr, 0, arr.len() - 1, cutoff, &mut aux);
}

/// 排序 `arr[low..=high]` 部分, 如果元数较少, 就使用希尔排序.
fn sort_cutoff_with_shell<T>(
    arr: &mut [T],
    low: usize,
    high: usize,
    cutoff: usize,
    aux: &mut Vec<T>,
) where
    T: PartialOrd + Clone,
{
    if low >= high {
        return;
    }

    if high - low <= cutoff {
        shell_sort(&mut arr[low..=high]);
        return;
    }

    let middle = low + (high - low) / 2;

    // 递归排序左侧部分数组
    sort_cutoff_with_shell(arr, low, middle, cutoff, aux);
    // 递归排序右侧部分数组
    sort_cutoff_with_shell(arr, middle + 1, high, cutoff, aux);

    // 合并左右两侧部分数组
    if arr[middle] > arr[middle + 1] {
        merge_with_aux(arr, low, middle, high, aux);
    }
}

/// 迭代形式的归并排序, 自底向上 bottom-up merge sort
pub fn bottom_up_merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    let len = arr.len();
    if len < 2 {
        return;
    }

    let mut aux = arr.to_vec();

    // 开始排序的数组大小, 从 1 到 len / 2
    // current_size 的取值是 1, 2, 4, 8, ...
    let mut current_size = 1;

    while current_size < len {
        // 归并排序的数组左侧索引
        let mut left_start = 0;

        // 子数组的起始点不同, 这样就可以遍历整个数组.
        // left_start 的取值是 0, 2 * current_size, 4 * current_size, ...
        // right_end 的取值是 2 * current_size, 4 * current_size, 6 * current_size, ...
        while left_start < len - 1 {
            let middle = (left_start + current_size - 1).min(len - 1);
            let right_end = (left_start + 2 * current_size - 1).min(len - 1);

            // 合并左右两侧部分数组
            merge_with_aux(arr, left_start, middle, right_end, &mut aux);

            left_start += 2 * current_size;
        }

        current_size *= 2;
    }
}

/// 三路归并排序
pub fn three_way_merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    let mut aux = arr.to_vec();
    three_way_sort(arr, 0, arr.len(), &mut aux);
}

/// 三路排序 `arr[low..high]`
fn three_way_sort<T>(arr: &mut [T], low: usize, high: usize, aux: &mut Vec<T>)
where
    T: PartialOrd + Clone,
{
    if low + 2 >= high {
        return;
    }

    // 将数组分成三部分
    let middle1 = low + (high - low) / 3;
    let middle2 = low + 2 * (high - low) / 3 + 1;
    println!("low: {low}, middle1: {middle1}, middle2: {middle2}, high: {high}");

    // 递归排序各部分数组
    three_way_sort(arr, low, middle1, aux);
    three_way_sort(arr, middle1, middle2, aux);
    three_way_sort(arr, middle2, high, aux);

    // 合并三部分数组
    three_way_merge(arr, low, middle1, middle2, high, aux);
}

/// 合并 `arr[low..middle1]`, `arr[middle1..middle2]` 以及 `arr[middle2..high]` 三个子数组.
///
/// 它不是原地合并.
#[allow(clippy::needless_range_loop)]
fn three_way_merge<T>(
    arr: &mut [T],
    low: usize,
    middle1: usize,
    middle2: usize,
    high: usize,
    aux: &mut [T],
) where
    T: PartialOrd + Clone,
{
    // 辅助数组, 先将数组复制一份.
    aux.clone_from_slice(&arr[low..high]);

    // 再合并回原数组.
    let mut i = low;
    let mut j = middle1;
    let mut k = middle2;
    let mut l = low;

    // 首先合并较小的子数组
    while i < middle1 && j < middle2 && k < high {
        let curr_index = if aux[i] < aux[j] && aux[i] < aux[k] {
            &mut i
        } else if aux[j] < aux[k] {
            &mut j
        } else {
            &mut k
        };
        arr[l].clone_from(&aux[*curr_index]);
        *curr_index += 1;
        l += 1;
    }

    // 然后合并剩余部分的子数组
    while i < middle1 && j < middle2 {
        let curr_index = if aux[i] < aux[j] {
            &mut i
        } else {
            &mut j
        };
        arr[l].clone_from(&aux[*curr_index]);
        *curr_index += 1;
        l += 1;
    }

    while j < middle2 && k < high {
        let curr_index = if aux[j] < aux[k] {
            &mut j
        } else {
            &mut k
        };
        arr[l].clone_from(&aux[*curr_index]);
        *curr_index += 1;
        l += 1;
    }

    while i < middle1 && k < high {
        let curr_index = if aux[i] < aux[k] {
            &mut i
        } else {
            &mut k
        };
        arr[l].clone_from(&aux[*curr_index]);
        *curr_index += 1;
        l += 1;
    }

    while i < middle1 {
        arr[l].clone_from(&aux[i]);
        i += 1;
        l += 1;
    }
    while j < middle1 {
        arr[l].clone_from(&aux[j]);
        j += 1;
        l += 1;
    }
    while k < middle1 {
        arr[l].clone_from(&aux[k]);
        k += 1;
        l += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{bottom_up_merge_sort, insertion_merge_sort, shell_merge_sort, three_way_merge_sort, topdown_merge_sort};

    #[test]
    fn test_topdown_merge_sort() {
        let mut list = [0, 5, 3, 2, 2];
        topdown_merge_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        topdown_merge_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        topdown_merge_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        topdown_merge_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }

    #[test]
    fn test_insertion_merge_sort() {
        let mut list = [0, 5, 3, 2, 2];
        insertion_merge_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        insertion_merge_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        insertion_merge_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        insertion_merge_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }

    #[test]
    fn test_shell_merge_sort() {
        let mut list = [0, 5, 3, 2, 2];
        shell_merge_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        shell_merge_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        shell_merge_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        shell_merge_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }

    #[test]
    fn test_bottom_up_merge_sort() {
        let mut list = [0, 5, 3, 2, 2];
        bottom_up_merge_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        bottom_up_merge_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        bottom_up_merge_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        bottom_up_merge_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }

    #[test]
    fn test_three_way_merge_sort() {
        let mut list = [0, 5, 3, 2, 2];
        three_way_merge_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        three_way_merge_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        three_way_merge_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        three_way_merge_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
