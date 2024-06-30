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

/// 选择排序的一个小优化.
///
/// 将最小的元素放在左侧的同时, 将最大的元素放在右侧.
pub fn selection_sort_min_max<T>(list: &mut [T])
where
    T: PartialOrd + std::fmt::Debug,
{
    let len = list.len();
    if list.len() < 2 {
        return;
    }

    let mut start = 0;
    let mut end = len - 1;
    while start < end {
        // 找到最小元素的索引
        let mut min_index = start;
        let mut max_index = start;
        for i in start..=end {
            if list[i] < list[min_index] {
                min_index = i;
            }
            if list[i] > list[max_index] {
                max_index = i;
            }
        }

        println!("start: {start}, end: {end}, min: {min_index}, max: {max_index}");
        println!("start value: {:?}, end: {:?}, min: {:?}, max: {:?}",
                 list[start], list[end], list[min_index], list[max_index]);
        if min_index == 497 {
            println!("list slice: {list:?}");
        }

        if start != min_index {
            list.swap(start, min_index);
        }

        if end != max_index && !(start == max_index && end == min_index) {
            list.swap(end, max_index);
        }

        start += 1;
        if end > 1 {
            end -= 1;
        }
    }
    println!("END of sort");
}

#[cfg(test)]
mod tests {
    use super::{selection_sort, selection_sort_min_max, selection_sort_recursive};

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

    #[test]
    fn test_selection_sort_min_max() {
        let mut list = [0, 5, 3, 2, 2];
        selection_sort_min_max(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        selection_sort_min_max(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        selection_sort_min_max(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = [
            28894, 30024, 31175, 29332, 36942
        ];
        selection_sort_min_max(&mut list);
        assert_eq!(
            list,
            [
                28894, 29332, 30024, 31175, 36942
            ]
        );

        let mut list = [
            3713, 13249, 19224, 13962, -3804, -10101, 19000, 13820, 13993, 799, 14012
            , 3752, -12288,
        ];
        selection_sort_min_max(&mut list);
        assert_eq!(
            list,
            [
                -12288, -10101, -3804, 799, 3713, 3752, 13249, 13820, 13962, 13993, 14012, 19000, 19224,
            ]
        );
    }
}
