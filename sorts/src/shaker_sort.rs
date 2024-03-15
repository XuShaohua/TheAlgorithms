// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// Shaker sort, or Cocktail sort, is an extension to bubble sort, by operating
/// in two directions.
pub fn shaker_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();
    let mut start = 0;
    let mut end = len - 1;
    let mut swapped = true;

    while swapped {
        // Reset swap flag each loop.
        swapped = false;

        for i in start..end {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }

        end -= 1;

        swapped = false;

        // From right to left, doing the same comparison.
        for i in (start..end).rev() {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                swapped = true;
            }
        }
        start += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::shaker_sort;

    #[test]
    fn test_shaker_sort() {
        let mut list = [0, 5, 3, 2, 2];
        shaker_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        shaker_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        shaker_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        shaker_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
