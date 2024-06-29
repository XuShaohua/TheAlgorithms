// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// 如果传入的数据是增序排好的, 那么只需要 N-1 次的比较, 以及 0 次的交换;
/// 平珓情况以及最坏情况下, 使用 N^2/2 次比较以及 N^2/2 次交换.
pub fn bubble_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();
    for i in 0..len {
        let mut swapped = false;
        // 以 (len - i - 1) 为分隔点, 左侧部分是无序的, 右侧部分是有序的
        for j in 0..(len - i - 1) {
            if list[j] > list[j + 1] {
                swapped = true;
                list.swap(j, j + 1);
            }
        }

        // 如果没有元素需要交换, 说明左侧部分也是有序的
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
