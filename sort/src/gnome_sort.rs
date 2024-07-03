// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// Gnome sort is a variation of the insertion sort sorting algorithm
/// that does not use nested loops.
///
/// [Gnome sort](https://en.wikipedia.org/wiki/Gnome_sort)
pub fn gnome_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    let mut index = 0;
    while index < arr.len() {
        if index == 0 || arr[index] >= arr[index - 1] {
            index += 1;
        } else {
            arr.swap(index, index - 1);
            index -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::gnome_sort;

    #[test]
    fn test_gnome_sort() {
        let mut list = [0, 5, 3, 2, 2];
        gnome_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        gnome_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        gnome_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        gnome_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
