// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// This sorting algorithm sorts an array using the principle of bubble sort,
/// but does it both from left to right and right to left.
///
/// Hence, it's called "Double sort"
pub fn double_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();

    for _ in 0..=(len - 1) / 2 {
        // No need to traverse to end of list
        for i in 0..(len - 1) {
            if list[i + 1] < list[i] {
                // Apply bubble sort from left to right (forward)
                list.swap(i + 1, i);
            }
            if list[len - 1 - i] < list[len - 2 - i] {
                // Apply bubble sort from right to left (backward)
                list.swap(len - 1 - i, len - 2 - i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::double_sort;

    #[test]
    fn test_double_sort() {
        let mut list = [0, 5, 3, 2, 2];
        double_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        double_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        double_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        double_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
