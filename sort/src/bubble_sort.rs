// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// 如果传入的数据是增序排好的, 那么只需要 `n-1` 次的比较, 以及 0 次的交换;
/// 平珓情况以及最坏情况下, 使用 `n^2 / 2` 次比较以及 `n^2 / 2` 次交换.
pub fn bubble_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let len = arr.len();
    for i in 0..len {
        let mut swapped = false;
        // 以 (len - i - 1) 为分隔点, 左侧部分是无序的, 右侧部分是有序的
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                swapped = true;
                arr.swap(j, j + 1);
            }
        }

        // 如果没有元素需要交换, 说明左侧部分也是有序的
        if !swapped {
            break;
        }
    }
}

/// 递归形式的冒泡排序算法.
///
/// 与迭代形式的算法相比, 递归形式实现的算法, 并没有性能上的优势.
pub fn recursive_bubble_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();
    if len < 2 {
        return;
    }

    let mut swapped = false;
    for j in 0..(len - 1) {
        if list[j] > list[j + 1] {
            swapped = true;
            list.swap(j, j + 1);
        }
    }

    // 如果没有元素需要交换, 说明数组有序的
    if !swapped {
        return;
    }

    recursive_bubble_sort(&mut list[..(len - 1)]);
}

#[cfg(test)]
mod tests {
    use super::{bubble_sort, recursive_bubble_sort};

    #[test]
    fn test_bubble_sort() {
        let mut list = [0, 5, 3, 2, 2];
        bubble_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        bubble_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        bubble_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        bubble_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }

    #[test]
    fn test_recursive_bubble_sort() {
        let mut list = [0, 5, 3, 2, 2];
        recursive_bubble_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        recursive_bubble_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        recursive_bubble_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        recursive_bubble_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
