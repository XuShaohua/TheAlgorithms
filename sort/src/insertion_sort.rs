// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// 其思路是, 先将前 i 个元素调整为增序的, 随着 i 从 0 增大到 n, 整个序列就变得是增序了.
pub fn insertion_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();
    for i in 1..len {
        for j in (1..=i).rev() {
            if list[j - 1] > list[j] {
                list.swap(j - 1, j);
            } else {
                break;
            }
        }
    }
}

/// 递归风格的插入排序算法
pub fn insertion_sort_recursive<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();
    if len < 2 {
        return;
    }

    // 先将 list[..(len-1)] 中的元素排序.
    insertion_sort_recursive(&mut list[..len - 1]);

    // 然后将最后一个元素插入到合适的位置.
    for i in (1..len).rev() {
        if list[i - 1] > list[i] {
            list.swap(i - 1, i);
        } else {
            break;
        }
    }
}

#[allow(dead_code)]
fn binary_search_fake<T>(list: &[T], target: &T) -> usize
where
    T: PartialOrd,
{
    for i in (0..list.len()).rev() {
        if list[i] < *target {
            return i + 1;
        }
    }
    0
}

fn binary_search<T>(list: &[T], target: &T) -> usize
where
    T: PartialOrd,
{
    let mut left = 0;
    let mut right = list.len() - 1;
    while left < right {
        let middle = left + (right - left) / 2;
        // 找到了相等的元素, 就返回该位置的下一个位置
        if list[middle] == *target {
            return middle + 1;
        } else if list[middle] < *target {
            left = middle + 1;
        } else {
            right = middle;
        }
    }

    // 没有找到相等的元素, 就返回期望的位置.
    if list[list.len() - 1] < *target {
        return list.len();
    }
    if list[0] > *target {
        return 0;
    }
    left
}


/// 二分插入排序法 binary insertion sort
pub fn binary_insertion_sort<T>(list: &mut [T])
where
    T: PartialOrd + std::fmt::Debug,
{
    let len = list.len();
    if len < 2 {
        return;
    }

    for i in 1..len {
        let target_pos = binary_search(&list[..i], &list[i]);
        for j in (target_pos..i).rev() {
            list.swap(j, j + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{binary_insertion_sort, binary_search, binary_search_fake, insertion_sort, insertion_sort_recursive};

    #[test]
    fn test_insertion_sort() {
        let mut list = [0, 5, 3, 2, 2];
        insertion_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        insertion_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        insertion_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        insertion_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }

    #[test]
    fn test_insertion_sort_recursive() {
        let mut list = [0, 5, 3, 2, 2];
        insertion_sort_recursive(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        insertion_sort_recursive(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        insertion_sort_recursive(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        insertion_sort_recursive(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }

    #[test]
    fn test_binary_search_fake() {
        let list = [0, 5, 3, 2, 2];
        let pos = binary_search_fake(&list[0..1], &5);
        assert_eq!(pos, 1);

        let list = [0, 5, 3, 2, 2];
        let pos = binary_search_fake(&list[0..2], &3);
        assert_eq!(pos, 1);

        let list = [-5, -2, -45];
        let pos = binary_search_fake(&list[0..1], &(-2));
        assert_eq!(pos, 1);

        let list = [-5, -2, -45];
        let pos = binary_search_fake(&list[0..2], &(-45));
        assert_eq!(pos, 0);
    }

    #[test]
    fn test_binary_search() {
        let list = [0, 5, 3, 2, 2];
        let pos = binary_search(&list[0..1], &5);
        assert_eq!(pos, 1);

        let list = [0, 5, 3, 2, 2];
        let pos = binary_search(&list[0..2], &3);
        assert_eq!(pos, 1);

        let list = [-5, -2, -45];
        let pos = binary_search(&list[0..1], &(-2));
        assert_eq!(pos, 1);

        let list = [-5, -2, -45];
        let pos = binary_search(&list[0..2], &(-45));
        assert_eq!(pos, 0);
    }

    #[test]
    fn test_binary_insertion_sort() {
        let mut list = [0, 5, 3, 2, 2];
        binary_insertion_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        binary_insertion_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        binary_insertion_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        binary_insertion_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
