// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// Like insertion sort, but moves through the right part of the array.
///
/// On the first pass, the minimum element is exchanged with each of elements
/// to its left, eventually putting it into the left end of the array.
/// On the second pass, the second smallest element will be put into position.
/// And so forth.
///
/// Inadaptive sort.
///
/// 如果传入的数据是增序排好的, 那么只需要 N-1 次的比较, 以及 0 次的交换;
/// 平珓情况以及最坏情况下, 使用 N^2/2 次比较以及 N^2/2 次交换.
pub fn bubble_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();
    for i in 0..len {
        let mut swapped = false;
        for j in (i + 1..len).rev() {
            if list[j - 1] > list[j] {
                swapped = true;
                list.swap(j - 1, j);
            }
        }
        // Stop iteration if the collection is sorted.
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

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
}
