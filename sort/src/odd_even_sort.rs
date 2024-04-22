// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// Odd even sort is a variant of bubble sort.
///
/// [Odd even sort](https://en.wikipedia.org/wiki/Odd%E2%80%93even_sort)
pub fn odd_even_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let mut is_sorted = false;
    let len = arr.len();

    // Keep looping until all indices are traversed.
    while !is_sorted {
        is_sorted = true;

        // Iterating over all even indices
        for i in (0..(len - 1)).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                is_sorted = false;
            }
        }

        // Iterating over all odd indices
        for i in (1..(len - 1)).step_by(2) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                is_sorted = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::odd_even_sort;

    #[test]
    fn test_odd_even_sort() {
        let mut list = [0, 5, 3, 2, 2];
        odd_even_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        odd_even_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        odd_even_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        odd_even_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
