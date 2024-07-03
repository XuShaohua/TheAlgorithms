// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// Shell sort is a simple extension to insertion sort that allows exchanging
/// elements that far apart.
///
/// It produces partially sorted array (h-sorted array).
pub fn shell_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    const FACTOR: usize = 3;
    let len = arr.len();

    // 计算第一个 gap 的值, 大概是 len/3
    let mut h = 1;
    while h < len / FACTOR {
        h = FACTOR * h + 1;
    }

    while h >= 1 {
        // 使用插入排序, 将 `arr[0..h]` 排序好
        for i in h..len {
            let mut j = i;
            while j >= h && arr[j - h] > arr[j] {
                arr.swap(j - h, j);
                j -= h;
            }
        }

        h /= FACTOR;
    }
}

#[cfg(test)]
mod tests {
    use super::shell_sort;

    #[test]
    fn test_shell_sort() {
        let mut list = [0, 5, 3, 2, 2];
        shell_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        shell_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        shell_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        shell_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
