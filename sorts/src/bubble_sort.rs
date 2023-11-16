// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn bubble_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();
    for i in 0..len {
        for j in (i + 1..len).rev() {
            if list[j - 1] > list[j] {
                list.swap(j - 1, j);
            }
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
            -998166, -996360, -995703, -995238, -995066, -994740, -992987, -983833, -987905,
            -980069, -977640,
        ];
        bubble_sort(&mut list);
        assert_eq!(
            list,
            [
                -998166, -996360, -995703, -995238, -995066, -994740, -992987, -987905, -983833,
                -980069, -977640,
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
