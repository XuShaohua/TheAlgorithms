// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::insertion_sort::insertion_sort;
use crate::shell_sort::shell_sort;

/// Timsort 是对归并排序 (merge sort) 的优化.
pub fn timsort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    const RUN: usize = 32;

    let len = arr.len();
    if len < 2 {
        return;
    }

    // 先将数组分隔成大小相同的子数组, 并利用插入排序进行排序.
    // 插入排序比较善于处理已基本有序的较小的数组.
    for i in (0..len).step_by(RUN) {
        let end = (i + RUN).min(len);
        insertion_sort(&mut arr[i..end]);
    }

    // 然后将各个子数组合并在一起
    // 数组间隔依次是 RUN, RUN * 2, RUN * 4, ...
    let mut size = RUN;
    while size < len {
        // 合并子数组
        for left in (0..len).step_by(2 * size) {
            // 两个子数组分别是 `arr[left..=middle]` 和 `arr[middle+1..=right]`.
            let middle = left + size - 1;
            let right = (left + 2 * size - 1).min(len - 1);

            if middle < right {
                merge(arr, left, middle, right);
            }
        }

        size *= 2;
    }
}

/// 合并子数组 `arr[left..=middle]` 和 `arr[middle+1..=right]`
fn merge<T>(arr: &mut [T], left: usize, middle: usize, right: usize)
where
    T: PartialOrd + Clone,
{
    // 先创建辅助数组
    let aux_left = arr[left..=middle].to_vec();
    let aux_right = arr[middle + 1..=right].to_vec();
    let left_len = middle - left + 1;
    let right_len = right - middle;

    // 合并子数组
    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    while i < left_len && j < right_len {
        if aux_left[i] < aux_right[j] {
            arr[k].clone_from(&aux_left[i]);
            i += 1;
        } else {
            arr[k].clone_from(&aux_right[j]);
            j += 1;
        }
        k += 1;
    }

    // 最后复制剩下的元素
    while i < left_len {
        arr[k].clone_from(&aux_left[i]);
        i += 1;
        k += 1;
    }

    while j < right_len {
        arr[k].clone_from(&aux_right[j]);
        j += 1;
        k += 1;
    }
}

/// 使用希尔排序代替插入排序
///
/// 只创建一次辅助数组
pub fn shell_timsort<T>(arr: &mut [T])
where
    T: PartialOrd + Clone,
{
    const RUN: usize = 64;

    let len = arr.len();
    if len < 2 {
        return;
    }

    // 先将数组分隔成大小相同的子数组, 并利用插入排序进行排序.
    // 插入排序比较善于处理已基本有序的较小的数组.
    for i in (0..len).step_by(RUN) {
        let end = (i + RUN).min(len);
        shell_sort(&mut arr[i..end]);
    }

    // 然后将各个子数组合并在一起
    // 数组间隔依次是 RUN, RUN * 2, RUN * 4, ...
    let mut size = RUN;
    let mut aux = arr.to_vec();

    while size < len {
        // 合并子数组
        for left in (0..len).step_by(2 * size) {
            // 两个子数组分别是 `arr[left..=middle]` 和 `arr[middle+1..=right]`.
            let middle = left + size - 1;
            let right = (left + 2 * size - 1).min(len - 1);

            if middle < right {
                merge_with_aux(arr, left, middle, right, &mut aux);
            }
        }

        size *= 2;
    }
}

/// 合并子数组 `arr[left..=middle]` 和 `arr[middle+1..=right]`
fn merge_with_aux<T>(arr: &mut [T], left: usize, middle: usize, right: usize, aux: &mut [T])
where
    T: PartialOrd + Clone,
{
    // 先创建辅助数组
    for i in left..=middle {
        aux[i].clone_from(&arr[i]);
    }
    for i in (middle + 1)..=right {
        aux[i].clone_from(&arr[i]);
    }
    let left_len = middle - left + 1;
    let right_len = right - middle;

    // 合并子数组
    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    while i < left_len && j < right_len {
        let index = if aux[i] < aux[j] {
            &mut i
        } else {
            &mut j
        };
        arr[k].clone_from(&aux[*index]);
        *index += 1;
        k += 1;
    }

    // 最后复制剩下的元素
    while i < right_len {
        arr[k].clone_from(&aux[i]);
        i += 1;
        k += 1;
    }

    while j < right_len {
        arr[k].clone_from(&aux[j]);
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::timsort::{shell_timsort, timsort};

    #[test]
    fn test_timsort() {
        let mut list = [0, 5, 3, 2, 2];
        timsort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        timsort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        timsort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        timsort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }

    #[test]
    fn test_shell_timsort() {
        let mut list = [0, 5, 3, 2, 2];
        shell_timsort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        shell_timsort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        shell_timsort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        shell_timsort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}