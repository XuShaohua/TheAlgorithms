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
