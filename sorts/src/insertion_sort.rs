// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// Place one element at a time, inserting each into its proper place.
///
/// The elements to the left of the current index are in sorted order during
/// the sort, but they are not in their final positions, as they may have to
/// be moved to make room for smaller elements later.
///
/// 其思路是, 先将前 i 个元素调整为增序的, 随着 i 从 0 增大到 n, 整个序列
/// 就变得是增序了.
///
/// 如果传数的数据是增序排好的, 那么只需要 N-1 次的比较, 以及 0 次的交换;
/// 如果传数的数据是降序排好的, 那么需要 N^2/2 次的比较, 以及 N^2/2 次的交换;
/// 如果是乱序的, 大概需要 N^2/4 次的比较, 以及 N^2/4 次的交换.
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

/// Insertion sort, no optimization.
///
/// Nonadaptive sort.
pub fn insertion_sort_vanilla<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();
    for i in 1..len {
        for j in (1..=i).rev() {
            if list[j - 1] > list[j] {
                list.swap(j - 1, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{insertion_sort, insertion_sort_vanilla};

    #[test]
    fn test_insertion_sort() {
        let mut list = [0, 5, 3, 2, 2];
        insertion_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        insertion_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998166, -996360, -995703, -995238, -995066, -994740, -992987, -983833, -987905,
            -980069, -977640,
        ];
        insertion_sort(&mut list);
        assert_eq!(
            list,
            [
                -998166, -996360, -995703, -995238, -995066, -994740, -992987, -987905, -983833,
                -980069, -977640,
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
    fn test_insertion_sort_vanilla() {
        let mut list = [0, 5, 3, 2, 2];
        insertion_sort_vanilla(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        insertion_sort_vanilla(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998166, -996360, -995703, -995238, -995066, -994740, -992987, -983833, -987905,
            -980069, -977640,
        ];
        insertion_sort_vanilla(&mut list);
        assert_eq!(
            list,
            [
                -998166, -996360, -995703, -995238, -995066, -994740, -992987, -987905, -983833,
                -980069, -977640,
            ]
        );
    }
}
