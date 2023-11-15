// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;

use crate::generics;
use crate::insertion_sort::insertion_sort_internal;

/// Devide-and-conquer recurrence, D(N) = NlgN.
///
/// Merge sort uses at most NlgN compares and 6NlgN array accesses to sort any
/// array of size N.
pub fn merge_sort<T>(vec: &mut Vec<T>)
where
    T: Copy + PartialOrd + fmt::Display,
{
    let mut aux = vec.clone();
    sort_array(vec, &mut aux, 0, vec.len() - 1);
}

fn merge_array<T>(vec: &mut Vec<T>, aux: &mut Vec<T>, low: usize, middle: usize, high: usize)
where
    T: Copy + PartialOrd + fmt::Display,
{
    if low < middle {
        assert!(generics::is_sorted(&vec[low..middle]));
    }
    if middle + 1 < high {
        assert!(generics::is_sorted(&vec[middle + 1..high]));
    }

    let mut i = low;
    let mut j = middle + 1;

    // Copy from original array to auxilary array.
    for k in low..=high {
        aux[k] = vec[k];
    }

    //         i          j
    // aux[] |........|........|
    //       low     mid      hi
    //
    // a[]   |........|........|
    //          k
    for k in low..=high {
        if i > middle {
            vec[k] = aux[j];
            j += 1;
        } else if j > high {
            vec[k] = aux[i];
            i += 1;
        } else if aux[j] < aux[i] {
            vec[k] = aux[j];
            j += 1;
        } else {
            vec[k] = aux[i];
            i += 1;
        }
    }
}

fn sort_array<T>(vec: &mut Vec<T>, aux: &mut Vec<T>, low: usize, high: usize)
where
    T: Copy + PartialOrd + fmt::Display,
{
    if low >= high {
        return;
    }
    //generics::show(&vec[low..high]);

    // Use insertion sort for small arrays.
    const CUTOFF: usize = 7;
    if high <= low + CUTOFF - 1 {
        insertion_sort_internal(vec, low, high);
        return;
    }

    let middle = low + (high - low) / 2;

    sort_array(vec, aux, low, middle);
    sort_array(vec, aux, middle + 1, high);

    // If is already sorted, returns immediately.
    if vec[middle] <= vec[middle + 1] {
        return;
    }
    merge_array(vec, aux, low, middle, high);
}
