// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn selection_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();
    if list.len() < 2 {
        return;
    }
    for i in 0..(len - 1) {

        // 找到最小元素的索引
        let mut min_index = i;
        for j in (i + 1)..len {
            if list[j] < list[min_index] {
                min_index = j;
            }
        }

        // 如果最小元素不是 `list[i]`, 就交换两个元素
        if i != min_index {
            list.swap(i, min_index);
        }
    }
}

/// 递归实现选择排序
pub fn selection_sort_recursive<T>(list: &mut [T])
where
    T: PartialOrd,
{
    fn get_min_index<T>(list: &[T], i: usize, len: usize) -> usize
    where
        T: PartialOrd,
    {
        if i == len - 1 {
            return i;
        }
        let j = get_min_index(list, i + 1, len);
        if list[i] < list[j] {
            i
        } else {
            j
        }
    }

    let len = list.len();
    if list.len() < 2 {
        return;
    }

    let min_index = get_min_index(list, 0, len);
    // 将最小的元素交换到最左侧
    if min_index != 0 {
        list.swap(0, min_index);
    }

    // 递归排序剩下的元素
    selection_sort_recursive(&mut list[1..]);
}

#[cfg(test)]
mod tests {
    use super::{selection_sort, selection_sort_recursive};

    #[test]
    fn test_selection_sort() {
        let mut list = [0, 5, 3, 2, 2];
        selection_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        selection_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        selection_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );
    }

    #[test]
    fn test_selection_sort_recursive() {
        let mut list = [0, 5, 3, 2, 2];
        selection_sort_recursive(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        selection_sort_recursive(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        selection_sort_recursive(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );
    }
}
