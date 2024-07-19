// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::env::args;

use sort::bubble_sort::{bubble_sort, recursive_bubble_sort};
use sort::bucket_sort::bucket_sort;
use sort::counting_sort::{counting_sort, counting_sort_with_map};
use sort::gnome_sort::gnome_sort;
use sort::heap_sort::heap_sort;
use sort::insertion_sort::{binary_insertion_sort, insertion_sort, recursive_insertion_sort};
use sort::merge_sort::{bottom_up_merge_sort, in_place_merge_sort, in_place_shell_merge_sort, insertion_merge_sort, merge_sort, shell_merge_sort, three_way_merge_sort};
use sort::quicksort::{head_quicksort, insertion_quicksort, iterative_quicksort, quicksort, two_pointer_quicksort};
use sort::selection_sort::{recursive_selection_sort, selection_sort, two_way_selection_sort};
use sort::shell_sort::shell_sort;
use sort::timsort::{shell_timsort, timsort};
use sort::util::{is_sorted, read_ints, show_brief};

fn sort_list(sort_method: &str, list: &mut Vec<i32>) {
    match sort_method {
        "binary-insertion-sort" => binary_insertion_sort(list),
        "bottom-up-merge-sort" => bottom_up_merge_sort(list),
        "bubble-sort" => bubble_sort(list),
        "bucket-sort" => bucket_sort(list),
        "counting-sort" => counting_sort(list),
        "counting-sort-with-map" => counting_sort_with_map(list),
        "gnome-sort" => gnome_sort(list),
        "head-quicksort" => head_quicksort(list),
        "heap-sort" => heap_sort(list),
        "in-place-merge-sort" => in_place_merge_sort(list),
        "in-place-shell-merge-sort" => in_place_shell_merge_sort(list),
        "insertion-merge-sort" => insertion_merge_sort(list),
        "insertion-quicksort" => insertion_quicksort(list),
        "insertion-sort" => insertion_sort(list),
        "iterative-quicksort" => iterative_quicksort(list),
        "merge-sort" => merge_sort(list),
        "quicksort" => quicksort(list),
        //"radix-sort" => radix_sort(list),
        "recursive-bubble-sort" => recursive_bubble_sort(list),
        "recursive-insertion-sort" => recursive_insertion_sort(list),
        "recursive-selection-sort" => recursive_selection_sort(list),
        "selection-sort" => selection_sort(list),
        "shell-merge-sort" => shell_merge_sort(list),
        "shell-sort" => shell_sort(list),
        "shell-timesort" => shell_timsort(list),
        "three-way-merge-sort" => three_way_merge_sort(list),
        "timsort" => timsort(list),
        "two-pointer-quicksort" => two_pointer_quicksort(list),
        "two-way-selection-sort" => two_way_selection_sort(list),
        _ => panic!("Unknown sort method!"),
    }
}

fn main() {
    let mut args = args();
    let _app_name = args.next();
    let sort_method = args.next().expect("Please specify sort name");
    println!("Sort method: {sort_method}");

    let mut list = read_ints();
    println!("LIST:");
    show_brief(&list);
    sort_list(&sort_method, &mut list);
    assert!(is_sorted(&list));
    println!("RESULT:");
    show_brief(&list);
}
