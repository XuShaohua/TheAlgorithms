// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use crate::insertion_sort::insertion_sort;
use crate::shell_sort::shell_sort;

pub fn merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    topdown_merge_sort(arr);
}

/// 对于元素个数为 `N` 的数组, 自顶向下的归并排序 (top-down merge sort) 最多使用 `N log(N)` 次比较
/// 以及 `6N log(N)` 次元素访问操作.
pub fn topdown_merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
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
    let aux = arr[..=high].to_vec();

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

/// 对于元素数较少的数组, 使用插入排序
pub fn insertion_merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    const CUTOFF: usize = 24;
    let mut aux = arr.to_vec();
    sort_cutoff_with_insertion(arr, 0, arr.len() - 1, CUTOFF, &mut aux);
}

/// 排序 `arr[low..=high]` 部分, 如果元数较少, 就使用插入排序.
fn sort_cutoff_with_insertion<T>(arr: &mut [T], low: usize, high: usize, cutoff: usize, aux: &mut Vec<T>)
where
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
fn merge_with_aux<T>(arr: &mut [T], low: usize, middle: usize, high: usize, aux: &mut Vec<T>)
where
    T: PartialOrd + Clone,
{
    // 辅助数组, 先将数组复制一份.
    aux.clear();
    for item in &arr[..=high] {
        aux.push(item.clone());
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
    const CUTOFF: usize = 48;
    let mut aux = arr.to_vec();
    sort_cutoff_with_shell(arr, 0, arr.len() - 1, CUTOFF, &mut aux);
}

/// 排序 `arr[low..=high]` 部分, 如果元数较少, 就使用希尔排序.
fn sort_cutoff_with_shell<T>(arr: &mut [T], low: usize, high: usize, cutoff: usize, aux: &mut Vec<T>)
where
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

#[cfg(test)]
mod tests {
    use super::{insertion_merge_sort, shell_merge_sort, topdown_merge_sort};

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
}
