// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use crate::insertion_sort;

const CUTOFF: usize = 24;

/// Devide-and-conquer recurrence, `O(NlogN)`.
/// Its prime disadvantage is that it uses extra space.
///
pub fn merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    topdown_merge_sort(arr);
}

/// Top-down merge sort uses at most `NlgN` compares and `6NlgN` array accesses to
/// sort any array of size N.
pub fn topdown_merge_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    sort(arr, 0, arr.len() - 1);
}

/// Sort `arr[low..=high]`
fn sort<T>(arr: &mut [T], low: usize, high: usize)
where
    T: PartialOrd + Copy,
{
    println!("low: {low}, high: {high}");
    if low >= high {
        return;
    }
    // Use insertion sort for small subarrays.
    if high < low + CUTOFF {
        insertion_sort(&mut arr[low..=high]);
        return;
    }

    let middle = low + (high - low) / 2;

    // Sort left part
    sort(arr, low, middle);
    // Sort right part
    sort(arr, middle + 1, high);

    // If is already sorted, returns immediately.
    if arr[middle] <= arr[middle + 1] {
        return;
    }
    merge(arr, low, middle, high);
}

/// Merge arr[low..=middle] and arr[middle+1..=high]
///
/// It's not inplacement merge.
#[allow(clippy::needless_range_loop)]
fn merge<T>(arr: &mut [T], low: usize, middle: usize, high: usize)
where
    T: PartialOrd + Copy,
{
    println!("merge: low: {low}, middle: {middle}, high: {high}");
    let aux = arr[0..=high].to_vec();

    // Merge back to arr.
    let mut i = low;
    let mut j = middle + 1;
    for k in low..=high {
        if i > middle {
            arr[k] = aux[j];
            j += 1;
        } else if j > high {
            arr[k] = aux[i];
            i += 1;
        } else if aux[j] < aux[i] {
            arr[k] = aux[j];
            j += 1;
        } else {
            arr[k] = aux[i];
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn test_merge_sort() {
        let mut list = [0, 5, 3, 2, 2];
        merge_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        merge_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998166, -996360, -995703, -995238, -995066, -994740, -992987, -983833, -987905,
            -980069, -977640,
        ];
        merge_sort(&mut list);
        assert_eq!(
            list,
            [
                -998166, -996360, -995703, -995238, -995066, -994740, -992987, -987905, -983833,
                -980069, -977640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        merge_sort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}
