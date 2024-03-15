// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// Selection sort works as follows:
/// - Find the smallest element in the array, and exchange it with the first
/// element in the array.
/// - Find the second smallest element in the array, and exchange it with the
/// element in the second position.
/// - Continue in this way until the entire array is sorted.
///
/// It takes about as long to run selection sort for a file that is already
/// in order, or for a file with all keys equal, as it does for a randomly
/// ordered file.
///
/// It is the method of choice for sorting files with huge items and small keys.
/// For such applications, the cost of moving the data dominates the cost of
/// making comparisons, and no algorithms can sort a file with substantially less
/// data movement than selection sort.
///
/// 即使输入数据已经是排好序的, 该算法依然需要 N^2 次的操作.
/// N^2 / 2 次比较以及 N 次交换.
///
/// 平均是 O(NlogN)
///
/// 这种算法, 每个元素最多只移动一次, 也就是说, 这个算法比较适合那种比较元素时的
/// 成本低, 但移动元素成本比较高的情况.
pub fn selection_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    if list.is_empty() {
        return;
    }
    let len = list.len();
    for i in 0..(len - 1) {
        let mut min_index = i;
        for j in (i + 1)..len {
            if list[j] < list[min_index] {
                min_index = j;
            }
        }
        if i != min_index {
            list.swap(i, min_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

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
}
